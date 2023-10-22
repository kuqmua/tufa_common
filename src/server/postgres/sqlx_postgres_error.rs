#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::tufa_server::routes::api::cats::TryRead,
    crate::repositories_types::tufa_server::routes::api::cats::TryReadOne,
    crate::repositories_types::tufa_server::routes::api::cats::TryCreateOne,
    crate::repositories_types::tufa_server::routes::api::cats::TryCreateMany,
    crate::repositories_types::tufa_server::routes::api::cats::TryReadManyWithBody
)]
pub enum SqlxPostgresErrorErrorNamed {
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
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
