#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence, from_enum::FromEnum)]
#[from_enum::from_enum_paths(
    crate::repositories_types::tufa_server::routes::api::cats::get::route::GetHttpResponse,
    crate::repositories_types::tufa_server::routes::api::cats::get::route::GetErrorNamedWithSerializeDeserialize,
    crate::repositories_types::tufa_server::routes::api::cats::get::request::GetHttpResponseVariants,
    crate::repositories_types::tufa_server::routes::api::cats::get::request::TryGetHttpResponseVariantsWithSerializeDeserialize,

    crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdHttpResponse,
    crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamedWithSerializeDeserialize,
    crate::repositories_types::tufa_server::routes::api::cats::get_by_id::request::GetByIdHttpResponseVariants,
    crate::repositories_types::tufa_server::routes::api::cats::get_by_id::request::TryGetByIdErrorHttpResponseWithSerializeDeserialize
)]
pub enum SqlxPostgresErrorErrorNamed<'a> {
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
    },
}

// pub fn sqlx_error_to_status_code(sqlx_error: &sqlx::Error) -> http::StatusCode {
//     match sqlx_error {
//         sqlx::Error::Configuration(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
//         sqlx::Error::Database(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
//         sqlx::Error::Io(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
//         sqlx::Error::Tls(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
//         sqlx::Error::Protocol(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
//         sqlx::Error::RowNotFound => http::StatusCode::NOT_FOUND,
//         sqlx::Error::TypeNotFound { type_name: _ } => http::StatusCode::BAD_REQUEST,
//         sqlx::Error::ColumnIndexOutOfBounds { index: _, len: _ } => {
//             http::StatusCode::INTERNAL_SERVER_ERROR
//         }
//         sqlx::Error::ColumnNotFound(_) => http::StatusCode::BAD_REQUEST,
//         sqlx::Error::ColumnDecode {
//             index: _,
//             source: _,
//         } => http::StatusCode::INTERNAL_SERVER_ERROR,
//         sqlx::Error::Decode(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
//         sqlx::Error::PoolTimedOut => http::StatusCode::REQUEST_TIMEOUT,
//         sqlx::Error::PoolClosed => http::StatusCode::INTERNAL_SERVER_ERROR,
//         sqlx::Error::WorkerCrashed => http::StatusCode::INTERNAL_SERVER_ERROR,
//         sqlx::Error::Migrate(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
//         _ => http::StatusCode::INTERNAL_SERVER_ERROR,
//     }
// }

// pub fn sqlx_error_to_actix_web_http_response_builder(
//     sqlx_error: &sqlx::Error,
// ) -> actix_web::HttpResponseBuilder {
//     match sqlx_error {
//         sqlx::Error::Configuration(_) => actix_web::HttpResponse::InternalServerError(),
//         sqlx::Error::Database(_) => actix_web::HttpResponse::InternalServerError(),
//         sqlx::Error::Io(_) => actix_web::HttpResponse::InternalServerError(),
//         sqlx::Error::Tls(_) => actix_web::HttpResponse::InternalServerError(),
//         sqlx::Error::Protocol(_) => actix_web::HttpResponse::InternalServerError(),
//         sqlx::Error::RowNotFound => actix_web::HttpResponse::NotFound(),
//         sqlx::Error::TypeNotFound { type_name: _ } => actix_web::HttpResponse::BadRequest(),
//         sqlx::Error::ColumnIndexOutOfBounds { index: _, len: _ } => {
//             actix_web::HttpResponse::InternalServerError()
//         }
//         sqlx::Error::ColumnNotFound(_) => actix_web::HttpResponse::BadRequest(),
//         sqlx::Error::ColumnDecode {
//             index: _,
//             source: _,
//         } => actix_web::HttpResponse::InternalServerError(),
//         sqlx::Error::Decode(_) => actix_web::HttpResponse::InternalServerError(),
//         sqlx::Error::PoolTimedOut => actix_web::HttpResponse::RequestTimeout(),
//         sqlx::Error::PoolClosed => actix_web::HttpResponse::InternalServerError(),
//         sqlx::Error::WorkerCrashed => actix_web::HttpResponse::InternalServerError(),
//         sqlx::Error::Migrate(_) => actix_web::HttpResponse::InternalServerError(),
//         _ => actix_web::HttpResponse::InternalServerError(),
//     }
// }

impl<'a> std::convert::From<sqlx::Error> for SqlxPostgresErrorErrorNamed<'a> {
    fn from(e: sqlx::Error) -> SqlxPostgresErrorErrorNamed<'a> {
        // todo https://github.com/cschaible/actix-web-security-samples/blob/46bb7aa62ada7cb176d8765e2f60b497392b1840/oauth-resource-server/backend/src/error/mod.rs#L46
        // todo https://www.postgresql.org/docs/current/errcodes-appendix.html
        match e {
            sqlx::Error::Configuration(box_dyn_error) => {
                SqlxPostgresErrorErrorNamed::Configuration {
                    configuration_box_dyn_error: box_dyn_error.to_string(),
                    code_occurence: crate::code_occurence_tufa_common!(),
                }
            }
            sqlx::Error::Database(box_dyn_database_error) => {
                SqlxPostgresErrorErrorNamed::Database {
                    box_dyn_database_error: box_dyn_database_error.message().to_string(),
                    code_occurence: crate::code_occurence_tufa_common!(),
                }
            }
            sqlx::Error::Io(io_error) => SqlxPostgresErrorErrorNamed::Io {
                io_error,
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            sqlx::Error::Tls(box_dyn_error) => SqlxPostgresErrorErrorNamed::Tls {
                box_dyn_error: box_dyn_error.to_string(),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            sqlx::Error::Protocol(string) => SqlxPostgresErrorErrorNamed::Protocol {
                protocol: string,
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            sqlx::Error::RowNotFound => SqlxPostgresErrorErrorNamed::RowNotFound {
                row_not_found: std::string::String::from("row_not_found"),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            sqlx::Error::TypeNotFound { type_name } => SqlxPostgresErrorErrorNamed::TypeNotFound {
                type_not_found: type_name,
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
                SqlxPostgresErrorErrorNamed::ColumnIndexOutOfBounds {
                    column_index_out_of_bounds: index,
                    len,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }
            }
            sqlx::Error::ColumnNotFound(column_not_found) => {
                SqlxPostgresErrorErrorNamed::ColumnNotFound {
                    column_not_found,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }
            }
            sqlx::Error::ColumnDecode { index, source } => {
                SqlxPostgresErrorErrorNamed::ColumnDecode {
                    column_decode_index: index,
                    source_handle: source.to_string(),
                    code_occurence: crate::code_occurence_tufa_common!(),
                }
            }
            sqlx::Error::Decode(decode_box_dyn_error) => SqlxPostgresErrorErrorNamed::Decode {
                decode_box_dyn_error: decode_box_dyn_error.to_string(),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            sqlx::Error::PoolTimedOut => SqlxPostgresErrorErrorNamed::PoolTimedOut {
                pool_timed_out: std::string::String::from("pool timed out"),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            sqlx::Error::PoolClosed => SqlxPostgresErrorErrorNamed::PoolClosed {
                pool_closed: std::string::String::from("pool closed"),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            sqlx::Error::WorkerCrashed => SqlxPostgresErrorErrorNamed::WorkerCrashed {
                worker_crashed: std::string::String::from("worker crashed"),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            sqlx::Error::Migrate(migrate_error) => SqlxPostgresErrorErrorNamed::Migrate {
                migrate: *migrate_error,
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            _ => SqlxPostgresErrorErrorNamed::UnexpectedCase {
                unexpected_case: std::string::String::from("unexpected_case"),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
        }
    }
}

// impl<'a> From<SqlxPostgresErrorErrorNamed<'a>>
//     for actix_web::web::Json<SqlxPostgresErrorErrorNamedWithSerializeDeserialize>
// {
//     fn from(val: SqlxPostgresErrorErrorNamed<'a>) -> Self {
//         actix_web::web::Json(val.into_serialize_deserialize_version())
//     }
// }

// impl<'a> From<SqlxPostgresErrorErrorNamed<'a>> for actix_web::Error {
//     fn from(val: SqlxPostgresErrorErrorNamed<'a>) -> Self {
//         match &val {
//             SqlxPostgresErrorErrorNamed::Configuration {
//                 configuration_box_dyn_error: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorInternalServerError(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::Database {
//                 box_dyn_database_error: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorInternalServerError(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::Io {
//                 io_error: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorInternalServerError(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::Tls {
//                 box_dyn_error: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorInternalServerError(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::Protocol {
//                 protocol: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorInternalServerError(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::RowNotFound {
//                 row_not_found: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorNotFound(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::TypeNotFound {
//                 type_not_found: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorBadRequest(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds: _,
//                 len: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorInternalServerError(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::ColumnNotFound {
//                 column_not_found: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorBadRequest(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::ColumnDecode {
//                 column_decode_index: _,
//                 source_handle: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorInternalServerError(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::Decode {
//                 decode_box_dyn_error: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorInternalServerError(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::PoolTimedOut {
//                 pool_timed_out: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorRequestTimeout(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::PoolClosed {
//                 pool_closed: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorInternalServerError(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::WorkerCrashed {
//                 worker_crashed: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorInternalServerError(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::Migrate {
//                 migrate: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorInternalServerError(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//             SqlxPostgresErrorErrorNamed::UnexpectedCase {
//                 unexpected_case: _,
//                 code_occurence: _,
//             } => actix_web::error::ErrorInternalServerError(actix_web::web::Json(
//                 val.into_serialize_deserialize_version(),
//             )),
//         }
//     }
// }

// impl<'a> From<SqlxPostgresErrorErrorNamed<'a>> for actix_web::HttpResponse {
//     fn from(val: SqlxPostgresErrorErrorNamed<'a>) -> Self {
//         actix_web::HttpResponseBuilder::from(&val).json(actix_web::web::Json(
//             val.into_serialize_deserialize_version(),
//         ))
//     }
// }

// impl<'a> From<&SqlxPostgresErrorErrorNamed<'a>> for http::StatusCode {
//     fn from(val: &SqlxPostgresErrorErrorNamed<'a>) -> Self {
//         match val {
//             SqlxPostgresErrorErrorNamed::Configuration {
//                 configuration_box_dyn_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             SqlxPostgresErrorErrorNamed::Database {
//                 box_dyn_database_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             SqlxPostgresErrorErrorNamed::Io {
//                 io_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             SqlxPostgresErrorErrorNamed::Tls {
//                 box_dyn_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             SqlxPostgresErrorErrorNamed::Protocol {
//                 protocol: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             SqlxPostgresErrorErrorNamed::RowNotFound {
//                 row_not_found: _,
//                 code_occurence: _,
//             } => http::StatusCode::NOT_FOUND,
//             SqlxPostgresErrorErrorNamed::TypeNotFound {
//                 type_not_found: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             SqlxPostgresErrorErrorNamed::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds: _,
//                 len: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             SqlxPostgresErrorErrorNamed::ColumnNotFound {
//                 column_not_found: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             SqlxPostgresErrorErrorNamed::ColumnDecode {
//                 column_decode_index: _,
//                 source_handle: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             SqlxPostgresErrorErrorNamed::Decode {
//                 decode_box_dyn_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             SqlxPostgresErrorErrorNamed::PoolTimedOut {
//                 pool_timed_out: _,
//                 code_occurence: _,
//             } => http::StatusCode::REQUEST_TIMEOUT,
//             SqlxPostgresErrorErrorNamed::PoolClosed {
//                 pool_closed: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             SqlxPostgresErrorErrorNamed::WorkerCrashed {
//                 worker_crashed: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             SqlxPostgresErrorErrorNamed::Migrate {
//                 migrate: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             SqlxPostgresErrorErrorNamed::UnexpectedCase {
//                 unexpected_case: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }
// }

// impl<'a> From<&SqlxPostgresErrorErrorNamed<'a>> for actix_web::HttpResponseBuilder {
//     fn from(val: &SqlxPostgresErrorErrorNamed<'a>) -> Self {
//         match val {
//             SqlxPostgresErrorErrorNamed::Configuration {
//                 configuration_box_dyn_error: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             SqlxPostgresErrorErrorNamed::Database {
//                 box_dyn_database_error: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             SqlxPostgresErrorErrorNamed::Io {
//                 io_error: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             SqlxPostgresErrorErrorNamed::Tls {
//                 box_dyn_error: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             SqlxPostgresErrorErrorNamed::Protocol {
//                 protocol: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             SqlxPostgresErrorErrorNamed::RowNotFound {
//                 row_not_found: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::NotFound(),
//             SqlxPostgresErrorErrorNamed::TypeNotFound {
//                 type_not_found: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::BadRequest(),
//             SqlxPostgresErrorErrorNamed::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds: _,
//                 len: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             SqlxPostgresErrorErrorNamed::ColumnNotFound {
//                 column_not_found: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::BadRequest(),
//             SqlxPostgresErrorErrorNamed::ColumnDecode {
//                 column_decode_index: _,
//                 source_handle: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             SqlxPostgresErrorErrorNamed::Decode {
//                 decode_box_dyn_error: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             SqlxPostgresErrorErrorNamed::PoolTimedOut {
//                 pool_timed_out: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::RequestTimeout(),
//             SqlxPostgresErrorErrorNamed::PoolClosed {
//                 pool_closed: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             SqlxPostgresErrorErrorNamed::WorkerCrashed {
//                 worker_crashed: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             SqlxPostgresErrorErrorNamed::Migrate {
//                 migrate: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             SqlxPostgresErrorErrorNamed::UnexpectedCase {
//                 unexpected_case: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//         }
//     }
// }
