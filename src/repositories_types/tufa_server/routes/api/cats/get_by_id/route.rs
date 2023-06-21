#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence, from_sqlx_postgres_error::FromSqlxPostgresError)]
pub enum GetByIdErrorNamed<'a> {
    Bigserial {
        #[eo_error_occurence]
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[non_exhaustive] case
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    }
}

impl<'a> From<&GetByIdErrorNamed<'a>> for actix_web::HttpResponseBuilder {
    fn from(val: &GetByIdErrorNamed<'a>) -> Self {
        match &val {
            GetByIdErrorNamed::Bigserial { bigserial: _, code_occurence: _ } => actix_web::HttpResponse::BadRequest(),
            GetByIdErrorNamed::Configuration {
                configuration_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetByIdErrorNamed::Database {
                box_dyn_database_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetByIdErrorNamed::Io {
                io_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetByIdErrorNamed::Tls {
                box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetByIdErrorNamed::Protocol {
                protocol: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetByIdErrorNamed::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::NotFound(),
            GetByIdErrorNamed::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            GetByIdErrorNamed::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetByIdErrorNamed::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            GetByIdErrorNamed::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetByIdErrorNamed::Decode {
                decode_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetByIdErrorNamed::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => actix_web::HttpResponse::RequestTimeout(),
            GetByIdErrorNamed::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetByIdErrorNamed::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetByIdErrorNamed::Migrate {
                migrate: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetByIdErrorNamed::UnexpectedCase {
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
