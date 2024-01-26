// #[derive(Debug, sqlx::Type)]
// #[sqlx(type_name = "header_pair")]
// pub struct HeaderPairRecord {
//     pub name: std::string::String,
//     pub value: Vec<u8>,
// }

// impl sqlx::postgres::PgHasArrayType for HeaderPairRecord {
//     fn array_type_info() -> sqlx::postgres::PgTypeInfo {
//         sqlx::postgres::PgTypeInfo::with_name("_header_pair")
//     }
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum GetSavedResponseErrorNamed {
//     PostgresSelect {
//         #[eo_display]
//         postgres_select: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     TryFromIntError {
//         #[eo_display]
//         try_from_int_error: std::num::TryFromIntError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     InvalidStatusCode {
//         #[eo_display]
//         invalid_status_code: http::status::InvalidStatusCode,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum SaveResponseErrorNamed {
//     BodyToBytes {
//         #[eo_display]
//         body_to_bytes: actix_web::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     PostgtesUpdate {
//         #[eo_display]
//         postgres_update: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     PostgtesTransactionCommit {
//         #[eo_display]
//         postgres_transaction_commit: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// pub enum NextAction {
//     ReturnSavedResponse(actix_web::HttpResponse),
//     StartProcessing(sqlx::Transaction<'static, sqlx::Postgres>),
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum TryProcessingErrorNamed {
//     PostgresPoolBegin {
//         #[eo_display]
//         pool_begin_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     PostgresInsert {
//         #[eo_display]
//         insert: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     GetSavedResponse {
//         #[eo_error_occurence]
//         get_saved_response: GetSavedResponseErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     SavedResponseIsNone {
//         #[eo_display_with_serialize_deserialize]
//         message: &'a str,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
