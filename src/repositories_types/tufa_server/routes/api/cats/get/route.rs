#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    into_actix_web_http_response::IntoActixWebHttpResponse,
    from_enum::FromEnumWithoutSerializeDeserialize,
)]
#[from_enum::from_enum_paths_without_serialize_deserialize(
    crate::repositories_types::tufa_server::routes::api::cats::get::request::TryGetResponseVariants
)]
pub enum GetHttpResponse {
    DesirableType(Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>),
    //
    Configuration {
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    Database {
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    Io {
        io_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    Tls {
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    Decode {
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
}

impl From<&GetHttpResponse> for http::StatusCode {
    fn from(val: &GetHttpResponse) -> Self {
        match &val {
            GetHttpResponse::DesirableType(_vec_cats) => http::StatusCode::OK,
            GetHttpResponse::Configuration {
                configuration_box_dyn_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponse::Database {
                box_dyn_database_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponse::Io {
                io_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponse::Tls {
                box_dyn_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponse::Protocol {
                protocol: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponse::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => http::StatusCode::NOT_FOUND,
            GetHttpResponse::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            GetHttpResponse::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponse::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            GetHttpResponse::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponse::Decode {
                decode_box_dyn_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponse::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => http::StatusCode::REQUEST_TIMEOUT,
            GetHttpResponse::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponse::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponse::Migrate {
                migrate: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponse::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    from_enum::FromEnumWithLifetime,
)]
#[from_enum::from_enum_paths_with_lifetime(GetHttpResponse)] //todo maybe add lifetime here ?
pub enum GetErrorNamed<'a> {
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
