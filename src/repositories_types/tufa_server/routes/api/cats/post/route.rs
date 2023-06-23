#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
)]
pub enum PostErrorNamed<'a> {
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

impl<'a> From<PostErrorNamed<'a>> for actix_web::HttpResponse {
    fn from(val: PostErrorNamed<'a>) -> Self {
        let mut actix_web_http_response: actix_web::HttpResponseBuilder = (&val).into();
        actix_web_http_response.json(actix_web::web::Json(
            val.into_serialize_deserialize_version(),
        ))
    }
}

impl<'a> From<&PostErrorNamed<'a>> for actix_web::HttpResponseBuilder {
    fn from(val: &PostErrorNamed<'a>) -> Self {
        match &val {
            PostErrorNamed::Configuration {
                configuration_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            PostErrorNamed::Database {
                box_dyn_database_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            PostErrorNamed::Io {
                io_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            PostErrorNamed::Tls {
                box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            PostErrorNamed::Protocol {
                protocol: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            PostErrorNamed::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::NotFound(),
            PostErrorNamed::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            PostErrorNamed::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            PostErrorNamed::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            PostErrorNamed::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            PostErrorNamed::Decode {
                decode_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            PostErrorNamed::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => actix_web::HttpResponse::RequestTimeout(),
            PostErrorNamed::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            PostErrorNamed::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            PostErrorNamed::Migrate {
                migrate: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            PostErrorNamed::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
        }
    }
}
