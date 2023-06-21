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

impl<'a> From<&GetErrorNamed<'a>> for actix_web::HttpResponseBuilder {
    fn from(val: &GetErrorNamed<'a>) -> Self {
        match &val {
            GetErrorNamed::Configuration {
                configuration_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetErrorNamed::Database {
                box_dyn_database_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetErrorNamed::Io {
                io_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetErrorNamed::Tls {
                box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetErrorNamed::Protocol {
                protocol: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetErrorNamed::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::NotFound(),
            GetErrorNamed::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            GetErrorNamed::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetErrorNamed::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            GetErrorNamed::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetErrorNamed::Decode {
                decode_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetErrorNamed::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => actix_web::HttpResponse::RequestTimeout(),
            GetErrorNamed::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetErrorNamed::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetErrorNamed::Migrate {
                migrate: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            GetErrorNamed::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
        }
    }
}

////////////////
#[derive(Debug, serde :: Serialize, serde :: Deserialize)] //thiserror :: Error
pub enum GetHttpResponse {
    Cats(Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>),
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

impl TryFrom<GetHttpResponse>
    for Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>
{
    type Error = GetErrorNamedWithSerializeDeserialize;
    fn try_from(value: GetHttpResponse) -> Result<Self, GetErrorNamedWithSerializeDeserialize> {
        match value {
            GetHttpResponse::Cats(cats) => Ok(cats),
            GetHttpResponse::Configuration {
                configuration_box_dyn_error,
                code_occurence,
            } => Err(GetErrorNamedWithSerializeDeserialize::Configuration {
                configuration_box_dyn_error,
                code_occurence,
            }),
            GetHttpResponse::Database {
                box_dyn_database_error,
                code_occurence,
            } => Err(GetErrorNamedWithSerializeDeserialize::Database {
                box_dyn_database_error,
                code_occurence,
            }),
            GetHttpResponse::Io {
                io_error,
                code_occurence,
            } => todo!(),
            GetHttpResponse::Tls {
                box_dyn_error,
                code_occurence,
            } => todo!(),
            GetHttpResponse::Protocol {
                protocol,
                code_occurence,
            } => todo!(),
            GetHttpResponse::RowNotFound {
                row_not_found,
                code_occurence,
            } => todo!(),
            GetHttpResponse::TypeNotFound {
                type_not_found,
                code_occurence,
            } => todo!(),
            GetHttpResponse::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => todo!(),
            GetHttpResponse::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => todo!(),
            GetHttpResponse::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => todo!(),
            GetHttpResponse::Decode {
                decode_box_dyn_error,
                code_occurence,
            } => todo!(),
            GetHttpResponse::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => todo!(),
            GetHttpResponse::PoolClosed {
                pool_closed,
                code_occurence,
            } => todo!(),
            GetHttpResponse::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => todo!(),
            GetHttpResponse::Migrate {
                migrate,
                code_occurence,
            } => todo!(),
            GetHttpResponse::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => todo!(),
        }
    }
}
