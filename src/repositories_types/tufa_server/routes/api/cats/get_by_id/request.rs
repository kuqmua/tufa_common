//
impl<'a> From<crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed<'a>> for actix_web::HttpResponse {
    fn from(val: crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed<'a>) -> Self {
        let mut actix_web_http_response: actix_web::HttpResponseBuilder = (&val).into();
        actix_web_http_response.json(actix_web::web::Json(
            val.into_serialize_deserialize_version(),
        ))
    }
}

impl<'a> From<&crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed<'a>> for actix_web::HttpResponseBuilder {
    fn from(val: &crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed<'a>) -> Self {
        match &val {
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Bigserial { bigserial: _, code_occurence: _ } => actix_web::HttpResponse::BadRequest(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Configuration {
                configuration_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Database {
                box_dyn_database_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Io {
                io_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Tls {
                box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Protocol {
                protocol: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::NotFound(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Decode {
                decode_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => actix_web::HttpResponse::RequestTimeout(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Migrate {
                migrate: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
        }
    }
}

#[derive(Debug)]
pub enum GetByIdExpectedStatusCode {
    Ok,
    BadRequest,
    InternalServerError,
}

impl std::convert::TryFrom<http::StatusCode> for GetByIdExpectedStatusCode {
    type Error = http::StatusCode;
    fn try_from(value: http::StatusCode) -> Result<Self, Self::Error> {
        if http::StatusCode::OK == value {
            Ok(Self::Ok)
        } else if http::StatusCode::BAD_REQUEST == value {
            Ok(Self::BadRequest)
        } else if http::StatusCode::INTERNAL_SERVER_ERROR == value {
            Ok(Self::InternalServerError)
        } else {
            Err(value)
        }
    }
}

impl GetByIdExpectedStatusCode {
    pub async fn try_into_expected_type<'a>(
        &self,
        response: reqwest::Response,
    ) -> Result<
        crate::repositories_types::tufa_server::routes::api::cats::Cat,
        Result<GetByIdExpectedErrorStatusCodesErrorUnnamed, GetByIdExpectedStatusCodesJsonConversionErrorNamed<'a>>,
    > {
        match self {
            GetByIdExpectedStatusCode::Ok => match response.json::<crate::repositories_types::tufa_server::routes::api::cats::Cat>().await {
                Ok(value) => Ok(value),
                Err(e) => Err(Err(GetByIdExpectedStatusCodesJsonConversionErrorNamed::Ok {
                    ok: e,
                    code_occurence: crate::code_occurence_tufa_common!()
                })),
            },
            GetByIdExpectedStatusCode::BadRequest => match response.json::<GetByIdStatusCodeBadRequestExpectedBodyTypeWithSerializeDeserialize>().await {
                Ok(bad_req) => Err(
                    Ok(
                        GetByIdExpectedErrorStatusCodesErrorUnnamed::BadRequest(bad_req)
                    )
                ),
                Err(e) => Err(
                    Err(
                        GetByIdExpectedStatusCodesJsonConversionErrorNamed::BadRequest { 
                            bad_request: e, 
                            code_occurence: crate::code_occurence_tufa_common!() 
                        }
                    )
                ),
            },
            GetByIdExpectedStatusCode::InternalServerError => match response.json::<GetByIdStatusCodeInternalServerErrorExpectedBodyTypeWithSerializeDeserialize>().await {
                Ok(internal_server_error) => Err(Ok(GetByIdExpectedErrorStatusCodesErrorUnnamed::InternalServerError(internal_server_error))),
                Err(e) => Err(Err(GetByIdExpectedStatusCodesJsonConversionErrorNamed::InternalServerError { 
                    internal_server_error: e, 
                    code_occurence: crate::code_occurence_tufa_common!() 
                })),
            },
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetByIdStatusCodeBadRequestExpectedBodyType<'a> {
    //todo struct concatination
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: &'a str,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //
    Bigserial {
        #[eo_error_occurence]
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetByIdStatusCodeInternalServerErrorExpectedBodyType<'a> {
    PostgresSelect {
        #[eo_display]
        postgres_select: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GetByIdExpectedErrorStatusCodesErrorUnnamed {
    BadRequest(GetByIdStatusCodeBadRequestExpectedBodyTypeWithSerializeDeserialize),
    InternalServerError(
        GetByIdStatusCodeInternalServerErrorExpectedBodyTypeWithSerializeDeserialize,
    ),
}

impl std::fmt::Display for GetByIdExpectedErrorStatusCodesErrorUnnamed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GetByIdExpectedErrorStatusCodesErrorUnnamed::BadRequest(e) => write!(f, "{e}"),
            GetByIdExpectedErrorStatusCodesErrorUnnamed::InternalServerError(e) => write!(f, "{e}"),
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetByIdExpectedStatusCodesJsonConversionErrorNamed<'a> {
    Ok {
        #[eo_display]
        ok: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    BadRequest {
        #[eo_display]
        bad_request: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    InternalServerError {
        #[eo_display]
        internal_server_error: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
//

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGetByIdErrorNamed<'a> {
    BelowZero {
        #[eo_display_with_serialize_deserialize]
        below_zero: i64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ExpectedServerError {
        #[eo_display_with_serialize_deserialize]
        expected_server_error: GetByIdExpectedErrorStatusCodesErrorUnnamed,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ExpectedStatusCodeBodyConversion {
        #[eo_display]
        expected_status_code: http::StatusCode,
        #[eo_error_occurence]
        conversion_error: crate::repositories_types::tufa_server::routes::api::cats::get_by_id::request::GetByIdExpectedStatusCodesJsonConversionErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    UnexpectedStatusCode {
        //todo add headers? body? as Option<String>
        #[eo_display]
        unexpected_status_code: http::StatusCode,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn try_get_by_id<'a>(
    server_location: std::string::String,
    path_parameters: crate::repositories_types::tufa_server::routes::api::cats::get_by_id::GetByIdPathParameters,
) -> Result<crate::repositories_types::tufa_server::routes::api::cats::Cat, TryGetByIdErrorNamed<'a>>
{
    // todo maybe path_parameters already must be non zero?
    if let true = path_parameters.id.is_negative() {
        return Err(TryGetByIdErrorNamed::BelowZero {
            below_zero: path_parameters.id,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    let url = format!(
        "{server_location}/api/{}/{}",
        crate::repositories_types::tufa_server::routes::api::cats::CATS,
        path_parameters.id
    );
    // println!("{url}");
    match reqwest::Client::new()
        .get(&url)
        .header(
            crate::common::git::project_git_info::PROJECT_COMMIT,
            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                .project_commit,
        )
        .send()
        .await
    {
        Ok(response) => {
            let response_status = response.status();
            match GetByIdExpectedStatusCode::try_from(response.status()) {
                Ok(expected_status_code) => {
                    match expected_status_code.try_into_expected_type(response).await {
                        Ok(value) => Ok(value),
                        Err(error_result) => match error_result {
                            Ok(expected_server_error) => {
                                Err(TryGetByIdErrorNamed::ExpectedServerError {
                                    expected_server_error,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                })
                            }
                            Err(conversion_error) => {
                                let e = TryGetByIdErrorNamed::ExpectedStatusCodeBodyConversion {
                                    expected_status_code: response_status,
                                    conversion_error,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                                Err(e)
                            }
                        },
                    }
                }
                Err(unexpected_status_code) => Err(TryGetByIdErrorNamed::UnexpectedStatusCode {
                    unexpected_status_code,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            }
        }
        Err(e) => Err(TryGetByIdErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
