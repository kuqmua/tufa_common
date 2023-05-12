#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "header_pair")]
struct HeaderPairRecord {
    name: String,
    value: Vec<u8>,
}

impl sqlx::postgres::PgHasArrayType for HeaderPairRecord {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_header_pair")
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetSavedResponseErrorNamed<'a> {
    PostgresSelect {
        #[eo_display]
        postgres_select: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    TryFromIntError {
        #[eo_display]
        try_from_int_error: std::num::TryFromIntError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    InvalidStatusCode {
        #[eo_display]
        invalid_status_code: http::status::InvalidStatusCode,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn get_saved_response<'a>(
    pool: &sqlx::PgPool,
    idempotency_key: &super::IdempotencyKey,
    user_id: uuid::Uuid,
) -> Result<Option<actix_web::HttpResponse>, GetSavedResponseErrorNamed<'a>> {
    //todo - sqlx::query! is a macro to check db on compile time. DATABASE_URL must be set in env variables. its not for lib. change it later
    let saved_response = match sqlx::query!(
        r#"
        SELECT 
            response_status_code as "response_status_code!", 
            response_headers as "response_headers!: Vec<HeaderPairRecord>",
            response_body as "response_body!"
        FROM idempotency
        WHERE 
          user_id = $1 AND
          idempotency_key = $2
        "#,
        user_id,
        idempotency_key.as_ref()
    )
    .fetch_optional(pool)
    .await {
        Err(e) => {
            return Err(GetSavedResponseErrorNamed::PostgresSelect {
                postgres_select: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        },
        Ok(option_record) => option_record,
    };
    if let Some(r) = saved_response {
        match r.response_status_code.try_into() {
            Err(e) => Err(GetSavedResponseErrorNamed::TryFromIntError {
                try_from_int_error: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
            Ok(status_code_as_u16) => match actix_web::http::StatusCode::from_u16(status_code_as_u16) {
                Err(e) => Err(GetSavedResponseErrorNamed::InvalidStatusCode {
                    invalid_status_code: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
                Ok(status_code) => {
                    let mut response = actix_web::HttpResponse::build(status_code);
                    for HeaderPairRecord { name, value } in r.response_headers {
                        response.append_header((name, value));
                    }
                    Ok(Some(response.body(r.response_body)))
                },
            },
        }
    } else {
        Ok(None)
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum SaveResponseErrorNamed<'a> {
    BodyToBytes {
        #[eo_display]
        body_to_bytes: actix_web::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgtesUpdate {
        #[eo_display]
        postgres_update: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgtesTransactionCommit {
        #[eo_display]
        postgres_transaction_commit: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn save_response<'a>(
    mut transaction: sqlx::Transaction<'static, sqlx::Postgres>,
    idempotency_key: &super::IdempotencyKey,
    user_id: uuid::Uuid,
    http_response: actix_web::HttpResponse,
) -> Result<actix_web::HttpResponse, SaveResponseErrorNamed<'a>> {
    let (response_head, body) = http_response.into_parts();
    // `MessageBody::Error` is not `Send` + `Sync`,
    let body = match actix_web::body::to_bytes(body).await {
        Err(e) => {
            return Err(SaveResponseErrorNamed::BodyToBytes {
                body_to_bytes: e.into(),
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        },
        Ok(bytes) => bytes,
    };
    let status_code = response_head.status().as_u16() as i16;
    let headers = {
        let mut h = Vec::with_capacity(response_head.headers().len());
        for (name, value) in response_head.headers().iter() {
            let name = name.as_str().to_owned();
            let value = value.as_bytes().to_owned();
            h.push(HeaderPairRecord { name, value });
        }
        h
    };
    if let Err(e) = sqlx::query_unchecked!(
        r#"
        UPDATE idempotency
        SET 
            response_status_code = $3, 
            response_headers = $4,
            response_body = $5
        WHERE
            user_id = $1 AND
            idempotency_key = $2
        "#,
        user_id,
        idempotency_key.as_ref(),
        status_code,
        headers,
        body.as_ref()
    )
    .execute(&mut transaction)
    .await {
        return Err(SaveResponseErrorNamed::PostgtesUpdate {
            postgres_update: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    };
    if let Err(e) = transaction.commit().await {
        return Err(SaveResponseErrorNamed::PostgtesTransactionCommit {
            postgres_transaction_commit: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    // We need `.map_into_boxed_body` to go from
    // `actix_web::HttpResponse<Bytes>` to `actix_web::HttpResponse<BoxBody>`
    let http_response = response_head.set_body(body).map_into_boxed_body();
    Ok(http_response)
}

#[allow(large_enum_variant)]
pub enum NextAction {
    ReturnSavedResponse(actix_web::HttpResponse),
    StartProcessing(sqlx::Transaction<'static, sqlx::Postgres>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryProcessingErrorNamed<'a> {
    PostgresPoolBegin {
        #[eo_display]
        pool_begin_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresInsert {
        #[eo_display]
        insert: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    GetSavedResponse {
        #[eo_error_occurence]
        get_saved_response: GetSavedResponseErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    SavedResponseIsNone {
        #[eo_display_with_serialize_deserialize]
        message: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    }
}

pub async fn try_processing<'a>(
    pool: &sqlx::PgPool,
    idempotency_key: &super::IdempotencyKey,
    user_id: uuid::Uuid,
) -> Result<NextAction, TryProcessingErrorNamed<'a>> {
    let mut transaction = match pool.begin().await {
        Err(e) => {
            return Err(TryProcessingErrorNamed::PostgresPoolBegin {
                pool_begin_error: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        },
        Ok(transaction) => transaction,
    };
    let n_inserted_rows = match sqlx::query!(
        r#"
        INSERT INTO idempotency (
            user_id, 
            idempotency_key,
            created_at
        ) 
        VALUES ($1, $2, now()) 
        ON CONFLICT DO NOTHING
        "#,
        user_id,
        idempotency_key.as_ref()
    )
    .execute(&mut transaction)
    .await {
        Err(e) => {
            return Err(TryProcessingErrorNamed::PostgresInsert {
                insert: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        },
        Ok(pg_query_result) => pg_query_result.rows_affected(),
    };
    if n_inserted_rows > 0 {
        Ok(NextAction::StartProcessing(transaction))
    } else {
        match get_saved_response(pool, idempotency_key, user_id).await {
            Err(e) => {
                return Err(TryProcessingErrorNamed::GetSavedResponse {
                    get_saved_response: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            },
            Ok(option_http_response) => match option_http_response {
                None => Err(TryProcessingErrorNamed::SavedResponseIsNone {
                    message: "We expected a saved response, we didn't find it",
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
                Some(saved_response) => Ok(NextAction::ReturnSavedResponse(saved_response)),
            },
        }
    }
}
