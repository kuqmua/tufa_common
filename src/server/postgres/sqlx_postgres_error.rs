#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::tufa_server::routes::api::cats::TryReadOne,
    crate::repositories_types::tufa_server::routes::api::cats::TryCreateOne,
    crate::repositories_types::tufa_server::routes::api::cats::TryCreateMany,
    crate::repositories_types::tufa_server::routes::api::cats::TryReadManyWithBody
)]
pub enum SqlxPostgresErrorErrorNamed {
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

impl std::convert::From<sqlx::Error> for SqlxPostgresErrorErrorNamed {
    fn from(e: sqlx::Error) -> SqlxPostgresErrorErrorNamed {
        // todo https://github.com/cschaible/actix-web-security-samples/blob/46bb7aa62ada7cb176d8765e2f60b497392b1840/oauth-resource-server/backend/src/error/mod.rs#L46
        // todo https://www.postgresql.org/docs/current/errcodes-appendix.html
        match e {
            sqlx::Error::Configuration(box_dyn_error) => {
                SqlxPostgresErrorErrorNamed::Configuration {
                    configuration: box_dyn_error.to_string(),
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
// /////////
// impl std::convert::From<SqlxPostgresErrorErrorNamed>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryReadManyResponseVariants
// {
//     fn from(val: SqlxPostgresErrorErrorNamed) -> Self {
//         match val.into_serialize_deserialize_version() {
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Configuration {
//                 configuration,
//                 code_occurence,
//             } => Self::Configuration {
//                 configuration,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             } => Self::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Io {
//                 io_error,
//                 code_occurence,
//             } => Self::Io {
//                 io_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             } => Self::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Protocol {
//                 protocol,
//                 code_occurence,
//             } => Self::Protocol {
//                 protocol,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             } => Self::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             } => Self::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             } => Self::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             } => Self::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             } => Self::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             } => Self::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             } => Self::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Migrate {
//                 migrate,
//                 code_occurence,
//             } => Self::Migrate {
//                 migrate,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// impl std::convert::From<SqlxPostgresErrorErrorNamed>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryReadOneResponseVariants
// {
//     fn from(val: SqlxPostgresErrorErrorNamed) -> Self {
//         match val.into_serialize_deserialize_version() {
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Configuration {
//                 configuration,
//                 code_occurence,
//             } => Self::Configuration {
//                 configuration,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             } => Self::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Io {
//                 io_error,
//                 code_occurence,
//             } => Self::Io {
//                 io_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             } => Self::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Protocol {
//                 protocol,
//                 code_occurence,
//             } => Self::Protocol {
//                 protocol,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             } => Self::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             } => Self::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             } => Self::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             } => Self::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             } => Self::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             } => Self::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             } => Self::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Migrate {
//                 migrate,
//                 code_occurence,
//             } => Self::Migrate {
//                 migrate,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryReadManyWithSerializeDeserialize
// {
//     fn from(val : SqlxPostgresErrorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             Configuration { configuration, code_occurence } =>
//             Self :: Configuration
//             { configuration, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence } => Self :: RowNotFound
//             { row_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             TypeNotFound { type_not_found, code_occurence } => Self ::
//             TypeNotFound { type_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnNotFound { column_not_found, code_occurence } => Self ::
//             ColumnNotFound { column_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             PoolTimedOut { pool_timed_out, code_occurence } => Self ::
//             PoolTimedOut { pool_timed_out, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             WorkerCrashed { worker_crashed, code_occurence } => Self ::
//             WorkerCrashed { worker_crashed, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             UnexpectedCase { unexpected_case, code_occurence } => Self ::
//             UnexpectedCase { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryReadOneWithSerializeDeserialize
// {
//     fn from(val : SqlxPostgresErrorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             Configuration { configuration, code_occurence } =>
//             Self :: Configuration
//             { configuration, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence } => Self :: RowNotFound
//             { row_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             TypeNotFound { type_not_found, code_occurence } => Self ::
//             TypeNotFound { type_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnNotFound { column_not_found, code_occurence } => Self ::
//             ColumnNotFound { column_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             PoolTimedOut { pool_timed_out, code_occurence } => Self ::
//             PoolTimedOut { pool_timed_out, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             WorkerCrashed { worker_crashed, code_occurence } => Self ::
//             WorkerCrashed { worker_crashed, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             UnexpectedCase { unexpected_case, code_occurence } => Self ::
//             UnexpectedCase { unexpected_case, code_occurence }
//         }
//     }
// }
// impl From<&SqlxPostgresErrorErrorNamed> for http::StatusCode {
//     fn from(val: &SqlxPostgresErrorErrorNamed) -> Self {
//         match &val {
//             SqlxPostgresErrorErrorNamed::Configuration {
//                 configuration: _,
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
// #[allow(clippy::enum_variant_names, dead_code)]
// enum SqlxPostgresErrorErrorNamedStatusCodesChecker {
//     ConfigurationTvfrr500InternalServerError,
//     DatabaseTvfrr500InternalServerError,
//     IoTvfrr500InternalServerError,
//     TlsTvfrr500InternalServerError,
//     ProtocolTvfrr500InternalServerError,
//     RowNotFoundTvfrr404NotFound,
//     TypeNotFoundTvfrr400BadRequest,
//     ColumnIndexOutOfBoundsTvfrr500InternalServerError,
//     ColumnNotFoundTvfrr400BadRequest,
//     ColumnDecodeTvfrr500InternalServerError,
//     DecodeTvfrr500InternalServerError,
//     PoolTimedOutTvfrr408RequestTimeout,
//     PoolClosedTvfrr500InternalServerError,
//     WorkerCrashedTvfrr500InternalServerError,
//     MigrateTvfrr500InternalServerError,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl std::convert::From<SqlxPostgresErrorErrorNamedStatusCodesChecker>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryReadManyStatusCodesChecker
// {
//     fn from(val: SqlxPostgresErrorErrorNamedStatusCodesChecker) -> Self {
//         match val
//         {
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ConfigurationTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             ConfigurationTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             DatabaseTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             DatabaseTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             IoTvfrr500InternalServerError => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker :: IoTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             TlsTvfrr500InternalServerError => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker :: TlsTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ProtocolTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             ProtocolTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             RowNotFoundTvfrr404NotFound => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker :: RowNotFoundTvfrr404NotFound,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             TypeNotFoundTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker :: TypeNotFoundTvfrr400BadRequest,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnIndexOutOfBoundsTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             ColumnIndexOutOfBoundsTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnNotFoundTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker :: ColumnNotFoundTvfrr400BadRequest,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnDecodeTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             ColumnDecodeTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             DecodeTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             DecodeTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             PoolTimedOutTvfrr408RequestTimeout => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             PoolTimedOutTvfrr408RequestTimeout,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             PoolClosedTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             PoolClosedTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             WorkerCrashedTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             WorkerCrashedTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             MigrateTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             MigrateTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
// impl std::convert::From<SqlxPostgresErrorErrorNamedStatusCodesChecker>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryReadOneStatusCodesChecker
// {
//     fn from(val: SqlxPostgresErrorErrorNamedStatusCodesChecker) -> Self {
//         match val
//         {
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ConfigurationTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker ::
//             ConfigurationTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             DatabaseTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker ::
//             DatabaseTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             IoTvfrr500InternalServerError => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker :: IoTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             TlsTvfrr500InternalServerError => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker :: TlsTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ProtocolTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker ::
//             ProtocolTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             RowNotFoundTvfrr404NotFound => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker :: RowNotFoundTvfrr404NotFound,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             TypeNotFoundTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker :: TypeNotFoundTvfrr400BadRequest,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnIndexOutOfBoundsTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker ::
//             ColumnIndexOutOfBoundsTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnNotFoundTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker :: ColumnNotFoundTvfrr400BadRequest,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnDecodeTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker ::
//             ColumnDecodeTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             DecodeTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker :: DecodeTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             PoolTimedOutTvfrr408RequestTimeout => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker ::
//             PoolTimedOutTvfrr408RequestTimeout,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             PoolClosedTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker ::
//             PoolClosedTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             WorkerCrashedTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker ::
//             WorkerCrashedTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             MigrateTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker ::
//             MigrateTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
// impl std::convert::From<SqlxPostgresErrorErrorNamed>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryCreateOneResponseVariants
// {
//     fn from(val: SqlxPostgresErrorErrorNamed) -> Self {
//         match val.into_serialize_deserialize_version() {
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Configuration {
//                 configuration,
//                 code_occurence,
//             } => Self::Configuration {
//                 configuration,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             } => Self::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Io {
//                 io_error,
//                 code_occurence,
//             } => Self::Io {
//                 io_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             } => Self::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Protocol {
//                 protocol,
//                 code_occurence,
//             } => Self::Protocol {
//                 protocol,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             } => Self::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             } => Self::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             } => Self::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             } => Self::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             } => Self::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             } => Self::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             } => Self::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Migrate {
//                 migrate,
//                 code_occurence,
//             } => Self::Migrate {
//                 migrate,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// impl std::convert::From<SqlxPostgresErrorErrorNamed>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryCreateManyResponseVariants
// {
//     fn from(val: SqlxPostgresErrorErrorNamed) -> Self {
//         match val.into_serialize_deserialize_version() {
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Configuration {
//                 configuration,
//                 code_occurence,
//             } => Self::Configuration {
//                 configuration,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             } => Self::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Io {
//                 io_error,
//                 code_occurence,
//             } => Self::Io {
//                 io_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             } => Self::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Protocol {
//                 protocol,
//                 code_occurence,
//             } => Self::Protocol {
//                 protocol,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             } => Self::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             } => Self::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             } => Self::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             } => Self::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             } => Self::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             } => Self::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             } => Self::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Migrate {
//                 migrate,
//                 code_occurence,
//             } => Self::Migrate {
//                 migrate,
//                 code_occurence,
//             },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryReadManyWithBodyResponseVariants
// {
//     fn from(val : SqlxPostgresErrorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             Configuration { configuration, code_occurence } =>
//             Self :: Configuration
//             { configuration, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence } => Self :: RowNotFound
//             { row_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             TypeNotFound { type_not_found, code_occurence } => Self ::
//             TypeNotFound { type_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnNotFound { column_not_found, code_occurence } => Self ::
//             ColumnNotFound { column_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             PoolTimedOut { pool_timed_out, code_occurence } => Self ::
//             PoolTimedOut { pool_timed_out, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             WorkerCrashed { worker_crashed, code_occurence } => Self ::
//             WorkerCrashed { worker_crashed, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             UnexpectedCase { unexpected_case, code_occurence } => Self ::
//             UnexpectedCase { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryCreateOneWithSerializeDeserialize
// {
//     fn from(val : SqlxPostgresErrorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             Configuration { configuration, code_occurence } =>
//             Self :: Configuration
//             { configuration, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence } => Self :: RowNotFound
//             { row_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             TypeNotFound { type_not_found, code_occurence } => Self ::
//             TypeNotFound { type_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnNotFound { column_not_found, code_occurence } => Self ::
//             ColumnNotFound { column_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             PoolTimedOut { pool_timed_out, code_occurence } => Self ::
//             PoolTimedOut { pool_timed_out, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             WorkerCrashed { worker_crashed, code_occurence } => Self ::
//             WorkerCrashed { worker_crashed, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             UnexpectedCase { unexpected_case, code_occurence } => Self ::
//             UnexpectedCase { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryCreateManyWithSerializeDeserialize
// {
//     fn from(val : SqlxPostgresErrorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             Configuration { configuration, code_occurence } =>
//             Self :: Configuration
//             { configuration, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence } => Self :: RowNotFound
//             { row_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             TypeNotFound { type_not_found, code_occurence } => Self ::
//             TypeNotFound { type_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnNotFound { column_not_found, code_occurence } => Self ::
//             ColumnNotFound { column_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             PoolTimedOut { pool_timed_out, code_occurence } => Self ::
//             PoolTimedOut { pool_timed_out, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             WorkerCrashed { worker_crashed, code_occurence } => Self ::
//             WorkerCrashed { worker_crashed, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             UnexpectedCase { unexpected_case, code_occurence } => Self ::
//             UnexpectedCase { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryReadManyWithBodyWithSerializeDeserialize
// {
//     fn from(val : SqlxPostgresErrorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             Configuration { configuration, code_occurence } =>
//             Self :: Configuration
//             { configuration, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence } => Self :: RowNotFound
//             { row_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             TypeNotFound { type_not_found, code_occurence } => Self ::
//             TypeNotFound { type_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnNotFound { column_not_found, code_occurence } => Self ::
//             ColumnNotFound { column_not_found, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             PoolTimedOut { pool_timed_out, code_occurence } => Self ::
//             PoolTimedOut { pool_timed_out, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             WorkerCrashed { worker_crashed, code_occurence } => Self ::
//             WorkerCrashed { worker_crashed, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
//             UnexpectedCase { unexpected_case, code_occurence } => Self ::
//             UnexpectedCase { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std::convert::From<SqlxPostgresErrorErrorNamedStatusCodesChecker>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryCreateOneStatusCodesChecker
// {
//     fn from(val: SqlxPostgresErrorErrorNamedStatusCodesChecker) -> Self {
//         match val
//         {
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ConfigurationTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             ConfigurationTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             DatabaseTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             DatabaseTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             IoTvfrr500InternalServerError => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker :: IoTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             TlsTvfrr500InternalServerError => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker :: TlsTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ProtocolTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             ProtocolTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             RowNotFoundTvfrr404NotFound => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker :: RowNotFoundTvfrr404NotFound,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             TypeNotFoundTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker :: TypeNotFoundTvfrr400BadRequest,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnIndexOutOfBoundsTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             ColumnIndexOutOfBoundsTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnNotFoundTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             ColumnNotFoundTvfrr400BadRequest,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnDecodeTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             ColumnDecodeTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             DecodeTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             DecodeTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             PoolTimedOutTvfrr408RequestTimeout => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             PoolTimedOutTvfrr408RequestTimeout,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             PoolClosedTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             PoolClosedTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             WorkerCrashedTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             WorkerCrashedTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             MigrateTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             MigrateTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
// impl std::convert::From<SqlxPostgresErrorErrorNamedStatusCodesChecker>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryCreateManyStatusCodesChecker
// {
//     fn from(val: SqlxPostgresErrorErrorNamedStatusCodesChecker) -> Self {
//         match val
//         {
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ConfigurationTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             ConfigurationTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             DatabaseTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             DatabaseTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             IoTvfrr500InternalServerError => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker :: IoTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             TlsTvfrr500InternalServerError => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker :: TlsTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ProtocolTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             ProtocolTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             RowNotFoundTvfrr404NotFound => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker :: RowNotFoundTvfrr404NotFound,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             TypeNotFoundTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker :: TypeNotFoundTvfrr400BadRequest,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnIndexOutOfBoundsTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             ColumnIndexOutOfBoundsTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnNotFoundTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             ColumnNotFoundTvfrr400BadRequest,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnDecodeTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             ColumnDecodeTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             DecodeTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             DecodeTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             PoolTimedOutTvfrr408RequestTimeout => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             PoolTimedOutTvfrr408RequestTimeout,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             PoolClosedTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             PoolClosedTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             WorkerCrashedTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             WorkerCrashedTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             MigrateTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             MigrateTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
// impl std :: convert :: From < SqlxPostgresErrorErrorNamedStatusCodesChecker
// > for crate :: repositories_types :: tufa_server :: routes :: api :: cats ::
// TryReadManyWithBodyStatusCodesChecker
// {
//     fn from(val : SqlxPostgresErrorErrorNamedStatusCodesChecker,) -> Self
//     {
//         match val
//         {
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ConfigurationTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             ConfigurationTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             DatabaseTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             DatabaseTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             IoTvfrr500InternalServerError => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             IoTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             TlsTvfrr500InternalServerError => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             TlsTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ProtocolTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             ProtocolTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             RowNotFoundTvfrr404NotFound => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             RowNotFoundTvfrr404NotFound,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             TypeNotFoundTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             TypeNotFoundTvfrr400BadRequest,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnIndexOutOfBoundsTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             ColumnIndexOutOfBoundsTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnNotFoundTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             ColumnNotFoundTvfrr400BadRequest,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             ColumnDecodeTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             ColumnDecodeTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             DecodeTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             DecodeTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             PoolTimedOutTvfrr408RequestTimeout => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             PoolTimedOutTvfrr408RequestTimeout,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             PoolClosedTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             PoolClosedTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             WorkerCrashedTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             WorkerCrashedTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             MigrateTvfrr500InternalServerError => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             MigrateTvfrr500InternalServerError,
//             SqlxPostgresErrorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyWithBodyStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
