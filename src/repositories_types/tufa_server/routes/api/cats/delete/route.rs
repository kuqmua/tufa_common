// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence::ErrorOccurence,
//     from_sqlx_postgres_error::FromSqlxPostgresError,
// )]
// pub enum DeleteErrorNamed<'a> {
//     NoParameters {
//         #[eo_display_with_serialize_deserialize]
//         no_parameters: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     //
//     Configuration {
//         #[eo_display_with_serialize_deserialize]
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     Database {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     Io {
//         #[eo_display]
//         io_error: std::io::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     Tls {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     Protocol {
//         #[eo_display_with_serialize_deserialize]
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     RowNotFound {
//         #[eo_display_with_serialize_deserialize]
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     TypeNotFound {
//         #[eo_display_with_serialize_deserialize]
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     ColumnIndexOutOfBounds {
//         #[eo_display_with_serialize_deserialize]
//         column_index_out_of_bounds: usize,
//         #[eo_display_with_serialize_deserialize]
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     ColumnNotFound {
//         #[eo_display_with_serialize_deserialize]
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     ColumnDecode {
//         #[eo_display_with_serialize_deserialize]
//         column_decode_index: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     Decode {
//         #[eo_display_with_serialize_deserialize]
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     PoolTimedOut {
//         #[eo_display_with_serialize_deserialize]
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     PoolClosed {
//         #[eo_display_with_serialize_deserialize]
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     WorkerCrashed {
//         #[eo_display_with_serialize_deserialize]
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     Migrate {
//         #[eo_display]
//         migrate: sqlx::migrate::MigrateError,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     //#[non_exhaustive] case
//     UnexpectedCase {
//         #[eo_display_with_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }

// impl<'a> From<DeleteErrorNamed<'a>> for actix_web::HttpResponse {
//     fn from(val: DeleteErrorNamed<'a>) -> Self {
//         let mut actix_web_http_response: actix_web::HttpResponseBuilder = (&val).into();
//         actix_web_http_response.json(actix_web::web::Json(
//             val.into_serialize_deserialize_version(),
//         ))
//     }
// }

// impl<'a> From<&DeleteErrorNamed<'a>> for actix_web::HttpResponseBuilder {
//     fn from(val: &DeleteErrorNamed<'a>) -> Self {
//         match &val {
//             DeleteErrorNamed::NoParameters {
//                 no_parameters: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::BadRequest(),
//             DeleteErrorNamed::Configuration {
//                 configuration_box_dyn_error: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             DeleteErrorNamed::Database {
//                 box_dyn_database_error: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             DeleteErrorNamed::Io {
//                 io_error: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             DeleteErrorNamed::Tls {
//                 box_dyn_error: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             DeleteErrorNamed::Protocol {
//                 protocol: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             DeleteErrorNamed::RowNotFound {
//                 row_not_found: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::NotFound(),
//             DeleteErrorNamed::TypeNotFound {
//                 type_not_found: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::BadRequest(),
//             DeleteErrorNamed::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds: _,
//                 len: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             DeleteErrorNamed::ColumnNotFound {
//                 column_not_found: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::BadRequest(),
//             DeleteErrorNamed::ColumnDecode {
//                 column_decode_index: _,
//                 source_handle: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             DeleteErrorNamed::Decode {
//                 decode_box_dyn_error: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             DeleteErrorNamed::PoolTimedOut {
//                 pool_timed_out: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::RequestTimeout(),
//             DeleteErrorNamed::PoolClosed {
//                 pool_closed: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             DeleteErrorNamed::WorkerCrashed {
//                 worker_crashed: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             DeleteErrorNamed::Migrate {
//                 migrate: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//             DeleteErrorNamed::UnexpectedCase {
//                 unexpected_case: _,
//                 code_occurence: _,
//             } => actix_web::HttpResponse::InternalServerError(),
//         }
//     }
// }
