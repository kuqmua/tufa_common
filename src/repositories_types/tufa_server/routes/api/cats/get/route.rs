#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum GetHttpResponse {
    Cats(Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>),
    //
    // ProjectCommitExtractorNotEqual {
    //     project_commit_not_equal: std::string::String,
    //     project_commit_to_use: std::string::String,
    //     code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    // },
    // ProjectCommitExtractorToStrConversion {
    //     project_commit_to_str_conversion: std::string::String,
    //     code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    // },
    // NoProjectCommitExtractorHeader {
    //     no_project_commit_header: std::string::String,
    //     code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    // },
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

impl From<GetHttpResponse> for actix_web::HttpResponse {
    fn from(val: GetHttpResponse) -> Self {
        let mut actix_web_http_response: actix_web::HttpResponseBuilder = (&val).into();
        match Vec::<crate::repositories_types::tufa_server::routes::api::cats::Cat>::try_from(val) {
            Ok(vec_cat) => actix_web_http_response.json(vec_cat),
            Err(e) => actix_web_http_response.json(actix_web::web::Json(e)),
        }
    }
}

impl From<&GetHttpResponse> for actix_web::HttpResponseBuilder {
    fn from(val: &GetHttpResponse) -> Self {
        match &val {
            GetHttpResponse::Cats(_vec_cats) => actix_web::HttpResponse::Ok(),
            GetHttpResponse::Configuration {
                configuration_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetHttpResponse::Database {
                box_dyn_database_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetHttpResponse::Io {
                io_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetHttpResponse::Tls {
                box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetHttpResponse::Protocol {
                protocol: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetHttpResponse::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::NotFound(),
            GetHttpResponse::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            GetHttpResponse::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetHttpResponse::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            GetHttpResponse::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetHttpResponse::Decode {
                decode_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetHttpResponse::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => actix_web::HttpResponse::RequestTimeout(),
            GetHttpResponse::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetHttpResponse::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetHttpResponse::Migrate {
                migrate: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetHttpResponse::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
        }
    }
}

impl<'a> From<GetErrorNamed<'a>> for GetHttpResponse {
    fn from(val: GetErrorNamed<'a>) -> Self {
        match val.into_serialize_deserialize_version() {
            GetErrorNamedWithSerializeDeserialize::Configuration {
                configuration_box_dyn_error,
                code_occurence,
            } => GetHttpResponse::Configuration {
                configuration_box_dyn_error,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::Database {
                box_dyn_database_error,
                code_occurence,
            } => GetHttpResponse::Database {
                box_dyn_database_error,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::Io {
                io_error,
                code_occurence,
            } => GetHttpResponse::Io {
                io_error,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::Tls {
                box_dyn_error,
                code_occurence,
            } => GetHttpResponse::Tls {
                box_dyn_error,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            } => GetHttpResponse::Protocol {
                protocol,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            } => GetHttpResponse::RowNotFound {
                row_not_found,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            } => GetHttpResponse::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => GetHttpResponse::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => GetHttpResponse::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => GetHttpResponse::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::Decode {
                decode_box_dyn_error,
                code_occurence,
            } => GetHttpResponse::Decode {
                decode_box_dyn_error,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => GetHttpResponse::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            } => GetHttpResponse::PoolClosed {
                pool_closed,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => GetHttpResponse::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            } => GetHttpResponse::Migrate {
                migrate,
                code_occurence,
            },
            GetErrorNamedWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => GetHttpResponse::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}

#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
)]
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
