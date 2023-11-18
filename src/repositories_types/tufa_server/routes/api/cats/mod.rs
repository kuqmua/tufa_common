//todo openapi
//todo test if create\update\delete empty array
pub trait GetConfigGetPostgresPool:
    crate::repositories_types::tufa_server::config::config_struct::GetConfig
    + crate::server::routes::helpers::get_postgres_pool::GetPostgresPool
    + crate::common::config::config_fields::GetSourcePlaceType
    + crate::common::config::config_fields::GetTimezone
{
}

pub type DynArcGetConfigGetPostgresPoolSendSync = std::sync::Arc<
    dyn crate::repositories_types::tufa_server::routes::api::cats::GetConfigGetPostgresPool
        + Send
        + Sync,
>;

#[derive(
    Debug,
    generate_postgresql_crud::GeneratePostgresqlCrud,
)]
pub struct Dog {
    #[generate_postgresql_crud_primary_key]
    pub id: sqlx::types::Uuid, //todo make it UuidWrapper todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    #[generate_postgresql_crud_varchar]
    pub name: std::string::String,
    #[generate_postgresql_crud_varchar]
    pub color: std::string::String,
}

// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence::ErrorOccurence,
//     from_sqlx_postgres_error::FromSqlxPostgresError,
//     type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
// )]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
//     std::vec::Vec::<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>,
//     tvfrr_201_created
// )]
// pub enum TryCreateMany {
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorNotEqual {
//         #[eo_display_with_serialize_deserialize]
//         project_commit_not_equal: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorToStrConversion {
//         #[eo_display]
//         project_commit_to_str_conversion: http::header::ToStrError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoProjectCommitExtractorHeader {
//         #[eo_display_with_serialize_deserialize]
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_500_internal_server_error]
//     Configuration {
//         #[eo_display_with_serialize_deserialize]
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Database {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Io {
//         #[eo_display]
//         io_error: std::io::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Tls {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Protocol {
//         #[eo_display_with_serialize_deserialize]
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_404_not_found]
//     RowNotFound {
//         #[eo_display_with_serialize_deserialize]
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     TypeNotFound {
//         #[eo_display_with_serialize_deserialize]
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnIndexOutOfBounds {
//         #[eo_display_with_serialize_deserialize]
//         column_index_out_of_bounds: usize,
//         #[eo_display_with_serialize_deserialize]
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ColumnNotFound {
//         #[eo_display_with_serialize_deserialize]
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnDecode {
//         #[eo_display_with_serialize_deserialize]
//         column_decode_index: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Decode {
//         #[eo_display_with_serialize_deserialize]
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_408_request_timeout]
//     PoolTimedOut {
//         #[eo_display_with_serialize_deserialize]
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PoolClosed {
//         #[eo_display_with_serialize_deserialize]
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     WorkerCrashed {
//         #[eo_display_with_serialize_deserialize]
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Migrate {
//         #[eo_display]
//         migrate: sqlx::migrate::MigrateError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     JsonDataError {
//         #[eo_display]
//         json_data_error: axum::extract::rejection::JsonDataError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     JsonSyntaxError {
//         #[eo_display]
//         json_syntax_error: axum::extract::rejection::JsonSyntaxError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     MissingJsonContentType {
//         #[eo_display_with_serialize_deserialize]
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     BytesRejection {
//         #[eo_display_with_serialize_deserialize]
//         bytes_rejection: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_500_internal_server_error]
//     BindQuery {
//         #[eo_error_occurence]
//         checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error] //todo what status should be there?
//     CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
//         #[eo_display]
//         uuid_wrapper_try_from_possible_uuid_wrapper_in_server: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //#[non_exhaustive] case
//     #[tvfrr_500_internal_server_error]
//     UnexpectedCase {
//         #[eo_display_with_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// // ////////////
// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence::ErrorOccurence,
//     from_sqlx_postgres_error::FromSqlxPostgresError,
//     type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
// )]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
//     crate::server::postgres::uuid_wrapper::PossibleUuidWrapper,
//     tvfrr_201_created
// )]
// pub enum TryCreateOne {
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorNotEqual {
//         #[eo_display_with_serialize_deserialize]
//         project_commit_not_equal: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorToStrConversion {
//         #[eo_display]
//         project_commit_to_str_conversion: http::header::ToStrError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoProjectCommitExtractorHeader {
//         #[eo_display_with_serialize_deserialize]
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_500_internal_server_error]
//     Configuration {
//         #[eo_display_with_serialize_deserialize]
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Database {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Io {
//         #[eo_display]
//         io_error: std::io::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Tls {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Protocol {
//         #[eo_display_with_serialize_deserialize]
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_404_not_found]
//     RowNotFound {
//         #[eo_display_with_serialize_deserialize]
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     TypeNotFound {
//         #[eo_display_with_serialize_deserialize]
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnIndexOutOfBounds {
//         #[eo_display_with_serialize_deserialize]
//         column_index_out_of_bounds: usize,
//         #[eo_display_with_serialize_deserialize]
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ColumnNotFound {
//         #[eo_display_with_serialize_deserialize]
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnDecode {
//         #[eo_display_with_serialize_deserialize]
//         column_decode_index: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Decode {
//         #[eo_display_with_serialize_deserialize]
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_408_request_timeout]
//     PoolTimedOut {
//         #[eo_display_with_serialize_deserialize]
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PoolClosed {
//         #[eo_display_with_serialize_deserialize]
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     WorkerCrashed {
//         #[eo_display_with_serialize_deserialize]
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Migrate {
//         #[eo_display]
//         migrate: sqlx::migrate::MigrateError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     JsonDataError {
//         #[eo_display]
//         json_data_error: axum::extract::rejection::JsonDataError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     JsonSyntaxError {
//         #[eo_display]
//         json_syntax_error: axum::extract::rejection::JsonSyntaxError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     MissingJsonContentType {
//         #[eo_display_with_serialize_deserialize]
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     BytesRejection {
//         #[eo_display_with_serialize_deserialize]
//         bytes_rejection: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error] //todo what status should be there?
//     CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
//         #[eo_display]
//         uuid_wrapper_try_from_possible_uuid_wrapper_in_server: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //#[non_exhaustive] case
//     #[tvfrr_500_internal_server_error]
//     UnexpectedCase {
//         #[eo_display_with_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// // ////////
// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence::ErrorOccurence,
//     from_sqlx_postgres_error::FromSqlxPostgresError,
//     type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
// )]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
//     (),
//     tvfrr_200_ok
// )]
// pub enum TryDeleteOne {
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorNotEqual {
//         #[eo_display_with_serialize_deserialize]
//         project_commit_not_equal: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorToStrConversion {
//         #[eo_display]
//         project_commit_to_str_conversion: http::header::ToStrError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoProjectCommitExtractorHeader {
//         #[eo_display_with_serialize_deserialize]
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_500_internal_server_error]
//     Configuration {
//         #[eo_display_with_serialize_deserialize]
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Database {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Io {
//         #[eo_display]
//         io_error: std::io::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Tls {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Protocol {
//         #[eo_display_with_serialize_deserialize]
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_404_not_found]
//     RowNotFound {
//         #[eo_display_with_serialize_deserialize]
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     TypeNotFound {
//         #[eo_display_with_serialize_deserialize]
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnIndexOutOfBounds {
//         #[eo_display_with_serialize_deserialize]
//         column_index_out_of_bounds: usize,
//         #[eo_display_with_serialize_deserialize]
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ColumnNotFound {
//         #[eo_display_with_serialize_deserialize]
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnDecode {
//         #[eo_display_with_serialize_deserialize]
//         column_decode_index: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Decode {
//         #[eo_display_with_serialize_deserialize]
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_408_request_timeout]
//     PoolTimedOut {
//         #[eo_display_with_serialize_deserialize]
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PoolClosed {
//         #[eo_display_with_serialize_deserialize]
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     WorkerCrashed {
//         #[eo_display_with_serialize_deserialize]
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Migrate {
//         #[eo_display]
//         migrate: sqlx::migrate::MigrateError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     FailedToDeserializePathParams {
//         #[eo_display_with_serialize_deserialize]
//         failed_to_deserialize_path_params: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     MissingPathParams {
//         #[eo_display_with_serialize_deserialize]
//         missing_path_params: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize {
//         #[eo_error_occurence]
//         delete_one_path_try_from_delete_one_path_with_serialize_deserialize:
//             DeleteOnePathTryFromDeleteOnePathWithSerializeDeserializeErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     //#[non_exhaustive] case
//     #[tvfrr_500_internal_server_error]
//     UnexpectedCase {
//         #[eo_display_with_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// // ///////////
// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence::ErrorOccurence,
//     from_sqlx_postgres_error::FromSqlxPostgresError,
//     type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
// )]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
//     (),
//     tvfrr_200_ok
// )]
// pub enum TryDeleteManyWithBody {
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorNotEqual {
//         #[eo_display_with_serialize_deserialize]
//         project_commit_not_equal: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorToStrConversion {
//         #[eo_display]
//         project_commit_to_str_conversion: http::header::ToStrError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoProjectCommitExtractorHeader {
//         #[eo_display_with_serialize_deserialize]
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_500_internal_server_error]
//     Configuration {
//         #[eo_display_with_serialize_deserialize]
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Database {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Io {
//         #[eo_display]
//         io_error: std::io::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Tls {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Protocol {
//         #[eo_display_with_serialize_deserialize]
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_404_not_found]
//     RowNotFound {
//         #[eo_display_with_serialize_deserialize]
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     TypeNotFound {
//         #[eo_display_with_serialize_deserialize]
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnIndexOutOfBounds {
//         #[eo_display_with_serialize_deserialize]
//         column_index_out_of_bounds: usize,
//         #[eo_display_with_serialize_deserialize]
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ColumnNotFound {
//         #[eo_display_with_serialize_deserialize]
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnDecode {
//         #[eo_display_with_serialize_deserialize]
//         column_decode_index: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Decode {
//         #[eo_display_with_serialize_deserialize]
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_408_request_timeout]
//     PoolTimedOut {
//         #[eo_display_with_serialize_deserialize]
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PoolClosed {
//         #[eo_display_with_serialize_deserialize]
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     WorkerCrashed {
//         #[eo_display_with_serialize_deserialize]
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Migrate {
//         #[eo_display]
//         migrate: sqlx::migrate::MigrateError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     JsonDataError {
//         #[eo_display]
//         json_data_error: axum::extract::rejection::JsonDataError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     JsonSyntaxError {
//         #[eo_display]
//         json_syntax_error: axum::extract::rejection::JsonSyntaxError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     MissingJsonContentType {
//         #[eo_display_with_serialize_deserialize]
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     BytesRejection {
//         #[eo_display_with_serialize_deserialize]
//         bytes_rejection: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     NotUniquePrimaryKey {
//         #[eo_vec_display]
//         not_unique_primary_keys: Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NotUniqueNameVec {
//         #[eo_vec_display_with_serialize_deserialize]
//         not_unique_name_vec: Vec<crate::server::postgres::regex_filter::RegexFilter>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NotUniqueColorVec {
//         #[eo_vec_display_with_serialize_deserialize]
//         not_unique_color_vec: Vec<crate::server::postgres::regex_filter::RegexFilter>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     BindQuery {
//         #[eo_error_occurence]
//         checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoPayloadFields {
//         #[eo_display_with_serialize_deserialize]
//         no_payload_fields: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoPayloadParameters {
//         #[eo_display_with_serialize_deserialize]
//         no_payload_parameters: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NonExistingPrimaryKeys {
//         #[eo_vec_display]
//         non_existing_primary_keys: Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     //todo what status code should return if non_existing_primary_keys = 400, but transaction rollback failed = 500
//     NonExistingPrimaryKeysAndFailedRollback {
//         #[eo_vec_display]
//         non_existing_primary_keys: Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
//         #[eo_display]
//         rollback_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PrimaryKeyFromRowAndFailedRollback {
//         #[eo_display]
//         primary_key_from_row: sqlx::Error,
//         #[eo_display]
//         rollback_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     CommitFailed {
//         #[eo_display]
//         commit_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     QueryAndRollbackFailed {
//         #[eo_display]
//         query_error: sqlx::Error,
//         #[eo_display]
//         rollback_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize {
//         #[eo_error_occurence]
//         delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize: DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserializeErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //#[non_exhaustive] case
//     #[tvfrr_500_internal_server_error]
//     UnexpectedCase {
//         #[eo_display_with_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// // /////////////
// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence::ErrorOccurence,
//     from_sqlx_postgres_error::FromSqlxPostgresError,
//     type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
// )]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
//     (),
//     tvfrr_200_ok
// )]
// pub enum TryDeleteMany {
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorNotEqual {
//         #[eo_display_with_serialize_deserialize]
//         project_commit_not_equal: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorToStrConversion {
//         #[eo_display]
//         project_commit_to_str_conversion: http::header::ToStrError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoProjectCommitExtractorHeader {
//         #[eo_display_with_serialize_deserialize]
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_500_internal_server_error]
//     Configuration {
//         #[eo_display_with_serialize_deserialize]
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Database {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Io {
//         #[eo_display]
//         io_error: std::io::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Tls {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Protocol {
//         #[eo_display_with_serialize_deserialize]
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_404_not_found]
//     RowNotFound {
//         #[eo_display_with_serialize_deserialize]
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     TypeNotFound {
//         #[eo_display_with_serialize_deserialize]
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnIndexOutOfBounds {
//         #[eo_display_with_serialize_deserialize]
//         column_index_out_of_bounds: usize,
//         #[eo_display_with_serialize_deserialize]
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ColumnNotFound {
//         #[eo_display_with_serialize_deserialize]
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnDecode {
//         #[eo_display_with_serialize_deserialize]
//         column_decode_index: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Decode {
//         #[eo_display_with_serialize_deserialize]
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_408_request_timeout]
//     PoolTimedOut {
//         #[eo_display_with_serialize_deserialize]
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PoolClosed {
//         #[eo_display_with_serialize_deserialize]
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     WorkerCrashed {
//         #[eo_display_with_serialize_deserialize]
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Migrate {
//         #[eo_display]
//         migrate: sqlx::migrate::MigrateError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     FailedToDeserializeQueryString {
//         #[eo_display_with_serialize_deserialize]
//         failed_to_deserialize_query_string: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     NotUniquePrimaryKey {
//         #[eo_vec_display]
//         not_unique_primary_keys: Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NotUniqueNameVec {
//         #[eo_vec_display_with_serialize_deserialize]
//         not_unique_name_vec: Vec<std::string::String>, //todo make it crate::server::postgres::regex_filter::RegexFilter
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NotUniqueColorVec {
//         #[eo_vec_display_with_serialize_deserialize]
//         not_unique_color_vec: Vec<std::string::String>, //todo make it crate::server::postgres::regex_filter::RegexFilter
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     BindQuery {
//         #[eo_error_occurence]
//         checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoQueryParameters {
//         #[eo_display_with_serialize_deserialize]
//         no_query_parameters: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     CommitFailed {
//         #[eo_display]
//         commit_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NonExistingPrimaryKeys {
//         #[eo_vec_display]
//         non_existing_primary_keys: Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PrimaryKeyFromRowAndFailedRollback {
//         #[eo_display]
//         primary_key_from_row: sqlx::Error,
//         #[eo_display]
//         rollback_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     //todo what status code should return if non_existing_primary_keys = 400, but transaction rollback failed = 500
//     NonExistingPrimaryKeysAndFailedRollback {
//         #[eo_vec_display]
//         non_existing_primary_keys: Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
//         #[eo_display]
//         rollback_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     QueryAndRollbackFailed {
//         #[eo_display]
//         query_error: sqlx::Error,
//         #[eo_display]
//         rollback_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserialize {
//         #[eo_error_occurence]
//         delete_many_query_try_from_delete_many_query_with_serialize_deserialize:
//             DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserializeErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //#[non_exhaustive] case
//     #[tvfrr_500_internal_server_error]
//     UnexpectedCase {
//         #[eo_display_with_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// // /////////////
// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence::ErrorOccurence,
//     from_sqlx_postgres_error::FromSqlxPostgresError,
//     type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
// )]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
//     crate::repositories_types::tufa_server::routes::api::cats::DogOptions,
//     tvfrr_200_ok
// )]
// pub enum TryReadOne {
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorNotEqual {
//         #[eo_display_with_serialize_deserialize]
//         project_commit_not_equal: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorToStrConversion {
//         #[eo_display]
//         project_commit_to_str_conversion: http::header::ToStrError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoProjectCommitExtractorHeader {
//         #[eo_display_with_serialize_deserialize]
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_500_internal_server_error]
//     Configuration {
//         #[eo_display_with_serialize_deserialize]
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Database {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Io {
//         #[eo_display]
//         io_error: std::io::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Tls {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Protocol {
//         #[eo_display_with_serialize_deserialize]
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_404_not_found]
//     RowNotFound {
//         #[eo_display_with_serialize_deserialize]
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     TypeNotFound {
//         #[eo_display_with_serialize_deserialize]
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnIndexOutOfBounds {
//         #[eo_display_with_serialize_deserialize]
//         column_index_out_of_bounds: usize,
//         #[eo_display_with_serialize_deserialize]
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ColumnNotFound {
//         #[eo_display_with_serialize_deserialize]
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnDecode {
//         #[eo_display_with_serialize_deserialize]
//         column_decode_index: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Decode {
//         #[eo_display_with_serialize_deserialize]
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_408_request_timeout]
//     PoolTimedOut {
//         #[eo_display_with_serialize_deserialize]
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PoolClosed {
//         #[eo_display_with_serialize_deserialize]
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     WorkerCrashed {
//         #[eo_display_with_serialize_deserialize]
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Migrate {
//         #[eo_display]
//         migrate: sqlx::migrate::MigrateError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     FailedToDeserializePathParams {
//         #[eo_display_with_serialize_deserialize]
//         failed_to_deserialize_path_params: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     MissingPathParams {
//         #[eo_display_with_serialize_deserialize]
//         missing_path_params: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     FailedToDeserializeQueryString {
//         #[eo_display_with_serialize_deserialize]
//         failed_to_deserialize_query_string: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ReadOnePathTryFromReadOnePathWithSerializeDeserialize {
//         #[eo_error_occurence]
//         read_one_path_try_from_read_one_path_with_serialize_deserialize:
//             ReadOnePathTryFromReadOnePathWithSerializeDeserializeErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //#[non_exhaustive] case
//     #[tvfrr_500_internal_server_error]
//     UnexpectedCase {
//         #[eo_display_with_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// // ////////
// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence::ErrorOccurence,
//     from_sqlx_postgres_error::FromSqlxPostgresError,
//     type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
// )]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
//     Vec::<crate::repositories_types::tufa_server::routes::api::cats::DogOptions>,
//     tvfrr_200_ok
// )]
// pub enum TryReadManyWithBody {
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorNotEqual {
//         #[eo_display_with_serialize_deserialize]
//         project_commit_not_equal: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorToStrConversion {
//         #[eo_display]
//         project_commit_to_str_conversion: http::header::ToStrError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoProjectCommitExtractorHeader {
//         #[eo_display_with_serialize_deserialize]
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_500_internal_server_error]
//     Configuration {
//         #[eo_display_with_serialize_deserialize]
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Database {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Io {
//         #[eo_display]
//         io_error: std::io::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Tls {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Protocol {
//         #[eo_display_with_serialize_deserialize]
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_404_not_found]
//     RowNotFound {
//         #[eo_display_with_serialize_deserialize]
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     TypeNotFound {
//         #[eo_display_with_serialize_deserialize]
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnIndexOutOfBounds {
//         #[eo_display_with_serialize_deserialize]
//         column_index_out_of_bounds: usize,
//         #[eo_display_with_serialize_deserialize]
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ColumnNotFound {
//         #[eo_display_with_serialize_deserialize]
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnDecode {
//         #[eo_display_with_serialize_deserialize]
//         column_decode_index: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Decode {
//         #[eo_display_with_serialize_deserialize]
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_408_request_timeout]
//     PoolTimedOut {
//         #[eo_display_with_serialize_deserialize]
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PoolClosed {
//         #[eo_display_with_serialize_deserialize]
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     WorkerCrashed {
//         #[eo_display_with_serialize_deserialize]
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Migrate {
//         #[eo_display]
//         migrate: sqlx::migrate::MigrateError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     JsonDataError {
//         #[eo_display]
//         json_data_error: axum::extract::rejection::JsonDataError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     JsonSyntaxError {
//         #[eo_display]
//         json_syntax_error: axum::extract::rejection::JsonSyntaxError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     MissingJsonContentType {
//         #[eo_display_with_serialize_deserialize]
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     BytesRejection {
//         #[eo_display_with_serialize_deserialize]
//         bytes_rejection: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     NotUniquePrimaryKey {
//         #[eo_vec_display]
//         not_unique_primary_keys: Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NotUniqueNameVec {
//         #[eo_vec_display_with_serialize_deserialize]
//         not_unique_name_vec: Vec<crate::server::postgres::regex_filter::RegexFilter>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NotUniqueColorVec {
//         #[eo_vec_display_with_serialize_deserialize]
//         not_unique_color_vec: Vec<crate::server::postgres::regex_filter::RegexFilter>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     BindQuery {
//         #[eo_error_occurence]
//         checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NotUuid {
//         #[eo_display]
//         not_uuid: sqlx::types::uuid::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize {
//         #[eo_error_occurence]
//         read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize:
//             ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserializeErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //#[non_exhaustive] case
//     #[tvfrr_500_internal_server_error]
//     UnexpectedCase {
//         #[eo_display_with_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// // ////////
// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence::ErrorOccurence,
//     from_sqlx_postgres_error::FromSqlxPostgresError,
//     type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
// )]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
//     Vec::<crate::repositories_types::tufa_server::routes::api::cats::DogOptions>,
//     tvfrr_200_ok
// )]
// pub enum TryReadMany {
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorNotEqual {
//         #[eo_display_with_serialize_deserialize]
//         project_commit_not_equal: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorToStrConversion {
//         #[eo_display]
//         project_commit_to_str_conversion: http::header::ToStrError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoProjectCommitExtractorHeader {
//         #[eo_display_with_serialize_deserialize]
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_500_internal_server_error]
//     Configuration {
//         #[eo_display_with_serialize_deserialize]
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Database {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Io {
//         #[eo_display]
//         io_error: std::io::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Tls {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Protocol {
//         #[eo_display_with_serialize_deserialize]
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_404_not_found]
//     RowNotFound {
//         #[eo_display_with_serialize_deserialize]
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     TypeNotFound {
//         #[eo_display_with_serialize_deserialize]
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnIndexOutOfBounds {
//         #[eo_display_with_serialize_deserialize]
//         column_index_out_of_bounds: usize,
//         #[eo_display_with_serialize_deserialize]
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ColumnNotFound {
//         #[eo_display_with_serialize_deserialize]
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnDecode {
//         #[eo_display_with_serialize_deserialize]
//         column_decode_index: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Decode {
//         #[eo_display_with_serialize_deserialize]
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_408_request_timeout]
//     PoolTimedOut {
//         #[eo_display_with_serialize_deserialize]
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PoolClosed {
//         #[eo_display_with_serialize_deserialize]
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     WorkerCrashed {
//         #[eo_display_with_serialize_deserialize]
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Migrate {
//         #[eo_display]
//         migrate: sqlx::migrate::MigrateError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     NotUniquePrimaryKey {
//         #[eo_vec_display]
//         not_unique_primary_keys: Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NotUniqueNameVec {
//         #[eo_vec_display_with_serialize_deserialize]
//         not_unique_name_vec: Vec<std::string::String>, //todo crate::server::postgres::regex_filter::RegexFilter
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NotUniqueColorVec {
//         #[eo_vec_display_with_serialize_deserialize]
//         not_unique_color_vec: Vec<std::string::String>, //todo crate::server::postgres::regex_filter::RegexFilter
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     FailedToDeserializeQueryString {
//         #[eo_display_with_serialize_deserialize]
//         failed_to_deserialize_query_string: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     BindQuery {
//         #[eo_error_occurence]
//         checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ReadManyQueryTryFromReadManyQueryWithSerializeDeserialize {
//         #[eo_error_occurence]
//         read_many_query_try_from_read_many_query_with_serialize_deserialize:
//             ReadManyQueryTryFromReadManyQueryWithSerializeDeserializeErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     //#[non_exhaustive] case
//     #[tvfrr_500_internal_server_error]
//     UnexpectedCase {
//         #[eo_display_with_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //todo - no parameters case?
// }

// // /////////////
// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence::ErrorOccurence,
//     from_sqlx_postgres_error::FromSqlxPostgresError,
//     type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
// )]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
//     (),
//     tvfrr_200_ok
// )]
// pub enum TryUpdateOne {
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorNotEqual {
//         #[eo_display_with_serialize_deserialize]
//         project_commit_not_equal: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorToStrConversion {
//         #[eo_display]
//         project_commit_to_str_conversion: http::header::ToStrError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoProjectCommitExtractorHeader {
//         #[eo_display_with_serialize_deserialize]
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_500_internal_server_error]
//     Configuration {
//         #[eo_display_with_serialize_deserialize]
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Database {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Io {
//         #[eo_display]
//         io_error: std::io::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Tls {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Protocol {
//         #[eo_display_with_serialize_deserialize]
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_404_not_found]
//     RowNotFound {
//         #[eo_display_with_serialize_deserialize]
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     TypeNotFound {
//         #[eo_display_with_serialize_deserialize]
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnIndexOutOfBounds {
//         #[eo_display_with_serialize_deserialize]
//         column_index_out_of_bounds: usize,
//         #[eo_display_with_serialize_deserialize]
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ColumnNotFound {
//         #[eo_display_with_serialize_deserialize]
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnDecode {
//         #[eo_display_with_serialize_deserialize]
//         column_decode_index: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Decode {
//         #[eo_display_with_serialize_deserialize]
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_408_request_timeout]
//     PoolTimedOut {
//         #[eo_display_with_serialize_deserialize]
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PoolClosed {
//         #[eo_display_with_serialize_deserialize]
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     WorkerCrashed {
//         #[eo_display_with_serialize_deserialize]
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Migrate {
//         #[eo_display]
//         migrate: sqlx::migrate::MigrateError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     JsonDataError {
//         #[eo_display_with_serialize_deserialize]
//         json_data_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     JsonSyntaxError {
//         #[eo_display_with_serialize_deserialize]
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     MissingJsonContentType {
//         #[eo_display_with_serialize_deserialize]
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     BytesRejection {
//         #[eo_display_with_serialize_deserialize]
//         bytes_rejection: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     FailedToDeserializePathParams {
//         #[eo_display_with_serialize_deserialize]
//         failed_to_deserialize_path_params: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     MissingPathParams {
//         #[eo_display_with_serialize_deserialize]
//         missing_path_params: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_500_internal_server_error]
//     BindQuery {
//         #[eo_error_occurence]
//         checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoPayloadFields {
//         #[eo_display_with_serialize_deserialize]
//         no_payload_fields: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize {
//         #[eo_error_occurence]
//         update_one_path_try_from_update_one_path_with_serialize_deserialize:
//             UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //#[non_exhaustive] case
//     #[tvfrr_500_internal_server_error]
//     UnexpectedCase {
//         #[eo_display_with_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// // /////////
// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence::ErrorOccurence,
//     from_sqlx_postgres_error::FromSqlxPostgresError,
//     type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
// )]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
//     (),
//     tvfrr_200_ok
// )]
// pub enum TryUpdateMany {
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorNotEqual {
//         #[eo_display_with_serialize_deserialize]
//         project_commit_not_equal: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ProjectCommitExtractorToStrConversion {
//         #[eo_display]
//         project_commit_to_str_conversion: http::header::ToStrError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoProjectCommitExtractorHeader {
//         #[eo_display_with_serialize_deserialize]
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_500_internal_server_error]
//     Configuration {
//         #[eo_display_with_serialize_deserialize]
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Database {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Io {
//         #[eo_display]
//         io_error: std::io::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Tls {
//         #[eo_display_with_serialize_deserialize]
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Protocol {
//         #[eo_display_with_serialize_deserialize]
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_404_not_found]
//     RowNotFound {
//         #[eo_display_with_serialize_deserialize]
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     TypeNotFound {
//         #[eo_display_with_serialize_deserialize]
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnIndexOutOfBounds {
//         #[eo_display_with_serialize_deserialize]
//         column_index_out_of_bounds: usize,
//         #[eo_display_with_serialize_deserialize]
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     ColumnNotFound {
//         #[eo_display_with_serialize_deserialize]
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     ColumnDecode {
//         #[eo_display_with_serialize_deserialize]
//         column_decode_index: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Decode {
//         #[eo_display_with_serialize_deserialize]
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_408_request_timeout]
//     PoolTimedOut {
//         #[eo_display_with_serialize_deserialize]
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PoolClosed {
//         #[eo_display_with_serialize_deserialize]
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     WorkerCrashed {
//         #[eo_display_with_serialize_deserialize]
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     Migrate {
//         #[eo_display]
//         migrate: sqlx::migrate::MigrateError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     JsonDataError {
//         #[eo_display_with_serialize_deserialize]
//         json_data_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     JsonSyntaxError {
//         #[eo_display_with_serialize_deserialize]
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     MissingJsonContentType {
//         #[eo_display_with_serialize_deserialize]
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     BytesRejection {
//         #[eo_display_with_serialize_deserialize]
//         bytes_rejection: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //
//     #[tvfrr_400_bad_request]
//     NotUniquePrimaryKey {
//         #[eo_vec_display]
//         not_unique_primary_keys: Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     BindQuery {
//         #[eo_error_occurence]
//         checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     CheckedAdd {
//         #[eo_display_with_serialize_deserialize]
//         checked_add: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NoPayloadFields {
//         #[eo_display_with_serialize_deserialize]
//         no_payload_fields: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     CommitFailed {
//         #[eo_display]
//         commit_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     NonExistingPrimaryKeys {
//         #[eo_vec_display]
//         non_existing_primary_keys: Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     PrimaryKeyFromRowAndFailedRollback {
//         #[eo_display]
//         primary_key_from_row: sqlx::Error,
//         #[eo_display]
//         rollback_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     //todo what status code should return if non_existing_primary_keys = 400, but transaction rollback failed = 500
//     NonExistingPrimaryKeysAndFailedRollback {
//         #[eo_vec_display]
//         non_existing_primary_keys: Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
//         #[eo_display]
//         rollback_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_500_internal_server_error]
//     QueryAndRollbackFailed {
//         #[eo_display]
//         query_error: sqlx::Error,
//         #[eo_display]
//         rollback_error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize {
//         #[eo_error_occurence]
//         update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize: UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //#[non_exhaustive] case
//     #[tvfrr_500_internal_server_error]
//     UnexpectedCase {
//         #[eo_display_with_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// //////
// ////////////////////////////////////////////////////////////////
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// pub enum TryCreateManyResponseVariants {
//     Desirable(std :: vec :: Vec :: < crate :: server :: postgres ::
//     uuid_wrapper :: PossibleUuidWrapper >), ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, RowNotFound
//     {
//         row_not_found : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolTimedOut
//     {
//         pool_timed_out : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, JsonDataError
//     {
//         json_data_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, JsonSyntaxError
//     {
//         json_syntax_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, MissingJsonContentType
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, BytesRejection
//     {
//         bytes_rejection : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//     {
//         uuid_wrapper_try_from_possible_uuid_wrapper_in_server : std :: string
//         :: String, code_occurence : crate :: common :: code_occurence ::
//         CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryCreateMany> for TryCreateManyResponseVariants {
//     fn from(val: TryCreateMany) -> Self {
//         match val.into_serialize_deserialize_version()
//         {
//             TryCreateManyWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryCreateManyWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryCreateManyWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence } => Self ::
//             Configuration { configuration_box_dyn_error, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence } => Self :: RowNotFound
//             { row_not_found, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence } => Self :: PoolTimedOut
//             { pool_timed_out, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence } => Self :: WorkerCrashed
//             { worker_crashed, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence }, TryCreateManyWithSerializeDeserialize
//             :: JsonDataError { json_data_error, code_occurence } => Self ::
//             JsonDataError { json_data_error, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: MissingJsonContentType
//             { json_syntax_error, code_occurence } => Self ::
//             MissingJsonContentType { json_syntax_error, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             TryCreateManyWithSerializeDeserialize :: BindQuery
//             { checked_add, code_occurence } => Self :: BindQuery
//             { checked_add, code_occurence },
//             TryCreateManyWithSerializeDeserialize ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
//                 code_occurence
//             } => Self ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
//                 code_occurence
//             }, TryCreateManyWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std::convert::From<&TryCreateManyResponseVariants> for http::StatusCode {
//     fn from(value: &TryCreateManyResponseVariants) -> Self {
//         match value
//         {
//             TryCreateManyResponseVariants :: Desirable(_) => http ::
//             StatusCode :: CREATED, TryCreateManyResponseVariants ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryCreateManyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryCreateManyResponseVariants
//             :: NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryCreateManyResponseVariants ::
//             Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryCreateManyResponseVariants
//             :: Database { box_dyn_database_error : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryCreateManyResponseVariants :: Io
//             { io_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryCreateManyResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryCreateManyResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryCreateManyResponseVariants ::
//             RowNotFound { row_not_found : _, code_occurence : _ } => http ::
//             StatusCode :: NOT_FOUND, TryCreateManyResponseVariants ::
//             TypeNotFound { type_not_found : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryCreateManyResponseVariants ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryCreateManyResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryCreateManyResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryCreateManyResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryCreateManyResponseVariants
//             :: PoolTimedOut { pool_timed_out : _, code_occurence : _ } => http
//             :: StatusCode :: REQUEST_TIMEOUT, TryCreateManyResponseVariants ::
//             PoolClosed { pool_closed : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryCreateManyResponseVariants
//             :: WorkerCrashed { worker_crashed : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryCreateManyResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryCreateManyResponseVariants ::
//             JsonDataError { json_data_error : _, code_occurence : _ } => http
//             :: StatusCode :: BAD_REQUEST, TryCreateManyResponseVariants ::
//             JsonSyntaxError { json_syntax_error : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryCreateManyResponseVariants
//             :: MissingJsonContentType
//             { json_syntax_error : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryCreateManyResponseVariants ::
//             BytesRejection { bytes_rejection : _, code_occurence : _ } => http
//             :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryCreateManyResponseVariants :: BindQuery
//             { checked_add : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryCreateManyResponseVariants ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server : _,
//                 code_occurence : _
//             } => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryCreateManyResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryCreateManyResponseVariantsTvfrr400BadRequest {
//     ProjectCommitExtractorNotEqual {
//         project_commit_not_equal: std::string::String,
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ProjectCommitExtractorToStrConversion {
//         project_commit_to_str_conversion: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     NoProjectCommitExtractorHeader {
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     TypeNotFound {
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnNotFound {
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     JsonDataError {
//         json_data_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     JsonSyntaxError {
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     MissingJsonContentType {
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryCreateManyResponseVariantsTvfrr400BadRequest>
//     for TryCreateManyResponseVariants
// {
//     fn from(value: TryCreateManyResponseVariantsTvfrr400BadRequest) -> Self {
//         match value
//         {
//             TryCreateManyResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryCreateManyResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryCreateManyResponseVariantsTvfrr400BadRequest ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryCreateManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryCreateManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryCreateManyResponseVariantsTvfrr400BadRequest :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             TryCreateManyResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             TryCreateManyResponseVariantsTvfrr400BadRequest ::
//             MissingJsonContentType { json_syntax_error, code_occurence } =>
//             Self :: MissingJsonContentType
//             { json_syntax_error, code_occurence }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryCreateManyResponseVariantsTvfrr500InternalServerError {
//     Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, BytesRejection
//     {
//         bytes_rejection : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//     {
//         uuid_wrapper_try_from_possible_uuid_wrapper_in_server : std :: string
//         :: String, code_occurence : crate :: common :: code_occurence ::
//         CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryCreateManyResponseVariantsTvfrr500InternalServerError>
//     for TryCreateManyResponseVariants
// {
//     fn from(value: TryCreateManyResponseVariantsTvfrr500InternalServerError) -> Self {
//         match value
//         {
//             TryCreateManyResponseVariantsTvfrr500InternalServerError ::
//             Configuration { configuration_box_dyn_error, code_occurence } =>
//             Self :: Configuration
//             { configuration_box_dyn_error, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError ::
//             Database { box_dyn_database_error, code_occurence } => Self ::
//             Database { box_dyn_database_error, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError ::
//             Protocol { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError ::
//             PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError ::
//             WorkerCrashed { worker_crashed, code_occurence } => Self ::
//             WorkerCrashed { worker_crashed, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError ::
//             Migrate { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError ::
//             BytesRejection { bytes_rejection, code_occurence } => Self ::
//             BytesRejection { bytes_rejection, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError ::
//             BindQuery { checked_add, code_occurence } => Self :: BindQuery
//             { checked_add, code_occurence },
//             TryCreateManyResponseVariantsTvfrr500InternalServerError ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
//                 code_occurence
//             } => Self ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
//                 code_occurence
//             }, TryCreateManyResponseVariantsTvfrr500InternalServerError ::
//             UnexpectedCase { unexpected_case, code_occurence } => Self ::
//             UnexpectedCase { unexpected_case, code_occurence }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryCreateManyResponseVariantsTvfrr408RequestTimeout {
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryCreateManyResponseVariantsTvfrr408RequestTimeout>
//     for TryCreateManyResponseVariants
// {
//     fn from(value: TryCreateManyResponseVariantsTvfrr408RequestTimeout) -> Self {
//         match value {
//             TryCreateManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryCreateManyResponseVariantsTvfrr404NotFound {
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryCreateManyResponseVariantsTvfrr404NotFound>
//     for TryCreateManyResponseVariants
// {
//     fn from(value: TryCreateManyResponseVariantsTvfrr404NotFound) -> Self {
//         match value {
//             TryCreateManyResponseVariantsTvfrr404NotFound::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryCreateManyResponseVariantsTvfrr201Created {
//     Desirable(std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>),
// }
// impl std::convert::From<TryCreateManyResponseVariantsTvfrr201Created>
//     for TryCreateManyResponseVariants
// {
//     fn from(value: TryCreateManyResponseVariantsTvfrr201Created) -> Self {
//         match value {
//             TryCreateManyResponseVariantsTvfrr201Created::Desirable(i) => Self::Desirable(i),
//         }
//     }
// }
// async fn try_from_response_try_create_many(
//     response: reqwest::Response,
// ) -> Result<
//     TryCreateManyResponseVariants,
//     crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
// > {
//     let status_code = response.status();
//     let headers = response.headers().clone();
//     if status_code == http::StatusCode::CREATED {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryCreateManyResponseVariantsTvfrr201Created > (& response_text)
//             {
//                 Ok(value) => Ok(TryCreateManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::BAD_REQUEST {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryCreateManyResponseVariantsTvfrr400BadRequest >
//             (& response_text)
//             {
//                 Ok(value) => Ok(TryCreateManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryCreateManyResponseVariantsTvfrr500InternalServerError >
//             (& response_text)
//             {
//                 Ok(value) => Ok(TryCreateManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::NOT_FOUND {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryCreateManyResponseVariantsTvfrr404NotFound > (& response_text)
//             {
//                 Ok(value) => Ok(TryCreateManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else {
//         match response.text().await
//         {
//             Ok(response_text) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ResponseText(response_text)
//             },), Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ReqwestError(e),
//             },),
//         }
//     }
// }
// impl TryFrom<TryCreateManyResponseVariants>
//     for std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>
// {
//     type Error = TryCreateManyWithSerializeDeserialize;
//     fn try_from(value: TryCreateManyResponseVariants) -> Result<Self, Self::Error> {
//         match value
//         {
//             TryCreateManyResponseVariants :: Desirable(i) => Ok(i),
//             TryCreateManyResponseVariants :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } =>
//             Err(TryCreateManyWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }), TryCreateManyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence }),
//             TryCreateManyResponseVariants :: NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence }),
//             TryCreateManyResponseVariants :: Configuration
//             { configuration_box_dyn_error, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence }),
//             TryCreateManyResponseVariants :: Database
//             { box_dyn_database_error, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence }),
//             TryCreateManyResponseVariants :: Io { io_error, code_occurence }
//             =>
//             Err(TryCreateManyWithSerializeDeserialize :: Io
//             { io_error, code_occurence }), TryCreateManyResponseVariants ::
//             Tls { box_dyn_error, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence }), TryCreateManyResponseVariants
//             :: Protocol { protocol, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence }), TryCreateManyResponseVariants ::
//             RowNotFound { row_not_found, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence }), TryCreateManyResponseVariants
//             :: TypeNotFound { type_not_found, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence }), TryCreateManyResponseVariants
//             :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence }),
//             TryCreateManyResponseVariants :: ColumnNotFound
//             { column_not_found, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence }),
//             TryCreateManyResponseVariants :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence }),
//             TryCreateManyResponseVariants :: Decode
//             { decode_box_dyn_error, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence }),
//             TryCreateManyResponseVariants :: PoolTimedOut
//             { pool_timed_out, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence }), TryCreateManyResponseVariants
//             :: PoolClosed { pool_closed, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence }), TryCreateManyResponseVariants ::
//             WorkerCrashed { worker_crashed, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence }), TryCreateManyResponseVariants
//             :: Migrate { migrate, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence }), TryCreateManyResponseVariants ::
//             JsonDataError { json_data_error, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence }),
//             TryCreateManyResponseVariants :: JsonSyntaxError
//             { json_syntax_error, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence }),
//             TryCreateManyResponseVariants :: MissingJsonContentType
//             { json_syntax_error, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize ::
//             MissingJsonContentType { json_syntax_error, code_occurence }),
//             TryCreateManyResponseVariants :: BytesRejection
//             { bytes_rejection, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence }),
//             TryCreateManyResponseVariants :: BindQuery
//             { checked_add, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: BindQuery
//             { checked_add, code_occurence }), TryCreateManyResponseVariants ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
//                 code_occurence
//             } =>
//             Err(TryCreateManyWithSerializeDeserialize ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
//                 code_occurence
//             }), TryCreateManyResponseVariants :: UnexpectedCase
//             { unexpected_case, code_occurence } =>
//             Err(TryCreateManyWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence })
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
// pub enum TryCreateManyRequestError {
//     ExpectedType {
//         #[eo_display_with_serialize_deserialize]
//         expected_type: TryCreateManyWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedStatusCode {
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_foreign_type]
//         response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToGetResponseText {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     DeserializeResponse {
//         #[eo_display]
//         serde: serde_json::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Reqwest {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// async fn tvfrr_extraction_logic_try_create_many<'a>(
//     future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
// ) -> Result<
//     std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>,
//     TryCreateManyRequestError,
// > {
//     match future.await
//     {
//         Ok(response) => match
//         try_from_response_try_create_many(response).await
//         {
//             Ok(variants) => match std :: vec :: Vec :: < crate :: server ::
//             postgres :: uuid_wrapper :: PossibleUuidWrapper > ::
//             try_from(variants)
//             {
//                 Ok(value) => Ok(value), Err(e) =>
//                 Err(TryCreateManyRequestError :: ExpectedType
//                 {
//                     expected_type : e, code_occurence : crate ::
//                     code_occurence_tufa_common! (),
//                 }),
//             }, Err(e) => match e
//             {
//                 crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: StatusCode
//                 { status_code, headers, response_text_result, } =>
//                 Err(TryCreateManyRequestError :: UnexpectedStatusCode
//                 {
//                     status_code, headers, response_text_result, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: FailedToGetResponseText
//                 { reqwest, status_code, headers } =>
//                 Err(TryCreateManyRequestError :: FailedToGetResponseText
//                 {
//                     reqwest, status_code, headers, code_occurence : crate ::
//                     code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde, status_code, headers, response_text, } =>
//                 Err(TryCreateManyRequestError :: DeserializeResponse
//                 {
//                     serde, status_code, headers, response_text, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }),
//             },
//         }, Err(e) =>
//         Err(TryCreateManyRequestError :: Reqwest
//         {
//             reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
//             (),
//         }),
//     }
// }
// pub enum TryCreateManyStatusCodesChecker {
//     ProjectCommitExtractorNotEqualTvfrr400BadRequest,
//     ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
//     NoProjectCommitExtractorHeaderTvfrr400BadRequest,
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
//     JsonDataErrorTvfrr400BadRequest,
//     JsonSyntaxErrorTvfrr400BadRequest,
//     MissingJsonContentTypeTvfrr400BadRequest,
//     BytesRejectionTvfrr500InternalServerError,
//     BindQueryTvfrr500InternalServerError,
//     CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl axum::response::IntoResponse for TryCreateManyResponseVariants {
//     fn into_response(self) -> axum::response::Response {
//         match & self
//         {
//             TryCreateManyResponseVariants :: Desirable(_) =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: CREATED ; res
//             } TryCreateManyResponseVariants :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateManyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateManyResponseVariants :: NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateManyResponseVariants :: Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: Database
//             { box_dyn_database_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: Io
//             { io_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: NOT_FOUND ; res
//             }, TryCreateManyResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateManyResponseVariants :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateManyResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: PoolTimedOut
//             { pool_timed_out : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: REQUEST_TIMEOUT ; res
//             }, TryCreateManyResponseVariants :: PoolClosed
//             { pool_closed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: WorkerCrashed
//             { worker_crashed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: JsonDataError
//             { json_data_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateManyResponseVariants :: JsonSyntaxError
//             { json_syntax_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateManyResponseVariants :: MissingJsonContentType
//             { json_syntax_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateManyResponseVariants :: BytesRejection
//             { bytes_rejection : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: BindQuery
//             { checked_add : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateManyResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }
//         }
//     }
// }
// //////////
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// pub enum TryCreateOneResponseVariants {
//     Desirable(crate::server::postgres::uuid_wrapper::PossibleUuidWrapper),
//     ProjectCommitExtractorNotEqual {
//         project_commit_not_equal: std::string::String,
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ProjectCommitExtractorToStrConversion {
//         project_commit_to_str_conversion: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     NoProjectCommitExtractorHeader {
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Configuration {
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Database {
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Io {
//         io_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Tls {
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Protocol {
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     TypeNotFound {
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnIndexOutOfBounds {
//         column_index_out_of_bounds: usize,
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnNotFound {
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnDecode {
//         column_decode_index: std::string::String,
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Decode {
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     PoolClosed {
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     WorkerCrashed {
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Migrate {
//         migrate: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     JsonDataError {
//         json_data_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     JsonSyntaxError {
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     MissingJsonContentType {
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     BytesRejection {
//         bytes_rejection: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
//         uuid_wrapper_try_from_possible_uuid_wrapper_in_server: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedCase {
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryCreateOne> for TryCreateOneResponseVariants {
//     fn from(val: TryCreateOne) -> Self {
//         match val.into_serialize_deserialize_version()
//         {
//             TryCreateOneWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryCreateOneWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryCreateOneWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence } => Self ::
//             Configuration { configuration_box_dyn_error, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence }, TryCreateOneWithSerializeDeserialize
//             :: Tls { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence }, TryCreateOneWithSerializeDeserialize
//             :: RowNotFound { row_not_found, code_occurence } => Self ::
//             RowNotFound { row_not_found, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence } => Self :: PoolTimedOut
//             { pool_timed_out, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence } => Self :: WorkerCrashed
//             { worker_crashed, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence }, TryCreateOneWithSerializeDeserialize
//             :: JsonDataError { json_data_error, code_occurence } => Self ::
//             JsonDataError { json_data_error, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: MissingJsonContentType
//             { json_syntax_error, code_occurence } => Self ::
//             MissingJsonContentType { json_syntax_error, code_occurence },
//             TryCreateOneWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             TryCreateOneWithSerializeDeserialize ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
//                 code_occurence
//             } => Self ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
//                 code_occurence
//             }, TryCreateOneWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std::convert::From<&TryCreateOneResponseVariants> for http::StatusCode {
//     fn from(value: &TryCreateOneResponseVariants) -> Self {
//         match value
//         {
//             TryCreateOneResponseVariants :: Desirable(_) => http :: StatusCode
//             :: CREATED, TryCreateOneResponseVariants ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryCreateOneResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryCreateOneResponseVariants ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryCreateOneResponseVariants ::
//             Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryCreateOneResponseVariants
//             :: Database { box_dyn_database_error : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryCreateOneResponseVariants :: Io
//             { io_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryCreateOneResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryCreateOneResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryCreateOneResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
//             NOT_FOUND, TryCreateOneResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryCreateOneResponseVariants ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryCreateOneResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryCreateOneResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryCreateOneResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryCreateOneResponseVariants
//             :: PoolTimedOut { pool_timed_out : _, code_occurence : _ } => http
//             :: StatusCode :: REQUEST_TIMEOUT, TryCreateOneResponseVariants ::
//             PoolClosed { pool_closed : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryCreateOneResponseVariants
//             :: WorkerCrashed { worker_crashed : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryCreateOneResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryCreateOneResponseVariants ::
//             JsonDataError { json_data_error : _, code_occurence : _ } => http
//             :: StatusCode :: BAD_REQUEST, TryCreateOneResponseVariants ::
//             JsonSyntaxError { json_syntax_error : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryCreateOneResponseVariants ::
//             MissingJsonContentType
//             { json_syntax_error : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryCreateOneResponseVariants ::
//             BytesRejection { bytes_rejection : _, code_occurence : _ } => http
//             :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryCreateOneResponseVariants ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server : _,
//                 code_occurence : _
//             } => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryCreateOneResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryCreateOneResponseVariantsTvfrr400BadRequest {
//     ProjectCommitExtractorNotEqual {
//         project_commit_not_equal: std::string::String,
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ProjectCommitExtractorToStrConversion {
//         project_commit_to_str_conversion: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     NoProjectCommitExtractorHeader {
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     TypeNotFound {
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnNotFound {
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     JsonDataError {
//         json_data_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     JsonSyntaxError {
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     MissingJsonContentType {
//         json_syntax_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryCreateOneResponseVariantsTvfrr400BadRequest>
//     for TryCreateOneResponseVariants
// {
//     fn from(value: TryCreateOneResponseVariantsTvfrr400BadRequest) -> Self {
//         match value
//         {
//             TryCreateOneResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryCreateOneResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryCreateOneResponseVariantsTvfrr400BadRequest ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryCreateOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryCreateOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryCreateOneResponseVariantsTvfrr400BadRequest :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             TryCreateOneResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             TryCreateOneResponseVariantsTvfrr400BadRequest ::
//             MissingJsonContentType { json_syntax_error, code_occurence } =>
//             Self :: MissingJsonContentType
//             { json_syntax_error, code_occurence }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryCreateOneResponseVariantsTvfrr500InternalServerError {
//     Configuration {
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Database {
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Io {
//         io_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Tls {
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Protocol {
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnIndexOutOfBounds {
//         column_index_out_of_bounds: usize,
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnDecode {
//         column_decode_index: std::string::String,
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Decode {
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     PoolClosed {
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     WorkerCrashed {
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Migrate {
//         migrate: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     BytesRejection {
//         bytes_rejection: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
//         uuid_wrapper_try_from_possible_uuid_wrapper_in_server: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedCase {
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryCreateOneResponseVariantsTvfrr500InternalServerError>
//     for TryCreateOneResponseVariants
// {
//     fn from(value: TryCreateOneResponseVariantsTvfrr500InternalServerError) -> Self {
//         match value
//         {
//             TryCreateOneResponseVariantsTvfrr500InternalServerError ::
//             Configuration { configuration_box_dyn_error, code_occurence } =>
//             Self :: Configuration
//             { configuration_box_dyn_error, code_occurence },
//             TryCreateOneResponseVariantsTvfrr500InternalServerError ::
//             Database { box_dyn_database_error, code_occurence } => Self ::
//             Database { box_dyn_database_error, code_occurence },
//             TryCreateOneResponseVariantsTvfrr500InternalServerError :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             TryCreateOneResponseVariantsTvfrr500InternalServerError :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryCreateOneResponseVariantsTvfrr500InternalServerError ::
//             Protocol { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             TryCreateOneResponseVariantsTvfrr500InternalServerError ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryCreateOneResponseVariantsTvfrr500InternalServerError ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryCreateOneResponseVariantsTvfrr500InternalServerError :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryCreateOneResponseVariantsTvfrr500InternalServerError ::
//             PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryCreateOneResponseVariantsTvfrr500InternalServerError ::
//             WorkerCrashed { worker_crashed, code_occurence } => Self ::
//             WorkerCrashed { worker_crashed, code_occurence },
//             TryCreateOneResponseVariantsTvfrr500InternalServerError :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             TryCreateOneResponseVariantsTvfrr500InternalServerError ::
//             BytesRejection { bytes_rejection, code_occurence } => Self ::
//             BytesRejection { bytes_rejection, code_occurence },
//             TryCreateOneResponseVariantsTvfrr500InternalServerError ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
//                 code_occurence
//             } => Self ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
//                 code_occurence
//             }, TryCreateOneResponseVariantsTvfrr500InternalServerError ::
//             UnexpectedCase { unexpected_case, code_occurence } => Self ::
//             UnexpectedCase { unexpected_case, code_occurence }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryCreateOneResponseVariantsTvfrr408RequestTimeout {
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryCreateOneResponseVariantsTvfrr408RequestTimeout>
//     for TryCreateOneResponseVariants
// {
//     fn from(value: TryCreateOneResponseVariantsTvfrr408RequestTimeout) -> Self {
//         match value {
//             TryCreateOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryCreateOneResponseVariantsTvfrr404NotFound {
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryCreateOneResponseVariantsTvfrr404NotFound>
//     for TryCreateOneResponseVariants
// {
//     fn from(value: TryCreateOneResponseVariantsTvfrr404NotFound) -> Self {
//         match value {
//             TryCreateOneResponseVariantsTvfrr404NotFound::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryCreateOneResponseVariantsTvfrr201Created {
//     Desirable(crate::server::postgres::uuid_wrapper::PossibleUuidWrapper),
// }
// impl std::convert::From<TryCreateOneResponseVariantsTvfrr201Created>
//     for TryCreateOneResponseVariants
// {
//     fn from(value: TryCreateOneResponseVariantsTvfrr201Created) -> Self {
//         match value {
//             TryCreateOneResponseVariantsTvfrr201Created::Desirable(i) => Self::Desirable(i),
//         }
//     }
// }
// async fn try_from_response_try_create_one(
//     response: reqwest::Response,
// ) -> Result<
//     TryCreateOneResponseVariants,
//     crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
// > {
//     let status_code = response.status();
//     let headers = response.headers().clone();
//     if status_code == http::StatusCode::CREATED {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryCreateOneResponseVariantsTvfrr201Created > (& response_text)
//             {
//                 Ok(value) => Ok(TryCreateOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::BAD_REQUEST {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryCreateOneResponseVariantsTvfrr400BadRequest > (& response_text)
//             {
//                 Ok(value) => Ok(TryCreateOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryCreateOneResponseVariantsTvfrr500InternalServerError >
//             (& response_text)
//             {
//                 Ok(value) => Ok(TryCreateOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::NOT_FOUND {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryCreateOneResponseVariantsTvfrr404NotFound > (& response_text)
//             {
//                 Ok(value) => Ok(TryCreateOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else {
//         match response.text().await
//         {
//             Ok(response_text) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ResponseText(response_text)
//             },), Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ReqwestError(e),
//             },),
//         }
//     }
// }
// impl TryFrom<TryCreateOneResponseVariants>
//     for crate::server::postgres::uuid_wrapper::PossibleUuidWrapper
// {
//     type Error = TryCreateOneWithSerializeDeserialize;
//     fn try_from(value: TryCreateOneResponseVariants) -> Result<Self, Self::Error> {
//         match value
//         {
//             TryCreateOneResponseVariants :: Desirable(i) => Ok(i),
//             TryCreateOneResponseVariants :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } =>
//             Err(TryCreateOneWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }), TryCreateOneResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence }),
//             TryCreateOneResponseVariants :: NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence }),
//             TryCreateOneResponseVariants :: Configuration
//             { configuration_box_dyn_error, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence }),
//             TryCreateOneResponseVariants :: Database
//             { box_dyn_database_error, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence }),
//             TryCreateOneResponseVariants :: Io { io_error, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: Io
//             { io_error, code_occurence }), TryCreateOneResponseVariants :: Tls
//             { box_dyn_error, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence }), TryCreateOneResponseVariants
//             :: Protocol { protocol, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence }), TryCreateOneResponseVariants ::
//             RowNotFound { row_not_found, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence }), TryCreateOneResponseVariants
//             :: TypeNotFound { type_not_found, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence }), TryCreateOneResponseVariants
//             :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence }),
//             TryCreateOneResponseVariants :: ColumnNotFound
//             { column_not_found, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence }),
//             TryCreateOneResponseVariants :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence }),
//             TryCreateOneResponseVariants :: Decode
//             { decode_box_dyn_error, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence }),
//             TryCreateOneResponseVariants :: PoolTimedOut
//             { pool_timed_out, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence }), TryCreateOneResponseVariants
//             :: PoolClosed { pool_closed, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence }), TryCreateOneResponseVariants ::
//             WorkerCrashed { worker_crashed, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence }), TryCreateOneResponseVariants
//             :: Migrate { migrate, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence }), TryCreateOneResponseVariants ::
//             JsonDataError { json_data_error, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence }), TryCreateOneResponseVariants
//             :: JsonSyntaxError { json_syntax_error, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence }),
//             TryCreateOneResponseVariants :: MissingJsonContentType
//             { json_syntax_error, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: MissingJsonContentType
//             { json_syntax_error, code_occurence }),
//             TryCreateOneResponseVariants :: BytesRejection
//             { bytes_rejection, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence }), TryCreateOneResponseVariants
//             ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
//                 code_occurence
//             } =>
//             Err(TryCreateOneWithSerializeDeserialize ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
//                 code_occurence
//             }), TryCreateOneResponseVariants :: UnexpectedCase
//             { unexpected_case, code_occurence } =>
//             Err(TryCreateOneWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence })
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
// pub enum TryCreateOneRequestError {
//     ExpectedType {
//         #[eo_display_with_serialize_deserialize]
//         expected_type: TryCreateOneWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedStatusCode {
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_foreign_type]
//         response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToGetResponseText {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     DeserializeResponse {
//         #[eo_display]
//         serde: serde_json::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Reqwest {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// async fn tvfrr_extraction_logic_try_create_one<'a>(
//     future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
// ) -> Result<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper, TryCreateOneRequestError> {
//     match future.await
//     {
//         Ok(response) => match try_from_response_try_create_one(response).await
//         {
//             Ok(variants) => match crate :: server :: postgres :: uuid_wrapper
//             :: PossibleUuidWrapper :: try_from(variants)
//             {
//                 Ok(value) => Ok(value), Err(e) =>
//                 Err(TryCreateOneRequestError :: ExpectedType
//                 {
//                     expected_type : e, code_occurence : crate ::
//                     code_occurence_tufa_common! (),
//                 }),
//             }, Err(e) => match e
//             {
//                 crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: StatusCode
//                 { status_code, headers, response_text_result, } =>
//                 Err(TryCreateOneRequestError :: UnexpectedStatusCode
//                 {
//                     status_code, headers, response_text_result, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: FailedToGetResponseText
//                 { reqwest, status_code, headers } =>
//                 Err(TryCreateOneRequestError :: FailedToGetResponseText
//                 {
//                     reqwest, status_code, headers, code_occurence : crate ::
//                     code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde, status_code, headers, response_text, } =>
//                 Err(TryCreateOneRequestError :: DeserializeResponse
//                 {
//                     serde, status_code, headers, response_text, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }),
//             },
//         }, Err(e) =>
//         Err(TryCreateOneRequestError :: Reqwest
//         {
//             reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
//             (),
//         }),
//     }
// }
// pub enum TryCreateOneStatusCodesChecker {
//     ProjectCommitExtractorNotEqualTvfrr400BadRequest,
//     ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
//     NoProjectCommitExtractorHeaderTvfrr400BadRequest,
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
//     JsonDataErrorTvfrr400BadRequest,
//     JsonSyntaxErrorTvfrr400BadRequest,
//     MissingJsonContentTypeTvfrr400BadRequest,
//     BytesRejectionTvfrr500InternalServerError,
//     CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl axum::response::IntoResponse for TryCreateOneResponseVariants {
//     fn into_response(self) -> axum::response::Response {
//         match & self
//         {
//             TryCreateOneResponseVariants :: Desirable(_) =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: CREATED ; res
//             } TryCreateOneResponseVariants :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateOneResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateOneResponseVariants :: NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateOneResponseVariants :: Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants :: Database
//             { box_dyn_database_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants :: Io
//             { io_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: NOT_FOUND ; res
//             }, TryCreateOneResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateOneResponseVariants :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateOneResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants :: PoolTimedOut
//             { pool_timed_out : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: REQUEST_TIMEOUT ; res
//             }, TryCreateOneResponseVariants :: PoolClosed
//             { pool_closed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants :: WorkerCrashed
//             { worker_crashed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants :: JsonDataError
//             { json_data_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateOneResponseVariants :: JsonSyntaxError
//             { json_syntax_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateOneResponseVariants :: MissingJsonContentType
//             { json_syntax_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryCreateOneResponseVariants :: BytesRejection
//             { bytes_rejection : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants ::
//             CreatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
//             {
//                 uuid_wrapper_try_from_possible_uuid_wrapper_in_server : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryCreateOneResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }
//         }
//     }
// }
// //////
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// pub enum TryDeleteOneResponseVariants {
//     Desirable(()), ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, RowNotFound
//     {
//         row_not_found : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolTimedOut
//     {
//         pool_timed_out : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, FailedToDeserializePathParams
//     {
//         failed_to_deserialize_path_params : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, MissingPathParams
//     {
//         missing_path_params : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
//     {
//         delete_one_path_try_from_delete_one_path_with_serialize_deserialize :
//         DeleteOnePathTryFromDeleteOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryDeleteOne> for TryDeleteOneResponseVariants {
//     fn from(val: TryDeleteOne) -> Self {
//         match val.into_serialize_deserialize_version()
//         {
//             TryDeleteOneWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryDeleteOneWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryDeleteOneWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence } => Self ::
//             Configuration { configuration_box_dyn_error, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence }, TryDeleteOneWithSerializeDeserialize
//             :: Tls { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence }, TryDeleteOneWithSerializeDeserialize
//             :: RowNotFound { row_not_found, code_occurence } => Self ::
//             RowNotFound { row_not_found, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence } => Self :: PoolTimedOut
//             { pool_timed_out, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence } => Self :: WorkerCrashed
//             { worker_crashed, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence }, TryDeleteOneWithSerializeDeserialize
//             :: FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence } => Self ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence },
//             TryDeleteOneWithSerializeDeserialize :: MissingPathParams
//             { missing_path_params, code_occurence } => Self ::
//             MissingPathParams { missing_path_params, code_occurence },
//             TryDeleteOneWithSerializeDeserialize ::
//             DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
//             {
//                 delete_one_path_try_from_delete_one_path_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
//             {
//                 delete_one_path_try_from_delete_one_path_with_serialize_deserialize,
//                 code_occurence
//             }, TryDeleteOneWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std::convert::From<&TryDeleteOneResponseVariants> for http::StatusCode {
//     fn from(value: &TryDeleteOneResponseVariants) -> Self {
//         match value
//         {
//             TryDeleteOneResponseVariants :: Desirable(_) => http :: StatusCode
//             :: OK, TryDeleteOneResponseVariants ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryDeleteOneResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryDeleteOneResponseVariants ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteOneResponseVariants ::
//             Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryDeleteOneResponseVariants
//             :: Database { box_dyn_database_error : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteOneResponseVariants :: Io
//             { io_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryDeleteOneResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryDeleteOneResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryDeleteOneResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
//             NOT_FOUND, TryDeleteOneResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryDeleteOneResponseVariants ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteOneResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryDeleteOneResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteOneResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryDeleteOneResponseVariants
//             :: PoolTimedOut { pool_timed_out : _, code_occurence : _ } => http
//             :: StatusCode :: REQUEST_TIMEOUT, TryDeleteOneResponseVariants ::
//             PoolClosed { pool_closed : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryDeleteOneResponseVariants
//             :: WorkerCrashed { worker_crashed : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteOneResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryDeleteOneResponseVariants ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryDeleteOneResponseVariants ::
//             MissingPathParams { missing_path_params : _, code_occurence : _ }
//             => http :: StatusCode :: BAD_REQUEST, TryDeleteOneResponseVariants
//             :: DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
//             {
//                 delete_one_path_try_from_delete_one_path_with_serialize_deserialize
//                 : _, code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryDeleteOneResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryDeleteOneResponseVariantsTvfrr500InternalServerError {
//     Configuration {
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Database {
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Io {
//         io_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Tls {
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Protocol {
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnIndexOutOfBounds {
//         column_index_out_of_bounds: usize,
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnDecode {
//         column_decode_index: std::string::String,
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Decode {
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     PoolClosed {
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     WorkerCrashed {
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Migrate {
//         migrate: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedCase {
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryDeleteOneResponseVariantsTvfrr500InternalServerError>
//     for TryDeleteOneResponseVariants
// {
//     fn from(value: TryDeleteOneResponseVariantsTvfrr500InternalServerError) -> Self {
//         match value {
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             } => Self::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             },
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             } => Self::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             },
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError::Io {
//                 io_error,
//                 code_occurence,
//             } => Self::Io {
//                 io_error,
//                 code_occurence,
//             },
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             } => Self::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             },
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError::Protocol {
//                 protocol,
//                 code_occurence,
//             } => Self::Protocol {
//                 protocol,
//                 code_occurence,
//             },
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             } => Self::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             },
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             } => Self::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             },
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             } => Self::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             },
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             } => Self::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             },
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             } => Self::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             },
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError::Migrate {
//                 migrate,
//                 code_occurence,
//             } => Self::Migrate {
//                 migrate,
//                 code_occurence,
//             },
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryDeleteOneResponseVariantsTvfrr400BadRequest {
//     ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, FailedToDeserializePathParams
//     {
//         failed_to_deserialize_path_params : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, MissingPathParams
//     {
//         missing_path_params : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
//     {
//         delete_one_path_try_from_delete_one_path_with_serialize_deserialize :
//         DeleteOnePathTryFromDeleteOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryDeleteOneResponseVariantsTvfrr400BadRequest>
//     for TryDeleteOneResponseVariants
// {
//     fn from(value: TryDeleteOneResponseVariantsTvfrr400BadRequest) -> Self {
//         match value
//         {
//             TryDeleteOneResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryDeleteOneResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryDeleteOneResponseVariantsTvfrr400BadRequest ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryDeleteOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryDeleteOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryDeleteOneResponseVariantsTvfrr400BadRequest ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence } => Self ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence },
//             TryDeleteOneResponseVariantsTvfrr400BadRequest ::
//             MissingPathParams { missing_path_params, code_occurence } => Self
//             :: MissingPathParams { missing_path_params, code_occurence },
//             TryDeleteOneResponseVariantsTvfrr400BadRequest ::
//             DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
//             {
//                 delete_one_path_try_from_delete_one_path_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
//             {
//                 delete_one_path_try_from_delete_one_path_with_serialize_deserialize,
//                 code_occurence
//             }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryDeleteOneResponseVariantsTvfrr408RequestTimeout {
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryDeleteOneResponseVariantsTvfrr408RequestTimeout>
//     for TryDeleteOneResponseVariants
// {
//     fn from(value: TryDeleteOneResponseVariantsTvfrr408RequestTimeout) -> Self {
//         match value {
//             TryDeleteOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryDeleteOneResponseVariantsTvfrr404NotFound {
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryDeleteOneResponseVariantsTvfrr404NotFound>
//     for TryDeleteOneResponseVariants
// {
//     fn from(value: TryDeleteOneResponseVariantsTvfrr404NotFound) -> Self {
//         match value {
//             TryDeleteOneResponseVariantsTvfrr404NotFound::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//         }
//     }
// }
// async fn try_from_response_try_delete_one(
//     response: reqwest::Response,
// ) -> Result<
//     TryDeleteOneResponseVariants,
//     crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
// > {
//     let status_code = response.status();
//     let headers = response.headers().clone();
//     if status_code == http::StatusCode::OK {
//         Ok(TryDeleteOneResponseVariants::Desirable(()))
//     } else if status_code == http::StatusCode::BAD_REQUEST {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryDeleteOneResponseVariantsTvfrr400BadRequest > (& response_text)
//             {
//                 Ok(value) => Ok(TryDeleteOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryDeleteOneResponseVariantsTvfrr500InternalServerError >
//             (& response_text)
//             {
//                 Ok(value) => Ok(TryDeleteOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::NOT_FOUND {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryDeleteOneResponseVariantsTvfrr404NotFound > (& response_text)
//             {
//                 Ok(value) => Ok(TryDeleteOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else {
//         match response.text().await
//         {
//             Ok(response_text) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ResponseText(response_text)
//             },), Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ReqwestError(e),
//             },),
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
// pub enum TryDeleteOneRequestError {
//     ExpectedType {
//         #[eo_display_with_serialize_deserialize]
//         expected_type: TryDeleteOneWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedStatusCode {
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_foreign_type]
//         response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToGetResponseText {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     DeserializeResponse {
//         #[eo_display]
//         serde: serde_json::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Reqwest {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// async fn tvfrr_extraction_logic_try_delete_one<'a>(
//     future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
// ) -> Result<(), TryDeleteOneRequestError> {
//     match future.await
//     {
//         Ok(response) => match try_from_response_try_delete_one(response).await
//         {
//             Ok(_variants) => Ok(()), Err(e) => match e
//             {
//                 crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: StatusCode
//                 { status_code, headers, response_text_result, } =>
//                 Err(TryDeleteOneRequestError :: UnexpectedStatusCode
//                 {
//                     status_code, headers, response_text_result, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: FailedToGetResponseText
//                 { reqwest, status_code, headers } =>
//                 Err(TryDeleteOneRequestError :: FailedToGetResponseText
//                 {
//                     reqwest, status_code, headers, code_occurence : crate ::
//                     code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde, status_code, headers, response_text, } =>
//                 Err(TryDeleteOneRequestError :: DeserializeResponse
//                 {
//                     serde, status_code, headers, response_text, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }),
//             },
//         }, Err(e) =>
//         Err(TryDeleteOneRequestError :: Reqwest
//         {
//             reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
//             (),
//         }),
//     }
// }
// pub enum TryDeleteOneStatusCodesChecker {
//     ProjectCommitExtractorNotEqualTvfrr400BadRequest,
//     ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
//     NoProjectCommitExtractorHeaderTvfrr400BadRequest,
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
//     FailedToDeserializePathParamsTvfrr400BadRequest,
//     MissingPathParamsTvfrr400BadRequest,
//     DeleteOnePathTryFromDeleteOnePathWithSerializeDeserializeTvfrr400BadRequest,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl axum::response::IntoResponse for TryDeleteOneResponseVariants {
//     fn into_response(self) -> axum::response::Response {
//         match & self
//         {
//             TryDeleteOneResponseVariants :: Desirable(_) =>
//             { http :: StatusCode :: OK.into_response() }
//             TryDeleteOneResponseVariants :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteOneResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteOneResponseVariants :: NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteOneResponseVariants :: Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteOneResponseVariants :: Database
//             { box_dyn_database_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteOneResponseVariants :: Io
//             { io_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteOneResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteOneResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteOneResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: NOT_FOUND ; res
//             }, TryDeleteOneResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteOneResponseVariants :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteOneResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteOneResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteOneResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteOneResponseVariants :: PoolTimedOut
//             { pool_timed_out : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: REQUEST_TIMEOUT ; res
//             }, TryDeleteOneResponseVariants :: PoolClosed
//             { pool_closed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteOneResponseVariants :: WorkerCrashed
//             { worker_crashed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteOneResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteOneResponseVariants :: FailedToDeserializePathParams
//             { failed_to_deserialize_path_params : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteOneResponseVariants :: MissingPathParams
//             { missing_path_params : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteOneResponseVariants ::
//             DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
//             {
//                 delete_one_path_try_from_delete_one_path_with_serialize_deserialize
//                 : _, code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteOneResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }
//         }
//     }
// }
// /////
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// pub enum TryDeleteManyWithBodyResponseVariants {
//     Desirable(()), ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, RowNotFound
//     {
//         row_not_found : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolTimedOut
//     {
//         pool_timed_out : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, JsonDataError
//     {
//         json_data_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, JsonSyntaxError
//     {
//         json_syntax_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, MissingJsonContentType
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, BytesRejection
//     {
//         bytes_rejection : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, NotUniquePrimaryKey
//     {
//         not_unique_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniqueNameVec
//     {
//         not_unique_name_vec : Vec < crate :: server :: postgres ::
//         regex_filter :: RegexFilter < >>, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, NotUniqueColorVec
//     {
//         not_unique_color_vec : Vec < crate :: server :: postgres ::
//         regex_filter :: RegexFilter < >>, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoPayloadFields
//     {
//         no_payload_fields : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, NoPayloadParameters
//     {
//         no_payload_parameters : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, NonExistingPrimaryKeys
//     {
//         non_existing_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NonExistingPrimaryKeysAndFailedRollback
//     {
//         non_existing_primary_keys : Vec < std :: string :: String >,
//         rollback_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, PrimaryKeyFromRowAndFailedRollback
//     {
//         primary_key_from_row : std :: string :: String, rollback_error : std
//         :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, CommitFailed
//     {
//         commit_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, QueryAndRollbackFailed
//     {
//         query_error : std :: string :: String, rollback_error : std :: string
//         :: String, code_occurence : crate :: common :: code_occurence ::
//         CodeOccurence
//     },
//     DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
//     {
//         delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize
//         :
//         DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryDeleteManyWithBody> for TryDeleteManyWithBodyResponseVariants {
//     fn from(val: TryDeleteManyWithBody) -> Self {
//         match val.into_serialize_deserialize_version()
//         {
//             TryDeleteManyWithBodyWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryDeleteManyWithBodyWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence } => Self ::
//             Configuration { configuration_box_dyn_error, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence } => Self :: RowNotFound
//             { row_not_found, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence } => Self :: PoolTimedOut
//             { pool_timed_out, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence } => Self :: WorkerCrashed
//             { worker_crashed, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize ::
//             MissingJsonContentType { json_syntax_error, code_occurence } =>
//             Self :: MissingJsonContentType
//             { json_syntax_error, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize ::
//             NotUniquePrimaryKey { not_unique_primary_keys, code_occurence } =>
//             Self :: NotUniquePrimaryKey
//             { not_unique_primary_keys, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: NotUniqueNameVec
//             { not_unique_name_vec, code_occurence } => Self ::
//             NotUniqueNameVec { not_unique_name_vec, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: NotUniqueColorVec
//             { not_unique_color_vec, code_occurence } => Self ::
//             NotUniqueColorVec { not_unique_color_vec, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: BindQuery
//             { checked_add, code_occurence } => Self :: BindQuery
//             { checked_add, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: NoPayloadFields
//             { no_payload_fields, code_occurence } => Self :: NoPayloadFields
//             { no_payload_fields, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize ::
//             NoPayloadParameters { no_payload_parameters, code_occurence } =>
//             Self :: NoPayloadParameters
//             { no_payload_parameters, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize ::
//             NonExistingPrimaryKeys
//             { non_existing_primary_keys, code_occurence } => Self ::
//             NonExistingPrimaryKeys
//             { non_existing_primary_keys, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize ::
//             NonExistingPrimaryKeysAndFailedRollback
//             { non_existing_primary_keys, rollback_error, code_occurence } =>
//             Self :: NonExistingPrimaryKeysAndFailedRollback
//             { non_existing_primary_keys, rollback_error, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize ::
//             PrimaryKeyFromRowAndFailedRollback
//             { primary_key_from_row, rollback_error, code_occurence } => Self
//             :: PrimaryKeyFromRowAndFailedRollback
//             { primary_key_from_row, rollback_error, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize :: CommitFailed
//             { commit_error, code_occurence } => Self :: CommitFailed
//             { commit_error, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize ::
//             QueryAndRollbackFailed
//             { query_error, rollback_error, code_occurence } => Self ::
//             QueryAndRollbackFailed
//             { query_error, rollback_error, code_occurence },
//             TryDeleteManyWithBodyWithSerializeDeserialize ::
//             DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize,
//                 code_occurence
//             }, TryDeleteManyWithBodyWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std::convert::From<&TryDeleteManyWithBodyResponseVariants> for http::StatusCode {
//     fn from(value: &TryDeleteManyWithBodyResponseVariants) -> Self {
//         match value
//         {
//             TryDeleteManyWithBodyResponseVariants :: Desirable(_) => http ::
//             StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryDeleteManyWithBodyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST,
//             TryDeleteManyWithBodyResponseVariants ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyWithBodyResponseVariants
//             :: Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyWithBodyResponseVariants :: Database
//             { box_dyn_database_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyWithBodyResponseVariants :: Io
//             { io_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryDeleteManyWithBodyResponseVariants ::
//             Tls { box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyWithBodyResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryDeleteManyWithBodyResponseVariants ::
//             RowNotFound { row_not_found : _, code_occurence : _ } => http ::
//             StatusCode :: NOT_FOUND, TryDeleteManyWithBodyResponseVariants ::
//             TypeNotFound { type_not_found : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyWithBodyResponseVariants
//             :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyWithBodyResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryDeleteManyWithBodyResponseVariants ::
//             ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyWithBodyResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyWithBodyResponseVariants :: PoolTimedOut
//             { pool_timed_out : _, code_occurence : _ } => http :: StatusCode
//             :: REQUEST_TIMEOUT, TryDeleteManyWithBodyResponseVariants ::
//             PoolClosed { pool_closed : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyWithBodyResponseVariants :: WorkerCrashed
//             { worker_crashed : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR, TryDeleteManyWithBodyResponseVariants ::
//             Migrate { migrate : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR, TryDeleteManyWithBodyResponseVariants ::
//             JsonDataError { json_data_error : _, code_occurence : _ } => http
//             :: StatusCode :: BAD_REQUEST,
//             TryDeleteManyWithBodyResponseVariants :: JsonSyntaxError
//             { json_syntax_error : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyWithBodyResponseVariants
//             :: MissingJsonContentType
//             { json_syntax_error : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyWithBodyResponseVariants
//             :: BytesRejection { bytes_rejection : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyWithBodyResponseVariants :: NotUniquePrimaryKey
//             { not_unique_primary_keys : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyWithBodyResponseVariants
//             :: NotUniqueNameVec
//             { not_unique_name_vec : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyWithBodyResponseVariants
//             :: NotUniqueColorVec
//             { not_unique_color_vec : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyWithBodyResponseVariants
//             :: BindQuery { checked_add : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyWithBodyResponseVariants :: NoPayloadFields
//             { no_payload_fields : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyWithBodyResponseVariants
//             :: NoPayloadParameters
//             { no_payload_parameters : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyWithBodyResponseVariants
//             :: NonExistingPrimaryKeys
//             { non_existing_primary_keys : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyWithBodyResponseVariants
//             :: NonExistingPrimaryKeysAndFailedRollback
//             {
//                 non_existing_primary_keys : _, rollback_error : _,
//                 code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryDeleteManyWithBodyResponseVariants ::
//             PrimaryKeyFromRowAndFailedRollback
//             {
//                 primary_key_from_row : _, rollback_error : _, code_occurence :
//                 _
//             } => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyWithBodyResponseVariants :: CommitFailed
//             { commit_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryDeleteManyWithBodyResponseVariants ::
//             QueryAndRollbackFailed
//             { query_error : _, rollback_error : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyWithBodyResponseVariants ::
//             DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize
//                 : _, code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryDeleteManyWithBodyResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest {
//     ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, JsonDataError
//     {
//         json_data_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, JsonSyntaxError
//     {
//         json_syntax_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, MissingJsonContentType
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniquePrimaryKey
//     {
//         not_unique_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniqueNameVec
//     {
//         not_unique_name_vec : Vec < crate :: server :: postgres ::
//         regex_filter :: RegexFilter < >>, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, NotUniqueColorVec
//     {
//         not_unique_color_vec : Vec < crate :: server :: postgres ::
//         regex_filter :: RegexFilter < >>, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, NoPayloadFields
//     {
//         no_payload_fields : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, NoPayloadParameters
//     {
//         no_payload_parameters : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, NonExistingPrimaryKeys
//     {
//         non_existing_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NonExistingPrimaryKeysAndFailedRollback
//     {
//         non_existing_primary_keys : Vec < std :: string :: String >,
//         rollback_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     },
//     DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
//     {
//         delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize
//         :
//         DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest>
//     for TryDeleteManyWithBodyResponseVariants
// {
//     fn from(value: TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest) -> Self {
//         match value
//         {
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             TypeNotFound { type_not_found, code_occurence } => Self ::
//             TypeNotFound { type_not_found, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             ColumnNotFound { column_not_found, code_occurence } => Self ::
//             ColumnNotFound { column_not_found, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             JsonDataError { json_data_error, code_occurence } => Self ::
//             JsonDataError { json_data_error, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             JsonSyntaxError { json_syntax_error, code_occurence } => Self ::
//             JsonSyntaxError { json_syntax_error, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             MissingJsonContentType { json_syntax_error, code_occurence } =>
//             Self :: MissingJsonContentType
//             { json_syntax_error, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             NotUniquePrimaryKey { not_unique_primary_keys, code_occurence } =>
//             Self :: NotUniquePrimaryKey
//             { not_unique_primary_keys, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             NotUniqueNameVec { not_unique_name_vec, code_occurence } => Self
//             :: NotUniqueNameVec { not_unique_name_vec, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             NotUniqueColorVec { not_unique_color_vec, code_occurence } => Self
//             :: NotUniqueColorVec { not_unique_color_vec, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             NoPayloadFields { no_payload_fields, code_occurence } => Self ::
//             NoPayloadFields { no_payload_fields, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             NoPayloadParameters { no_payload_parameters, code_occurence } =>
//             Self :: NoPayloadParameters
//             { no_payload_parameters, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             NonExistingPrimaryKeys
//             { non_existing_primary_keys, code_occurence } => Self ::
//             NonExistingPrimaryKeys
//             { non_existing_primary_keys, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             NonExistingPrimaryKeysAndFailedRollback
//             { non_existing_primary_keys, rollback_error, code_occurence } =>
//             Self :: NonExistingPrimaryKeysAndFailedRollback
//             { non_existing_primary_keys, rollback_error, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize,
//                 code_occurence
//             }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError {
//     Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, BytesRejection
//     {
//         bytes_rejection : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, PrimaryKeyFromRowAndFailedRollback
//     {
//         primary_key_from_row : std :: string :: String, rollback_error : std
//         :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, CommitFailed
//     {
//         commit_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, QueryAndRollbackFailed
//     {
//         query_error : std :: string :: String, rollback_error : std :: string
//         :: String, code_occurence : crate :: common :: code_occurence ::
//         CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError>
//     for TryDeleteManyWithBodyResponseVariants
// {
//     fn from(value: TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError) -> Self {
//         match value
//         {
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: Configuration { configuration_box_dyn_error, code_occurence }
//             => Self :: Configuration
//             { configuration_box_dyn_error, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: Database { box_dyn_database_error, code_occurence } => Self ::
//             Database { box_dyn_database_error, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: Io { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: Tls { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: Protocol { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: Decode { decode_box_dyn_error, code_occurence } => Self ::
//             Decode { decode_box_dyn_error, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: PoolClosed { pool_closed, code_occurence } => Self ::
//             PoolClosed { pool_closed, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: WorkerCrashed { worker_crashed, code_occurence } => Self ::
//             WorkerCrashed { worker_crashed, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: Migrate { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: BytesRejection { bytes_rejection, code_occurence } => Self ::
//             BytesRejection { bytes_rejection, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: BindQuery { checked_add, code_occurence } => Self :: BindQuery
//             { checked_add, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: PrimaryKeyFromRowAndFailedRollback
//             { primary_key_from_row, rollback_error, code_occurence } => Self
//             :: PrimaryKeyFromRowAndFailedRollback
//             { primary_key_from_row, rollback_error, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: CommitFailed { commit_error, code_occurence } => Self ::
//             CommitFailed { commit_error, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: QueryAndRollbackFailed
//             { query_error, rollback_error, code_occurence } => Self ::
//             QueryAndRollbackFailed
//             { query_error, rollback_error, code_occurence },
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
//             :: UnexpectedCase { unexpected_case, code_occurence } => Self ::
//             UnexpectedCase { unexpected_case, code_occurence }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryDeleteManyWithBodyResponseVariantsTvfrr404NotFound {
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryDeleteManyWithBodyResponseVariantsTvfrr404NotFound>
//     for TryDeleteManyWithBodyResponseVariants
// {
//     fn from(value: TryDeleteManyWithBodyResponseVariantsTvfrr404NotFound) -> Self {
//         match value {
//             TryDeleteManyWithBodyResponseVariantsTvfrr404NotFound::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryDeleteManyWithBodyResponseVariantsTvfrr408RequestTimeout {
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryDeleteManyWithBodyResponseVariantsTvfrr408RequestTimeout>
//     for TryDeleteManyWithBodyResponseVariants
// {
//     fn from(value: TryDeleteManyWithBodyResponseVariantsTvfrr408RequestTimeout) -> Self {
//         match value {
//             TryDeleteManyWithBodyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//         }
//     }
// }
// async fn try_from_response_try_delete_many_with_body(
//     response: reqwest::Response,
// ) -> Result<
//     TryDeleteManyWithBodyResponseVariants,
//     crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
// > {
//     let status_code = response.status();
//     let headers = response.headers().clone();
//     if status_code == http::StatusCode::OK {
//         Ok(TryDeleteManyWithBodyResponseVariants::Desirable(()))
//     } else if status_code == http::StatusCode::BAD_REQUEST {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest >
//             (& response_text)
//             {
//                 Ok(value) =>
//                 Ok(TryDeleteManyWithBodyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError >
//             (& response_text)
//             {
//                 Ok(value) =>
//                 Ok(TryDeleteManyWithBodyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::NOT_FOUND {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryDeleteManyWithBodyResponseVariantsTvfrr404NotFound >
//             (& response_text)
//             {
//                 Ok(value) =>
//                 Ok(TryDeleteManyWithBodyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else {
//         match response.text().await
//         {
//             Ok(response_text) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ResponseText(response_text)
//             },), Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ReqwestError(e),
//             },),
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
// pub enum TryDeleteManyWithBodyRequestError {
//     ExpectedType {
//         #[eo_display_with_serialize_deserialize]
//         expected_type: TryDeleteManyWithBodyWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedStatusCode {
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_foreign_type]
//         response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToGetResponseText {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     DeserializeResponse {
//         #[eo_display]
//         serde: serde_json::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Reqwest {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// async fn tvfrr_extraction_logic_try_delete_many_with_body<'a>(
//     future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
// ) -> Result<(), TryDeleteManyWithBodyRequestError> {
//     match future.await
//     {
//         Ok(response) => match
//         try_from_response_try_delete_many_with_body(response).await
//         {
//             Ok(_variants) => Ok(()), Err(e) => match e
//             {
//                 crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: StatusCode
//                 { status_code, headers, response_text_result, } =>
//                 Err(TryDeleteManyWithBodyRequestError :: UnexpectedStatusCode
//                 {
//                     status_code, headers, response_text_result, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: FailedToGetResponseText
//                 { reqwest, status_code, headers } =>
//                 Err(TryDeleteManyWithBodyRequestError ::
//                 FailedToGetResponseText
//                 {
//                     reqwest, status_code, headers, code_occurence : crate ::
//                     code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde, status_code, headers, response_text, } =>
//                 Err(TryDeleteManyWithBodyRequestError :: DeserializeResponse
//                 {
//                     serde, status_code, headers, response_text, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }),
//             },
//         }, Err(e) =>
//         Err(TryDeleteManyWithBodyRequestError :: Reqwest
//         {
//             reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
//             (),
//         }),
//     }
// }
// pub enum TryDeleteManyWithBodyStatusCodesChecker {
//     ProjectCommitExtractorNotEqualTvfrr400BadRequest,
//     ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
//     NoProjectCommitExtractorHeaderTvfrr400BadRequest,
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
//     JsonDataErrorTvfrr400BadRequest,
//     JsonSyntaxErrorTvfrr400BadRequest,
//     MissingJsonContentTypeTvfrr400BadRequest,
//     BytesRejectionTvfrr500InternalServerError,
//     NotUniquePrimaryKeyTvfrr400BadRequest,
//     NotUniqueNameVecTvfrr400BadRequest,
//     NotUniqueColorVecTvfrr400BadRequest,
//     BindQueryTvfrr500InternalServerError,
//     NoPayloadFieldsTvfrr400BadRequest,
//     NoPayloadParametersTvfrr400BadRequest,
//     NonExistingPrimaryKeysTvfrr400BadRequest,
//     NonExistingPrimaryKeysAndFailedRollbackTvfrr400BadRequest,
//     PrimaryKeyFromRowAndFailedRollbackTvfrr500InternalServerError,
//     CommitFailedTvfrr500InternalServerError,
//     QueryAndRollbackFailedTvfrr500InternalServerError,
//     DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserializeTvfrr400BadRequest,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl axum::response::IntoResponse for TryDeleteManyWithBodyResponseVariants {
//     fn into_response(self) -> axum::response::Response {
//         match & self
//         {
//             TryDeleteManyWithBodyResponseVariants :: Desirable(_) =>
//             { http :: StatusCode :: OK.into_response() }
//             TryDeleteManyWithBodyResponseVariants ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants :: Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: Database
//             { box_dyn_database_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: Io
//             { io_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: NOT_FOUND ; res
//             }, TryDeleteManyWithBodyResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: PoolTimedOut
//             { pool_timed_out : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: REQUEST_TIMEOUT ; res
//             }, TryDeleteManyWithBodyResponseVariants :: PoolClosed
//             { pool_closed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: WorkerCrashed
//             { worker_crashed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: JsonDataError
//             { json_data_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants :: JsonSyntaxError
//             { json_syntax_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants :: MissingJsonContentType
//             { json_syntax_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants :: BytesRejection
//             { bytes_rejection : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: NotUniquePrimaryKey
//             { not_unique_primary_keys : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants :: NotUniqueNameVec
//             { not_unique_name_vec : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants :: NotUniqueColorVec
//             { not_unique_color_vec : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants :: BindQuery
//             { checked_add : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: NoPayloadFields
//             { no_payload_fields : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants :: NoPayloadParameters
//             { no_payload_parameters : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants :: NonExistingPrimaryKeys
//             { non_existing_primary_keys : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants ::
//             NonExistingPrimaryKeysAndFailedRollback
//             {
//                 non_existing_primary_keys : _, rollback_error : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants ::
//             PrimaryKeyFromRowAndFailedRollback
//             {
//                 primary_key_from_row : _, rollback_error : _, code_occurence :
//                 _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: CommitFailed
//             { commit_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants :: QueryAndRollbackFailed
//             { query_error : _, rollback_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyWithBodyResponseVariants ::
//             DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize
//                 : _, code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyWithBodyResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }
//         }
//     }
// }
// //////
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// pub enum TryDeleteManyResponseVariants {
//     Desirable(()), ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, RowNotFound
//     {
//         row_not_found : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolTimedOut
//     {
//         pool_timed_out : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, FailedToDeserializeQueryString
//     {
//         failed_to_deserialize_query_string : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniquePrimaryKey
//     {
//         not_unique_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniqueNameVec
//     {
//         not_unique_name_vec : Vec < std :: string :: String < >>,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniqueColorVec
//     {
//         not_unique_color_vec : Vec < std :: string :: String < >>,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoQueryParameters
//     {
//         no_query_parameters : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, CommitFailed
//     {
//         commit_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, NonExistingPrimaryKeys
//     {
//         non_existing_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, PrimaryKeyFromRowAndFailedRollback
//     {
//         primary_key_from_row : std :: string :: String, rollback_error : std
//         :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, NonExistingPrimaryKeysAndFailedRollback
//     {
//         non_existing_primary_keys : Vec < std :: string :: String >,
//         rollback_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, QueryAndRollbackFailed
//     {
//         query_error : std :: string :: String, rollback_error : std :: string
//         :: String, code_occurence : crate :: common :: code_occurence ::
//         CodeOccurence
//     }, DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserialize
//     {
//         delete_many_query_try_from_delete_many_query_with_serialize_deserialize
//         :
//         DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryDeleteMany> for TryDeleteManyResponseVariants {
//     fn from(val: TryDeleteMany) -> Self {
//         match val.into_serialize_deserialize_version()
//         {
//             TryDeleteManyWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryDeleteManyWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryDeleteManyWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence } => Self ::
//             Configuration { configuration_box_dyn_error, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence } => Self :: RowNotFound
//             { row_not_found, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence } => Self :: PoolTimedOut
//             { pool_timed_out, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence } => Self :: WorkerCrashed
//             { worker_crashed, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence }, TryDeleteManyWithSerializeDeserialize
//             :: FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence } => Self ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: NotUniquePrimaryKey
//             { not_unique_primary_keys, code_occurence } => Self ::
//             NotUniquePrimaryKey { not_unique_primary_keys, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: NotUniqueNameVec
//             { not_unique_name_vec, code_occurence } => Self ::
//             NotUniqueNameVec { not_unique_name_vec, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: NotUniqueColorVec
//             { not_unique_color_vec, code_occurence } => Self ::
//             NotUniqueColorVec { not_unique_color_vec, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: BindQuery
//             { checked_add, code_occurence } => Self :: BindQuery
//             { checked_add, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: NoQueryParameters
//             { no_query_parameters, code_occurence } => Self ::
//             NoQueryParameters { no_query_parameters, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: CommitFailed
//             { commit_error, code_occurence } => Self :: CommitFailed
//             { commit_error, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: NonExistingPrimaryKeys
//             { non_existing_primary_keys, code_occurence } => Self ::
//             NonExistingPrimaryKeys
//             { non_existing_primary_keys, code_occurence },
//             TryDeleteManyWithSerializeDeserialize ::
//             PrimaryKeyFromRowAndFailedRollback
//             { primary_key_from_row, rollback_error, code_occurence } => Self
//             :: PrimaryKeyFromRowAndFailedRollback
//             { primary_key_from_row, rollback_error, code_occurence },
//             TryDeleteManyWithSerializeDeserialize ::
//             NonExistingPrimaryKeysAndFailedRollback
//             { non_existing_primary_keys, rollback_error, code_occurence } =>
//             Self :: NonExistingPrimaryKeysAndFailedRollback
//             { non_existing_primary_keys, rollback_error, code_occurence },
//             TryDeleteManyWithSerializeDeserialize :: QueryAndRollbackFailed
//             { query_error, rollback_error, code_occurence } => Self ::
//             QueryAndRollbackFailed
//             { query_error, rollback_error, code_occurence },
//             TryDeleteManyWithSerializeDeserialize ::
//             DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserialize
//             {
//                 delete_many_query_try_from_delete_many_query_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserialize
//             {
//                 delete_many_query_try_from_delete_many_query_with_serialize_deserialize,
//                 code_occurence
//             }, TryDeleteManyWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std::convert::From<&TryDeleteManyResponseVariants> for http::StatusCode {
//     fn from(value: &TryDeleteManyResponseVariants) -> Self {
//         match value
//         {
//             TryDeleteManyResponseVariants :: Desirable(_) => http ::
//             StatusCode :: OK, TryDeleteManyResponseVariants ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryDeleteManyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryDeleteManyResponseVariants
//             :: NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyResponseVariants ::
//             Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryDeleteManyResponseVariants
//             :: Database { box_dyn_database_error : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyResponseVariants :: Io
//             { io_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryDeleteManyResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryDeleteManyResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryDeleteManyResponseVariants ::
//             RowNotFound { row_not_found : _, code_occurence : _ } => http ::
//             StatusCode :: NOT_FOUND, TryDeleteManyResponseVariants ::
//             TypeNotFound { type_not_found : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyResponseVariants ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryDeleteManyResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryDeleteManyResponseVariants
//             :: PoolTimedOut { pool_timed_out : _, code_occurence : _ } => http
//             :: StatusCode :: REQUEST_TIMEOUT, TryDeleteManyResponseVariants ::
//             PoolClosed { pool_closed : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryDeleteManyResponseVariants
//             :: WorkerCrashed { worker_crashed : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryDeleteManyResponseVariants ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryDeleteManyResponseVariants
//             :: NotUniquePrimaryKey
//             { not_unique_primary_keys : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyResponseVariants ::
//             NotUniqueNameVec { not_unique_name_vec : _, code_occurence : _ }
//             => http :: StatusCode :: BAD_REQUEST,
//             TryDeleteManyResponseVariants :: NotUniqueColorVec
//             { not_unique_color_vec : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyResponseVariants ::
//             BindQuery { checked_add : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryDeleteManyResponseVariants
//             :: NoQueryParameters
//             { no_query_parameters : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyResponseVariants ::
//             CommitFailed { commit_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryDeleteManyResponseVariants
//             :: NonExistingPrimaryKeys
//             { non_existing_primary_keys : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryDeleteManyResponseVariants ::
//             PrimaryKeyFromRowAndFailedRollback
//             {
//                 primary_key_from_row : _, rollback_error : _, code_occurence :
//                 _
//             } => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyResponseVariants ::
//             NonExistingPrimaryKeysAndFailedRollback
//             {
//                 non_existing_primary_keys : _, rollback_error : _,
//                 code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryDeleteManyResponseVariants :: QueryAndRollbackFailed
//             { query_error : _, rollback_error : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryDeleteManyResponseVariants ::
//             DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserialize
//             {
//                 delete_many_query_try_from_delete_many_query_with_serialize_deserialize
//                 : _, code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryDeleteManyResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryDeleteManyResponseVariantsTvfrr404NotFound {
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryDeleteManyResponseVariantsTvfrr404NotFound>
//     for TryDeleteManyResponseVariants
// {
//     fn from(value: TryDeleteManyResponseVariantsTvfrr404NotFound) -> Self {
//         match value {
//             TryDeleteManyResponseVariantsTvfrr404NotFound::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryDeleteManyResponseVariantsTvfrr400BadRequest {
//     ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, FailedToDeserializeQueryString
//     {
//         failed_to_deserialize_query_string : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniquePrimaryKey
//     {
//         not_unique_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniqueNameVec
//     {
//         not_unique_name_vec : Vec < std :: string :: String < >>,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniqueColorVec
//     {
//         not_unique_color_vec : Vec < std :: string :: String < >>,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoQueryParameters
//     {
//         no_query_parameters : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, NonExistingPrimaryKeys
//     {
//         non_existing_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NonExistingPrimaryKeysAndFailedRollback
//     {
//         non_existing_primary_keys : Vec < std :: string :: String >,
//         rollback_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserialize
//     {
//         delete_many_query_try_from_delete_many_query_with_serialize_deserialize
//         :
//         DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryDeleteManyResponseVariantsTvfrr400BadRequest>
//     for TryDeleteManyResponseVariants
// {
//     fn from(value: TryDeleteManyResponseVariantsTvfrr400BadRequest) -> Self {
//         match value
//         {
//             TryDeleteManyResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryDeleteManyResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr400BadRequest ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr400BadRequest ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence } => Self ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr400BadRequest ::
//             NotUniquePrimaryKey { not_unique_primary_keys, code_occurence } =>
//             Self :: NotUniquePrimaryKey
//             { not_unique_primary_keys, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr400BadRequest ::
//             NotUniqueNameVec { not_unique_name_vec, code_occurence } => Self
//             :: NotUniqueNameVec { not_unique_name_vec, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr400BadRequest ::
//             NotUniqueColorVec { not_unique_color_vec, code_occurence } => Self
//             :: NotUniqueColorVec { not_unique_color_vec, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr400BadRequest ::
//             NoQueryParameters { no_query_parameters, code_occurence } => Self
//             :: NoQueryParameters { no_query_parameters, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr400BadRequest ::
//             NonExistingPrimaryKeys
//             { non_existing_primary_keys, code_occurence } => Self ::
//             NonExistingPrimaryKeys
//             { non_existing_primary_keys, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr400BadRequest ::
//             NonExistingPrimaryKeysAndFailedRollback
//             { non_existing_primary_keys, rollback_error, code_occurence } =>
//             Self :: NonExistingPrimaryKeysAndFailedRollback
//             { non_existing_primary_keys, rollback_error, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr400BadRequest ::
//             DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserialize
//             {
//                 delete_many_query_try_from_delete_many_query_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserialize
//             {
//                 delete_many_query_try_from_delete_many_query_with_serialize_deserialize,
//                 code_occurence
//             }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryDeleteManyResponseVariantsTvfrr408RequestTimeout {
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryDeleteManyResponseVariantsTvfrr408RequestTimeout>
//     for TryDeleteManyResponseVariants
// {
//     fn from(value: TryDeleteManyResponseVariantsTvfrr408RequestTimeout) -> Self {
//         match value {
//             TryDeleteManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryDeleteManyResponseVariantsTvfrr500InternalServerError {
//     Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, CommitFailed
//     {
//         commit_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, PrimaryKeyFromRowAndFailedRollback
//     {
//         primary_key_from_row : std :: string :: String, rollback_error : std
//         :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, QueryAndRollbackFailed
//     {
//         query_error : std :: string :: String, rollback_error : std :: string
//         :: String, code_occurence : crate :: common :: code_occurence ::
//         CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryDeleteManyResponseVariantsTvfrr500InternalServerError>
//     for TryDeleteManyResponseVariants
// {
//     fn from(value: TryDeleteManyResponseVariantsTvfrr500InternalServerError) -> Self {
//         match value
//         {
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             Configuration { configuration_box_dyn_error, code_occurence } =>
//             Self :: Configuration
//             { configuration_box_dyn_error, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             Database { box_dyn_database_error, code_occurence } => Self ::
//             Database { box_dyn_database_error, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             Protocol { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             WorkerCrashed { worker_crashed, code_occurence } => Self ::
//             WorkerCrashed { worker_crashed, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             Migrate { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             BindQuery { checked_add, code_occurence } => Self :: BindQuery
//             { checked_add, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             CommitFailed { commit_error, code_occurence } => Self ::
//             CommitFailed { commit_error, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             PrimaryKeyFromRowAndFailedRollback
//             { primary_key_from_row, rollback_error, code_occurence } => Self
//             :: PrimaryKeyFromRowAndFailedRollback
//             { primary_key_from_row, rollback_error, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             QueryAndRollbackFailed
//             { query_error, rollback_error, code_occurence } => Self ::
//             QueryAndRollbackFailed
//             { query_error, rollback_error, code_occurence },
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
//             UnexpectedCase { unexpected_case, code_occurence } => Self ::
//             UnexpectedCase { unexpected_case, code_occurence }
//         }
//     }
// }
// async fn try_from_response_try_delete_many(
//     response: reqwest::Response,
// ) -> Result<
//     TryDeleteManyResponseVariants,
//     crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
// > {
//     let status_code = response.status();
//     let headers = response.headers().clone();
//     if status_code == http::StatusCode::OK {
//         Ok(TryDeleteManyResponseVariants::Desirable(()))
//     } else if status_code == http::StatusCode::BAD_REQUEST {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryDeleteManyResponseVariantsTvfrr400BadRequest >
//             (& response_text)
//             {
//                 Ok(value) => Ok(TryDeleteManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryDeleteManyResponseVariantsTvfrr500InternalServerError >
//             (& response_text)
//             {
//                 Ok(value) => Ok(TryDeleteManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::NOT_FOUND {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryDeleteManyResponseVariantsTvfrr404NotFound > (& response_text)
//             {
//                 Ok(value) => Ok(TryDeleteManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else {
//         match response.text().await
//         {
//             Ok(response_text) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ResponseText(response_text)
//             },), Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ReqwestError(e),
//             },),
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
// pub enum TryDeleteManyRequestError {
//     ExpectedType {
//         #[eo_display_with_serialize_deserialize]
//         expected_type: TryDeleteManyWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedStatusCode {
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_foreign_type]
//         response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToGetResponseText {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     DeserializeResponse {
//         #[eo_display]
//         serde: serde_json::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Reqwest {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// async fn tvfrr_extraction_logic_try_delete_many<'a>(
//     future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
// ) -> Result<(), TryDeleteManyRequestError> {
//     match future.await
//     {
//         Ok(response) => match
//         try_from_response_try_delete_many(response).await
//         {
//             Ok(_variants) => Ok(()), Err(e) => match e
//             {
//                 crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: StatusCode
//                 { status_code, headers, response_text_result, } =>
//                 Err(TryDeleteManyRequestError :: UnexpectedStatusCode
//                 {
//                     status_code, headers, response_text_result, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: FailedToGetResponseText
//                 { reqwest, status_code, headers } =>
//                 Err(TryDeleteManyRequestError :: FailedToGetResponseText
//                 {
//                     reqwest, status_code, headers, code_occurence : crate ::
//                     code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde, status_code, headers, response_text, } =>
//                 Err(TryDeleteManyRequestError :: DeserializeResponse
//                 {
//                     serde, status_code, headers, response_text, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }),
//             },
//         }, Err(e) =>
//         Err(TryDeleteManyRequestError :: Reqwest
//         {
//             reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
//             (),
//         }),
//     }
// }
// pub enum TryDeleteManyStatusCodesChecker {
//     ProjectCommitExtractorNotEqualTvfrr400BadRequest,
//     ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
//     NoProjectCommitExtractorHeaderTvfrr400BadRequest,
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
//     FailedToDeserializeQueryStringTvfrr400BadRequest,
//     NotUniquePrimaryKeyTvfrr400BadRequest,
//     NotUniqueNameVecTvfrr400BadRequest,
//     NotUniqueColorVecTvfrr400BadRequest,
//     BindQueryTvfrr500InternalServerError,
//     NoQueryParametersTvfrr400BadRequest,
//     CommitFailedTvfrr500InternalServerError,
//     NonExistingPrimaryKeysTvfrr400BadRequest,
//     PrimaryKeyFromRowAndFailedRollbackTvfrr500InternalServerError,
//     NonExistingPrimaryKeysAndFailedRollbackTvfrr400BadRequest,
//     QueryAndRollbackFailedTvfrr500InternalServerError,
//     DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserializeTvfrr400BadRequest,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl axum::response::IntoResponse for TryDeleteManyResponseVariants {
//     fn into_response(self) -> axum::response::Response {
//         match & self
//         {
//             TryDeleteManyResponseVariants :: Desirable(_) =>
//             { http :: StatusCode :: OK.into_response() }
//             TryDeleteManyResponseVariants :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants :: NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants :: Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: Database
//             { box_dyn_database_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: Io
//             { io_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: NOT_FOUND ; res
//             }, TryDeleteManyResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: PoolTimedOut
//             { pool_timed_out : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: REQUEST_TIMEOUT ; res
//             }, TryDeleteManyResponseVariants :: PoolClosed
//             { pool_closed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: WorkerCrashed
//             { worker_crashed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants :: NotUniquePrimaryKey
//             { not_unique_primary_keys : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants :: NotUniqueNameVec
//             { not_unique_name_vec : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants :: NotUniqueColorVec
//             { not_unique_color_vec : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants :: BindQuery
//             { checked_add : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: NoQueryParameters
//             { no_query_parameters : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants :: CommitFailed
//             { commit_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants :: NonExistingPrimaryKeys
//             { non_existing_primary_keys : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants ::
//             PrimaryKeyFromRowAndFailedRollback
//             {
//                 primary_key_from_row : _, rollback_error : _, code_occurence :
//                 _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants ::
//             NonExistingPrimaryKeysAndFailedRollback
//             {
//                 non_existing_primary_keys : _, rollback_error : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants :: QueryAndRollbackFailed
//             { query_error : _, rollback_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryDeleteManyResponseVariants ::
//             DeleteManyQueryTryFromDeleteManyQueryWithSerializeDeserialize
//             {
//                 delete_many_query_try_from_delete_many_query_with_serialize_deserialize
//                 : _, code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryDeleteManyResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }
//         }
//     }
// }
// /////////
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// pub enum TryReadOneResponseVariants {
//     Desirable(crate::repositories_types::tufa_server::routes::api::cats::DogOptions),
//     ProjectCommitExtractorNotEqual {
//         project_commit_not_equal: std::string::String,
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ProjectCommitExtractorToStrConversion {
//         project_commit_to_str_conversion: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     NoProjectCommitExtractorHeader {
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Configuration {
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Database {
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Io {
//         io_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Tls {
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Protocol {
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     TypeNotFound {
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnIndexOutOfBounds {
//         column_index_out_of_bounds: usize,
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnNotFound {
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnDecode {
//         column_decode_index: std::string::String,
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Decode {
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     PoolClosed {
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     WorkerCrashed {
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Migrate {
//         migrate: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToDeserializePathParams {
//         failed_to_deserialize_path_params: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     MissingPathParams {
//         missing_path_params: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToDeserializeQueryString {
//         failed_to_deserialize_query_string: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ReadOnePathTryFromReadOnePathWithSerializeDeserialize {
//         read_one_path_try_from_read_one_path_with_serialize_deserialize:
//             ReadOnePathTryFromReadOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedCase {
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryReadOne> for TryReadOneResponseVariants {
//     fn from(val: TryReadOne) -> Self {
//         match val.into_serialize_deserialize_version()
//         {
//             TryReadOneWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryReadOneWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryReadOneWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryReadOneWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence } => Self ::
//             Configuration { configuration_box_dyn_error, code_occurence },
//             TryReadOneWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             TryReadOneWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence }, TryReadOneWithSerializeDeserialize
//             :: Tls { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryReadOneWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence }, TryReadOneWithSerializeDeserialize
//             :: RowNotFound { row_not_found, code_occurence } => Self ::
//             RowNotFound { row_not_found, code_occurence },
//             TryReadOneWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryReadOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryReadOneWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryReadOneWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryReadOneWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryReadOneWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence } => Self :: PoolTimedOut
//             { pool_timed_out, code_occurence },
//             TryReadOneWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryReadOneWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence } => Self :: WorkerCrashed
//             { worker_crashed, code_occurence },
//             TryReadOneWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence }, TryReadOneWithSerializeDeserialize ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence } => Self ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence },
//             TryReadOneWithSerializeDeserialize :: MissingPathParams
//             { missing_path_params, code_occurence } => Self ::
//             MissingPathParams { missing_path_params, code_occurence },
//             TryReadOneWithSerializeDeserialize ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence } => Self ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence },
//             TryReadOneWithSerializeDeserialize ::
//             ReadOnePathTryFromReadOnePathWithSerializeDeserialize
//             {
//                 read_one_path_try_from_read_one_path_with_serialize_deserialize,
//                 code_occurence
//             } => Self :: ReadOnePathTryFromReadOnePathWithSerializeDeserialize
//             {
//                 read_one_path_try_from_read_one_path_with_serialize_deserialize,
//                 code_occurence
//             }, TryReadOneWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std::convert::From<&TryReadOneResponseVariants> for http::StatusCode {
//     fn from(value: &TryReadOneResponseVariants) -> Self {
//         match value {
//             TryReadOneResponseVariants::Desirable(_) => http::StatusCode::OK,
//             TryReadOneResponseVariants::ProjectCommitExtractorNotEqual {
//                 project_commit_not_equal: _,
//                 project_commit_to_use: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryReadOneResponseVariants::ProjectCommitExtractorToStrConversion {
//                 project_commit_to_str_conversion: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryReadOneResponseVariants::NoProjectCommitExtractorHeader {
//                 no_project_commit_header: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryReadOneResponseVariants::Configuration {
//                 configuration_box_dyn_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryReadOneResponseVariants::Database {
//                 box_dyn_database_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryReadOneResponseVariants::Io {
//                 io_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryReadOneResponseVariants::Tls {
//                 box_dyn_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryReadOneResponseVariants::Protocol {
//                 protocol: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryReadOneResponseVariants::RowNotFound {
//                 row_not_found: _,
//                 code_occurence: _,
//             } => http::StatusCode::NOT_FOUND,
//             TryReadOneResponseVariants::TypeNotFound {
//                 type_not_found: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryReadOneResponseVariants::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds: _,
//                 len: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryReadOneResponseVariants::ColumnNotFound {
//                 column_not_found: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryReadOneResponseVariants::ColumnDecode {
//                 column_decode_index: _,
//                 source_handle: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryReadOneResponseVariants::Decode {
//                 decode_box_dyn_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryReadOneResponseVariants::PoolTimedOut {
//                 pool_timed_out: _,
//                 code_occurence: _,
//             } => http::StatusCode::REQUEST_TIMEOUT,
//             TryReadOneResponseVariants::PoolClosed {
//                 pool_closed: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryReadOneResponseVariants::WorkerCrashed {
//                 worker_crashed: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryReadOneResponseVariants::Migrate {
//                 migrate: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryReadOneResponseVariants::FailedToDeserializePathParams {
//                 failed_to_deserialize_path_params: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryReadOneResponseVariants::MissingPathParams {
//                 missing_path_params: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryReadOneResponseVariants::FailedToDeserializeQueryString {
//                 failed_to_deserialize_query_string: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryReadOneResponseVariants::ReadOnePathTryFromReadOnePathWithSerializeDeserialize {
//                 read_one_path_try_from_read_one_path_with_serialize_deserialize: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryReadOneResponseVariants::UnexpectedCase {
//                 unexpected_case: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadOneResponseVariantsTvfrr408RequestTimeout {
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryReadOneResponseVariantsTvfrr408RequestTimeout>
//     for TryReadOneResponseVariants
// {
//     fn from(value: TryReadOneResponseVariantsTvfrr408RequestTimeout) -> Self {
//         match value {
//             TryReadOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadOneResponseVariantsTvfrr404NotFound {
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryReadOneResponseVariantsTvfrr404NotFound> for TryReadOneResponseVariants {
//     fn from(value: TryReadOneResponseVariantsTvfrr404NotFound) -> Self {
//         match value {
//             TryReadOneResponseVariantsTvfrr404NotFound::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadOneResponseVariantsTvfrr500InternalServerError {
//     Configuration {
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Database {
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Io {
//         io_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Tls {
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Protocol {
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnIndexOutOfBounds {
//         column_index_out_of_bounds: usize,
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnDecode {
//         column_decode_index: std::string::String,
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Decode {
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     PoolClosed {
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     WorkerCrashed {
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Migrate {
//         migrate: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedCase {
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryReadOneResponseVariantsTvfrr500InternalServerError>
//     for TryReadOneResponseVariants
// {
//     fn from(value: TryReadOneResponseVariantsTvfrr500InternalServerError) -> Self {
//         match value {
//             TryReadOneResponseVariantsTvfrr500InternalServerError::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             } => Self::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             },
//             TryReadOneResponseVariantsTvfrr500InternalServerError::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             } => Self::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             },
//             TryReadOneResponseVariantsTvfrr500InternalServerError::Io {
//                 io_error,
//                 code_occurence,
//             } => Self::Io {
//                 io_error,
//                 code_occurence,
//             },
//             TryReadOneResponseVariantsTvfrr500InternalServerError::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             } => Self::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             },
//             TryReadOneResponseVariantsTvfrr500InternalServerError::Protocol {
//                 protocol,
//                 code_occurence,
//             } => Self::Protocol {
//                 protocol,
//                 code_occurence,
//             },
//             TryReadOneResponseVariantsTvfrr500InternalServerError::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             } => Self::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             },
//             TryReadOneResponseVariantsTvfrr500InternalServerError::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             } => Self::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             },
//             TryReadOneResponseVariantsTvfrr500InternalServerError::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             } => Self::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             },
//             TryReadOneResponseVariantsTvfrr500InternalServerError::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             } => Self::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             },
//             TryReadOneResponseVariantsTvfrr500InternalServerError::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             } => Self::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             },
//             TryReadOneResponseVariantsTvfrr500InternalServerError::Migrate {
//                 migrate,
//                 code_occurence,
//             } => Self::Migrate {
//                 migrate,
//                 code_occurence,
//             },
//             TryReadOneResponseVariantsTvfrr500InternalServerError::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadOneResponseVariantsTvfrr400BadRequest {
//     ProjectCommitExtractorNotEqual {
//         project_commit_not_equal: std::string::String,
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ProjectCommitExtractorToStrConversion {
//         project_commit_to_str_conversion: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     NoProjectCommitExtractorHeader {
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     TypeNotFound {
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ColumnNotFound {
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToDeserializePathParams {
//         failed_to_deserialize_path_params: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     MissingPathParams {
//         missing_path_params: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToDeserializeQueryString {
//         failed_to_deserialize_query_string: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     ReadOnePathTryFromReadOnePathWithSerializeDeserialize {
//         read_one_path_try_from_read_one_path_with_serialize_deserialize:
//             ReadOnePathTryFromReadOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryReadOneResponseVariantsTvfrr400BadRequest>
//     for TryReadOneResponseVariants
// {
//     fn from(value: TryReadOneResponseVariantsTvfrr400BadRequest) -> Self {
//         match value
//         {
//             TryReadOneResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryReadOneResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryReadOneResponseVariantsTvfrr400BadRequest ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryReadOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryReadOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryReadOneResponseVariantsTvfrr400BadRequest ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence } => Self ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence },
//             TryReadOneResponseVariantsTvfrr400BadRequest :: MissingPathParams
//             { missing_path_params, code_occurence } => Self ::
//             MissingPathParams { missing_path_params, code_occurence },
//             TryReadOneResponseVariantsTvfrr400BadRequest ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence } => Self ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence },
//             TryReadOneResponseVariantsTvfrr400BadRequest ::
//             ReadOnePathTryFromReadOnePathWithSerializeDeserialize
//             {
//                 read_one_path_try_from_read_one_path_with_serialize_deserialize,
//                 code_occurence
//             } => Self :: ReadOnePathTryFromReadOnePathWithSerializeDeserialize
//             {
//                 read_one_path_try_from_read_one_path_with_serialize_deserialize,
//                 code_occurence
//             }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadOneResponseVariantsTvfrr200Ok {
//     Desirable(crate::repositories_types::tufa_server::routes::api::cats::DogOptions),
// }
// impl std::convert::From<TryReadOneResponseVariantsTvfrr200Ok> for TryReadOneResponseVariants {
//     fn from(value: TryReadOneResponseVariantsTvfrr200Ok) -> Self {
//         match value {
//             TryReadOneResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
//         }
//     }
// }
// async fn try_from_response_try_read_one(
//     response: reqwest::Response,
// ) -> Result<
//     TryReadOneResponseVariants,
//     crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
// > {
//     let status_code = response.status();
//     let headers = response.headers().clone();
//     if status_code == http::StatusCode::OK {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryReadOneResponseVariantsTvfrr200Ok > (& response_text)
//             {
//                 Ok(value) => Ok(TryReadOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::BAD_REQUEST {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryReadOneResponseVariantsTvfrr400BadRequest > (& response_text)
//             {
//                 Ok(value) => Ok(TryReadOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryReadOneResponseVariantsTvfrr500InternalServerError >
//             (& response_text)
//             {
//                 Ok(value) => Ok(TryReadOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::NOT_FOUND {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryReadOneResponseVariantsTvfrr404NotFound > (& response_text)
//             {
//                 Ok(value) => Ok(TryReadOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else {
//         match response.text().await
//         {
//             Ok(response_text) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ResponseText(response_text)
//             },), Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ReqwestError(e),
//             },),
//         }
//     }
// }
// impl TryFrom<TryReadOneResponseVariants>
//     for crate::repositories_types::tufa_server::routes::api::cats::DogOptions
// {
//     type Error = TryReadOneWithSerializeDeserialize;
//     fn try_from(value: TryReadOneResponseVariants) -> Result<Self, Self::Error> {
//         match value
//         {
//             TryReadOneResponseVariants :: Desirable(i) => Ok(i),
//             TryReadOneResponseVariants :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } =>
//             Err(TryReadOneWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }), TryReadOneResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence }),
//             TryReadOneResponseVariants :: NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence }),
//             TryReadOneResponseVariants :: Configuration
//             { configuration_box_dyn_error, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence }),
//             TryReadOneResponseVariants :: Database
//             { box_dyn_database_error, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence }),
//             TryReadOneResponseVariants :: Io { io_error, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: Io
//             { io_error, code_occurence }), TryReadOneResponseVariants :: Tls
//             { box_dyn_error, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence }), TryReadOneResponseVariants ::
//             Protocol { protocol, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence }), TryReadOneResponseVariants ::
//             RowNotFound { row_not_found, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence }), TryReadOneResponseVariants ::
//             TypeNotFound { type_not_found, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence }), TryReadOneResponseVariants ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence }),
//             TryReadOneResponseVariants :: ColumnNotFound
//             { column_not_found, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence }), TryReadOneResponseVariants
//             :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence }),
//             TryReadOneResponseVariants :: Decode
//             { decode_box_dyn_error, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence }),
//             TryReadOneResponseVariants :: PoolTimedOut
//             { pool_timed_out, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence }), TryReadOneResponseVariants ::
//             PoolClosed { pool_closed, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence }), TryReadOneResponseVariants ::
//             WorkerCrashed { worker_crashed, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence }), TryReadOneResponseVariants ::
//             Migrate { migrate, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence }), TryReadOneResponseVariants ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence }),
//             TryReadOneResponseVariants :: MissingPathParams
//             { missing_path_params, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: MissingPathParams
//             { missing_path_params, code_occurence }),
//             TryReadOneResponseVariants :: FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence }),
//             TryReadOneResponseVariants ::
//             ReadOnePathTryFromReadOnePathWithSerializeDeserialize
//             {
//                 read_one_path_try_from_read_one_path_with_serialize_deserialize,
//                 code_occurence
//             } =>
//             Err(TryReadOneWithSerializeDeserialize ::
//             ReadOnePathTryFromReadOnePathWithSerializeDeserialize
//             {
//                 read_one_path_try_from_read_one_path_with_serialize_deserialize,
//                 code_occurence
//             }), TryReadOneResponseVariants :: UnexpectedCase
//             { unexpected_case, code_occurence } =>
//             Err(TryReadOneWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence })
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
// pub enum TryReadOneRequestError {
//     ExpectedType {
//         #[eo_display_with_serialize_deserialize]
//         expected_type: TryReadOneWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedStatusCode {
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_foreign_type]
//         response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToGetResponseText {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     DeserializeResponse {
//         #[eo_display]
//         serde: serde_json::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Reqwest {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// async fn tvfrr_extraction_logic_try_read_one<'a>(
//     future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
// ) -> Result<
//     crate::repositories_types::tufa_server::routes::api::cats::DogOptions,
//     TryReadOneRequestError,
// > {
//     match future.await
//     {
//         Ok(response) => match try_from_response_try_read_one(response).await
//         {
//             Ok(variants) => match crate :: repositories_types :: tufa_server
//             :: routes :: api :: cats :: DogOptions :: try_from(variants)
//             {
//                 Ok(value) => Ok(value), Err(e) =>
//                 Err(TryReadOneRequestError :: ExpectedType
//                 {
//                     expected_type : e, code_occurence : crate ::
//                     code_occurence_tufa_common! (),
//                 }),
//             }, Err(e) => match e
//             {
//                 crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: StatusCode
//                 { status_code, headers, response_text_result, } =>
//                 Err(TryReadOneRequestError :: UnexpectedStatusCode
//                 {
//                     status_code, headers, response_text_result, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: FailedToGetResponseText
//                 { reqwest, status_code, headers } =>
//                 Err(TryReadOneRequestError :: FailedToGetResponseText
//                 {
//                     reqwest, status_code, headers, code_occurence : crate ::
//                     code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde, status_code, headers, response_text, } =>
//                 Err(TryReadOneRequestError :: DeserializeResponse
//                 {
//                     serde, status_code, headers, response_text, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }),
//             },
//         }, Err(e) =>
//         Err(TryReadOneRequestError :: Reqwest
//         {
//             reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
//             (),
//         }),
//     }
// }
// pub enum TryReadOneStatusCodesChecker {
//     ProjectCommitExtractorNotEqualTvfrr400BadRequest,
//     ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
//     NoProjectCommitExtractorHeaderTvfrr400BadRequest,
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
//     FailedToDeserializePathParamsTvfrr400BadRequest,
//     MissingPathParamsTvfrr400BadRequest,
//     FailedToDeserializeQueryStringTvfrr400BadRequest,
//     ReadOnePathTryFromReadOnePathWithSerializeDeserializeTvfrr400BadRequest,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl axum::response::IntoResponse for TryReadOneResponseVariants {
//     fn into_response(self) -> axum::response::Response {
//         match &self {
//             TryReadOneResponseVariants::Desirable(_) => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::OK;
//                 res
//             }
//             TryReadOneResponseVariants::ProjectCommitExtractorNotEqual {
//                 project_commit_not_equal: _,
//                 project_commit_to_use: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::BAD_REQUEST;
//                 res
//             }
//             TryReadOneResponseVariants::ProjectCommitExtractorToStrConversion {
//                 project_commit_to_str_conversion: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::BAD_REQUEST;
//                 res
//             }
//             TryReadOneResponseVariants::NoProjectCommitExtractorHeader {
//                 no_project_commit_header: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::BAD_REQUEST;
//                 res
//             }
//             TryReadOneResponseVariants::Configuration {
//                 configuration_box_dyn_error: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
//                 res
//             }
//             TryReadOneResponseVariants::Database {
//                 box_dyn_database_error: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
//                 res
//             }
//             TryReadOneResponseVariants::Io {
//                 io_error: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
//                 res
//             }
//             TryReadOneResponseVariants::Tls {
//                 box_dyn_error: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
//                 res
//             }
//             TryReadOneResponseVariants::Protocol {
//                 protocol: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
//                 res
//             }
//             TryReadOneResponseVariants::RowNotFound {
//                 row_not_found: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::NOT_FOUND;
//                 res
//             }
//             TryReadOneResponseVariants::TypeNotFound {
//                 type_not_found: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::BAD_REQUEST;
//                 res
//             }
//             TryReadOneResponseVariants::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds: _,
//                 len: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
//                 res
//             }
//             TryReadOneResponseVariants::ColumnNotFound {
//                 column_not_found: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::BAD_REQUEST;
//                 res
//             }
//             TryReadOneResponseVariants::ColumnDecode {
//                 column_decode_index: _,
//                 source_handle: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
//                 res
//             }
//             TryReadOneResponseVariants::Decode {
//                 decode_box_dyn_error: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
//                 res
//             }
//             TryReadOneResponseVariants::PoolTimedOut {
//                 pool_timed_out: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::REQUEST_TIMEOUT;
//                 res
//             }
//             TryReadOneResponseVariants::PoolClosed {
//                 pool_closed: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
//                 res
//             }
//             TryReadOneResponseVariants::WorkerCrashed {
//                 worker_crashed: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
//                 res
//             }
//             TryReadOneResponseVariants::Migrate {
//                 migrate: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
//                 res
//             }
//             TryReadOneResponseVariants::FailedToDeserializePathParams {
//                 failed_to_deserialize_path_params: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::BAD_REQUEST;
//                 res
//             }
//             TryReadOneResponseVariants::MissingPathParams {
//                 missing_path_params: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::BAD_REQUEST;
//                 res
//             }
//             TryReadOneResponseVariants::FailedToDeserializeQueryString {
//                 failed_to_deserialize_query_string: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::BAD_REQUEST;
//                 res
//             }
//             TryReadOneResponseVariants::ReadOnePathTryFromReadOnePathWithSerializeDeserialize {
//                 read_one_path_try_from_read_one_path_with_serialize_deserialize: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::BAD_REQUEST;
//                 res
//             }
//             TryReadOneResponseVariants::UnexpectedCase {
//                 unexpected_case: _,
//                 code_occurence: _,
//             } => {
//                 let mut res = axum::Json(self).into_response();
//                 *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
//                 res
//             }
//         }
//     }
// }
// /////////////
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// pub enum TryReadManyWithBodyResponseVariants {
//     Desirable(Vec :: < crate :: repositories_types :: tufa_server :: routes ::
//     api :: cats :: DogOptions >), ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, RowNotFound
//     {
//         row_not_found : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolTimedOut
//     {
//         pool_timed_out : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, JsonDataError
//     {
//         json_data_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, JsonSyntaxError
//     {
//         json_syntax_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, MissingJsonContentType
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, BytesRejection
//     {
//         bytes_rejection : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, NotUniquePrimaryKey
//     {
//         not_unique_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniqueNameVec
//     {
//         not_unique_name_vec : Vec < crate :: server :: postgres ::
//         regex_filter :: RegexFilter < >>, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, NotUniqueColorVec
//     {
//         not_unique_color_vec : Vec < crate :: server :: postgres ::
//         regex_filter :: RegexFilter < >>, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUuid
//     {
//         not_uuid : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     },
//     ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
//     {
//         read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize
//         :
//         ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryReadManyWithBody> for TryReadManyWithBodyResponseVariants {
//     fn from(val: TryReadManyWithBody) -> Self {
//         match val.into_serialize_deserialize_version()
//         {
//             TryReadManyWithBodyWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryReadManyWithBodyWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence } => Self ::
//             Configuration { configuration_box_dyn_error, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence } => Self :: RowNotFound
//             { row_not_found, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence } => Self :: PoolTimedOut
//             { pool_timed_out, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence } => Self :: WorkerCrashed
//             { worker_crashed, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize ::
//             MissingJsonContentType { json_syntax_error, code_occurence } =>
//             Self :: MissingJsonContentType
//             { json_syntax_error, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: NotUniquePrimaryKey
//             { not_unique_primary_keys, code_occurence } => Self ::
//             NotUniquePrimaryKey { not_unique_primary_keys, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: NotUniqueNameVec
//             { not_unique_name_vec, code_occurence } => Self ::
//             NotUniqueNameVec { not_unique_name_vec, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: NotUniqueColorVec
//             { not_unique_color_vec, code_occurence } => Self ::
//             NotUniqueColorVec { not_unique_color_vec, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: BindQuery
//             { checked_add, code_occurence } => Self :: BindQuery
//             { checked_add, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize :: NotUuid
//             { not_uuid, code_occurence } => Self :: NotUuid
//             { not_uuid, code_occurence },
//             TryReadManyWithBodyWithSerializeDeserialize ::
//             ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize,
//                 code_occurence
//             }, TryReadManyWithBodyWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std::convert::From<&TryReadManyWithBodyResponseVariants> for http::StatusCode {
//     fn from(value: &TryReadManyWithBodyResponseVariants) -> Self {
//         match value
//         {
//             TryReadManyWithBodyResponseVariants :: Desirable(_) => http ::
//             StatusCode :: OK, TryReadManyWithBodyResponseVariants ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryReadManyWithBodyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST,
//             TryReadManyWithBodyResponseVariants ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryReadManyWithBodyResponseVariants ::
//             Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyWithBodyResponseVariants :: Database
//             { box_dyn_database_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyWithBodyResponseVariants :: Io
//             { io_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryReadManyWithBodyResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryReadManyWithBodyResponseVariants ::
//             Protocol { protocol : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyWithBodyResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
//             NOT_FOUND, TryReadManyWithBodyResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryReadManyWithBodyResponseVariants ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyWithBodyResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryReadManyWithBodyResponseVariants ::
//             ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyWithBodyResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyWithBodyResponseVariants :: PoolTimedOut
//             { pool_timed_out : _, code_occurence : _ } => http :: StatusCode
//             :: REQUEST_TIMEOUT, TryReadManyWithBodyResponseVariants ::
//             PoolClosed { pool_closed : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyWithBodyResponseVariants :: WorkerCrashed
//             { worker_crashed : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR, TryReadManyWithBodyResponseVariants ::
//             Migrate { migrate : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR, TryReadManyWithBodyResponseVariants ::
//             JsonDataError { json_data_error : _, code_occurence : _ } => http
//             :: StatusCode :: BAD_REQUEST, TryReadManyWithBodyResponseVariants
//             :: JsonSyntaxError { json_syntax_error : _, code_occurence : _ }
//             => http :: StatusCode :: BAD_REQUEST,
//             TryReadManyWithBodyResponseVariants :: MissingJsonContentType
//             { json_syntax_error : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryReadManyWithBodyResponseVariants ::
//             BytesRejection { bytes_rejection : _, code_occurence : _ } => http
//             :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyWithBodyResponseVariants :: NotUniquePrimaryKey
//             { not_unique_primary_keys : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryReadManyWithBodyResponseVariants ::
//             NotUniqueNameVec { not_unique_name_vec : _, code_occurence : _ }
//             => http :: StatusCode :: BAD_REQUEST,
//             TryReadManyWithBodyResponseVariants :: NotUniqueColorVec
//             { not_unique_color_vec : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryReadManyWithBodyResponseVariants ::
//             BindQuery { checked_add : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyWithBodyResponseVariants :: NotUuid
//             { not_uuid : _, code_occurence : _ } => http :: StatusCode ::
//             BAD_REQUEST, TryReadManyWithBodyResponseVariants ::
//             ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize
//                 : _, code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryReadManyWithBodyResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadManyWithBodyResponseVariantsTvfrr408RequestTimeout {
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryReadManyWithBodyResponseVariantsTvfrr408RequestTimeout>
//     for TryReadManyWithBodyResponseVariants
// {
//     fn from(value: TryReadManyWithBodyResponseVariantsTvfrr408RequestTimeout) -> Self {
//         match value {
//             TryReadManyWithBodyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadManyWithBodyResponseVariantsTvfrr404NotFound {
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryReadManyWithBodyResponseVariantsTvfrr404NotFound>
//     for TryReadManyWithBodyResponseVariants
// {
//     fn from(value: TryReadManyWithBodyResponseVariantsTvfrr404NotFound) -> Self {
//         match value {
//             TryReadManyWithBodyResponseVariantsTvfrr404NotFound::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError {
//     Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, BytesRejection
//     {
//         bytes_rejection : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError>
//     for TryReadManyWithBodyResponseVariants
// {
//     fn from(value: TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError) -> Self {
//         match value
//         {
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             Configuration { configuration_box_dyn_error, code_occurence } =>
//             Self :: Configuration
//             { configuration_box_dyn_error, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             Database { box_dyn_database_error, code_occurence } => Self ::
//             Database { box_dyn_database_error, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             Io { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             Tls { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             Protocol { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             Decode { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             WorkerCrashed { worker_crashed, code_occurence } => Self ::
//             WorkerCrashed { worker_crashed, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             Migrate { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             BytesRejection { bytes_rejection, code_occurence } => Self ::
//             BytesRejection { bytes_rejection, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             BindQuery { checked_add, code_occurence } => Self :: BindQuery
//             { checked_add, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
//             UnexpectedCase { unexpected_case, code_occurence } => Self ::
//             UnexpectedCase { unexpected_case, code_occurence }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadManyWithBodyResponseVariantsTvfrr400BadRequest {
//     ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, JsonDataError
//     {
//         json_data_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, JsonSyntaxError
//     {
//         json_syntax_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, MissingJsonContentType
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniquePrimaryKey
//     {
//         not_unique_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniqueNameVec
//     {
//         not_unique_name_vec : Vec < crate :: server :: postgres ::
//         regex_filter :: RegexFilter < >>, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, NotUniqueColorVec
//     {
//         not_unique_color_vec : Vec < crate :: server :: postgres ::
//         regex_filter :: RegexFilter < >>, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, NotUuid
//     {
//         not_uuid : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     },
//     ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
//     {
//         read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize
//         :
//         ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryReadManyWithBodyResponseVariantsTvfrr400BadRequest>
//     for TryReadManyWithBodyResponseVariants
// {
//     fn from(value: TryReadManyWithBodyResponseVariantsTvfrr400BadRequest) -> Self {
//         match value
//         {
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             TypeNotFound { type_not_found, code_occurence } => Self ::
//             TypeNotFound { type_not_found, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             ColumnNotFound { column_not_found, code_occurence } => Self ::
//             ColumnNotFound { column_not_found, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             JsonDataError { json_data_error, code_occurence } => Self ::
//             JsonDataError { json_data_error, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             JsonSyntaxError { json_syntax_error, code_occurence } => Self ::
//             JsonSyntaxError { json_syntax_error, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             MissingJsonContentType { json_syntax_error, code_occurence } =>
//             Self :: MissingJsonContentType
//             { json_syntax_error, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             NotUniquePrimaryKey { not_unique_primary_keys, code_occurence } =>
//             Self :: NotUniquePrimaryKey
//             { not_unique_primary_keys, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             NotUniqueNameVec { not_unique_name_vec, code_occurence } => Self
//             :: NotUniqueNameVec { not_unique_name_vec, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             NotUniqueColorVec { not_unique_color_vec, code_occurence } => Self
//             :: NotUniqueColorVec { not_unique_color_vec, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest :: NotUuid
//             { not_uuid, code_occurence } => Self :: NotUuid
//             { not_uuid, code_occurence },
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
//             ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize,
//                 code_occurence
//             }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadManyWithBodyResponseVariantsTvfrr200Ok {
//     Desirable(Vec<crate::repositories_types::tufa_server::routes::api::cats::DogOptions>),
// }
// impl std::convert::From<TryReadManyWithBodyResponseVariantsTvfrr200Ok>
//     for TryReadManyWithBodyResponseVariants
// {
//     fn from(value: TryReadManyWithBodyResponseVariantsTvfrr200Ok) -> Self {
//         match value {
//             TryReadManyWithBodyResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
//         }
//     }
// }
// async fn try_from_response_try_read_many_with_body(
//     response: reqwest::Response,
// ) -> Result<
//     TryReadManyWithBodyResponseVariants,
//     crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
// > {
//     let status_code = response.status();
//     let headers = response.headers().clone();
//     if status_code == http::StatusCode::OK {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryReadManyWithBodyResponseVariantsTvfrr200Ok > (& response_text)
//             {
//                 Ok(value) =>
//                 Ok(TryReadManyWithBodyResponseVariants :: from(value)), Err(e)
//                 =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::BAD_REQUEST {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryReadManyWithBodyResponseVariantsTvfrr400BadRequest >
//             (& response_text)
//             {
//                 Ok(value) =>
//                 Ok(TryReadManyWithBodyResponseVariants :: from(value)), Err(e)
//                 =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError >
//             (& response_text)
//             {
//                 Ok(value) =>
//                 Ok(TryReadManyWithBodyResponseVariants :: from(value)), Err(e)
//                 =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::NOT_FOUND {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryReadManyWithBodyResponseVariantsTvfrr404NotFound >
//             (& response_text)
//             {
//                 Ok(value) =>
//                 Ok(TryReadManyWithBodyResponseVariants :: from(value)), Err(e)
//                 =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else {
//         match response.text().await
//         {
//             Ok(response_text) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ResponseText(response_text)
//             },), Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ReqwestError(e),
//             },),
//         }
//     }
// }
// impl TryFrom<TryReadManyWithBodyResponseVariants>
//     for Vec<crate::repositories_types::tufa_server::routes::api::cats::DogOptions>
// {
//     type Error = TryReadManyWithBodyWithSerializeDeserialize;
//     fn try_from(value: TryReadManyWithBodyResponseVariants) -> Result<Self, Self::Error> {
//         match value
//         {
//             TryReadManyWithBodyResponseVariants :: Desirable(i) => Ok(i),
//             TryReadManyWithBodyResponseVariants ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }), TryReadManyWithBodyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence }),
//             TryReadManyWithBodyResponseVariants ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: Configuration
//             { configuration_box_dyn_error, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: Database
//             { box_dyn_database_error, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: Io
//             { io_error, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: Io
//             { io_error, code_occurence }), TryReadManyWithBodyResponseVariants
//             :: Tls { box_dyn_error, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: Protocol
//             { protocol, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence }), TryReadManyWithBodyResponseVariants
//             :: RowNotFound { row_not_found, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: TypeNotFound
//             { type_not_found, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: ColumnNotFound
//             { column_not_found, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: Decode
//             { decode_box_dyn_error, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: PoolTimedOut
//             { pool_timed_out, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: PoolClosed
//             { pool_closed, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: WorkerCrashed
//             { worker_crashed, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: Migrate
//             { migrate, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence }), TryReadManyWithBodyResponseVariants
//             :: JsonDataError { json_data_error, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: JsonSyntaxError
//             { json_syntax_error, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: MissingJsonContentType
//             { json_syntax_error, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize ::
//             MissingJsonContentType { json_syntax_error, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: BytesRejection
//             { bytes_rejection, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: NotUniquePrimaryKey
//             { not_unique_primary_keys, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize ::
//             NotUniquePrimaryKey { not_unique_primary_keys, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: NotUniqueNameVec
//             { not_unique_name_vec, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize ::
//             NotUniqueNameVec { not_unique_name_vec, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: NotUniqueColorVec
//             { not_unique_color_vec, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize ::
//             NotUniqueColorVec { not_unique_color_vec, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: BindQuery
//             { checked_add, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: BindQuery
//             { checked_add, code_occurence }),
//             TryReadManyWithBodyResponseVariants :: NotUuid
//             { not_uuid, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: NotUuid
//             { not_uuid, code_occurence }), TryReadManyWithBodyResponseVariants
//             ::
//             ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize,
//                 code_occurence
//             } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize ::
//             ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize,
//                 code_occurence
//             }), TryReadManyWithBodyResponseVariants :: UnexpectedCase
//             { unexpected_case, code_occurence } =>
//             Err(TryReadManyWithBodyWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence })
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
// pub enum TryReadManyWithBodyRequestError {
//     ExpectedType {
//         #[eo_display_with_serialize_deserialize]
//         expected_type: TryReadManyWithBodyWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedStatusCode {
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_foreign_type]
//         response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToGetResponseText {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     DeserializeResponse {
//         #[eo_display]
//         serde: serde_json::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Reqwest {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// async fn tvfrr_extraction_logic_try_read_many_with_body<'a>(
//     future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
// ) -> Result<
//     Vec<crate::repositories_types::tufa_server::routes::api::cats::DogOptions>,
//     TryReadManyWithBodyRequestError,
// > {
//     match future.await
//     {
//         Ok(response) => match
//         try_from_response_try_read_many_with_body(response).await
//         {
//             Ok(variants) => match Vec :: < crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats :: DogOptions > ::
//             try_from(variants)
//             {
//                 Ok(value) => Ok(value), Err(e) =>
//                 Err(TryReadManyWithBodyRequestError :: ExpectedType
//                 {
//                     expected_type : e, code_occurence : crate ::
//                     code_occurence_tufa_common! (),
//                 }),
//             }, Err(e) => match e
//             {
//                 crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: StatusCode
//                 { status_code, headers, response_text_result, } =>
//                 Err(TryReadManyWithBodyRequestError :: UnexpectedStatusCode
//                 {
//                     status_code, headers, response_text_result, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: FailedToGetResponseText
//                 { reqwest, status_code, headers } =>
//                 Err(TryReadManyWithBodyRequestError :: FailedToGetResponseText
//                 {
//                     reqwest, status_code, headers, code_occurence : crate ::
//                     code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde, status_code, headers, response_text, } =>
//                 Err(TryReadManyWithBodyRequestError :: DeserializeResponse
//                 {
//                     serde, status_code, headers, response_text, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }),
//             },
//         }, Err(e) =>
//         Err(TryReadManyWithBodyRequestError :: Reqwest
//         {
//             reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
//             (),
//         }),
//     }
// }
// pub enum TryReadManyWithBodyStatusCodesChecker {
//     ProjectCommitExtractorNotEqualTvfrr400BadRequest,
//     ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
//     NoProjectCommitExtractorHeaderTvfrr400BadRequest,
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
//     JsonDataErrorTvfrr400BadRequest,
//     JsonSyntaxErrorTvfrr400BadRequest,
//     MissingJsonContentTypeTvfrr400BadRequest,
//     BytesRejectionTvfrr500InternalServerError,
//     NotUniquePrimaryKeyTvfrr400BadRequest,
//     NotUniqueNameVecTvfrr400BadRequest,
//     NotUniqueColorVecTvfrr400BadRequest,
//     BindQueryTvfrr500InternalServerError,
//     NotUuidTvfrr400BadRequest,
//     ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserializeTvfrr400BadRequest,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl axum::response::IntoResponse for TryReadManyWithBodyResponseVariants {
//     fn into_response(self) -> axum::response::Response {
//         match & self
//         {
//             TryReadManyWithBodyResponseVariants :: Desirable(_) =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: OK ; res
//             } TryReadManyWithBodyResponseVariants ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants :: Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: Database
//             { box_dyn_database_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: Io
//             { io_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: NOT_FOUND ; res
//             }, TryReadManyWithBodyResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: PoolTimedOut
//             { pool_timed_out : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: REQUEST_TIMEOUT ; res
//             }, TryReadManyWithBodyResponseVariants :: PoolClosed
//             { pool_closed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: WorkerCrashed
//             { worker_crashed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: JsonDataError
//             { json_data_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants :: JsonSyntaxError
//             { json_syntax_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants :: MissingJsonContentType
//             { json_syntax_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants :: BytesRejection
//             { bytes_rejection : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: NotUniquePrimaryKey
//             { not_unique_primary_keys : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants :: NotUniqueNameVec
//             { not_unique_name_vec : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants :: NotUniqueColorVec
//             { not_unique_color_vec : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants :: BindQuery
//             { checked_add : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyWithBodyResponseVariants :: NotUuid
//             { not_uuid : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants ::
//             ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
//             {
//                 read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize
//                 : _, code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyWithBodyResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }
//         }
//     }
// }
// //////////////////
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// pub enum TryReadManyResponseVariants {
//     Desirable(Vec :: < crate :: repositories_types :: tufa_server :: routes ::
//     api :: cats :: DogOptions >), ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, RowNotFound
//     {
//         row_not_found : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolTimedOut
//     {
//         pool_timed_out : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, NotUniquePrimaryKey
//     {
//         not_unique_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniqueNameVec
//     {
//         not_unique_name_vec : Vec < std :: string :: String < >>,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniqueColorVec
//     {
//         not_unique_color_vec : Vec < std :: string :: String < >>,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, FailedToDeserializeQueryString
//     {
//         failed_to_deserialize_query_string : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ReadManyQueryTryFromReadManyQueryWithSerializeDeserialize
//     {
//         read_many_query_try_from_read_many_query_with_serialize_deserialize :
//         ReadManyQueryTryFromReadManyQueryWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryReadMany> for TryReadManyResponseVariants {
//     fn from(val: TryReadMany) -> Self {
//         match val.into_serialize_deserialize_version()
//         {
//             TryReadManyWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryReadManyWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryReadManyWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryReadManyWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence } => Self ::
//             Configuration { configuration_box_dyn_error, code_occurence },
//             TryReadManyWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             TryReadManyWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence }, TryReadManyWithSerializeDeserialize
//             :: Tls { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryReadManyWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence }, TryReadManyWithSerializeDeserialize
//             :: RowNotFound { row_not_found, code_occurence } => Self ::
//             RowNotFound { row_not_found, code_occurence },
//             TryReadManyWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryReadManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryReadManyWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryReadManyWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryReadManyWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryReadManyWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence } => Self :: PoolTimedOut
//             { pool_timed_out, code_occurence },
//             TryReadManyWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryReadManyWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence } => Self :: WorkerCrashed
//             { worker_crashed, code_occurence },
//             TryReadManyWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence }, TryReadManyWithSerializeDeserialize
//             :: NotUniquePrimaryKey { not_unique_primary_keys, code_occurence }
//             => Self :: NotUniquePrimaryKey
//             { not_unique_primary_keys, code_occurence },
//             TryReadManyWithSerializeDeserialize :: NotUniqueNameVec
//             { not_unique_name_vec, code_occurence } => Self ::
//             NotUniqueNameVec { not_unique_name_vec, code_occurence },
//             TryReadManyWithSerializeDeserialize :: NotUniqueColorVec
//             { not_unique_color_vec, code_occurence } => Self ::
//             NotUniqueColorVec { not_unique_color_vec, code_occurence },
//             TryReadManyWithSerializeDeserialize ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence } => Self ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence },
//             TryReadManyWithSerializeDeserialize :: BindQuery
//             { checked_add, code_occurence } => Self :: BindQuery
//             { checked_add, code_occurence },
//             TryReadManyWithSerializeDeserialize ::
//             ReadManyQueryTryFromReadManyQueryWithSerializeDeserialize
//             {
//                 read_many_query_try_from_read_many_query_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             ReadManyQueryTryFromReadManyQueryWithSerializeDeserialize
//             {
//                 read_many_query_try_from_read_many_query_with_serialize_deserialize,
//                 code_occurence
//             }, TryReadManyWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std::convert::From<&TryReadManyResponseVariants> for http::StatusCode {
//     fn from(value: &TryReadManyResponseVariants) -> Self {
//         match value
//         {
//             TryReadManyResponseVariants :: Desirable(_) => http :: StatusCode
//             :: OK, TryReadManyResponseVariants ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryReadManyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryReadManyResponseVariants ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryReadManyResponseVariants ::
//             Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryReadManyResponseVariants
//             :: Database { box_dyn_database_error : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyResponseVariants :: Io
//             { io_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryReadManyResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryReadManyResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryReadManyResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
//             NOT_FOUND, TryReadManyResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryReadManyResponseVariants ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryReadManyResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryReadManyResponseVariants
//             :: PoolTimedOut { pool_timed_out : _, code_occurence : _ } => http
//             :: StatusCode :: REQUEST_TIMEOUT, TryReadManyResponseVariants ::
//             PoolClosed { pool_closed : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryReadManyResponseVariants
//             :: WorkerCrashed { worker_crashed : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryReadManyResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryReadManyResponseVariants ::
//             NotUniquePrimaryKey
//             { not_unique_primary_keys : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryReadManyResponseVariants ::
//             NotUniqueNameVec { not_unique_name_vec : _, code_occurence : _ }
//             => http :: StatusCode :: BAD_REQUEST, TryReadManyResponseVariants
//             :: NotUniqueColorVec
//             { not_unique_color_vec : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryReadManyResponseVariants ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryReadManyResponseVariants ::
//             BindQuery { checked_add : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryReadManyResponseVariants
//             :: ReadManyQueryTryFromReadManyQueryWithSerializeDeserialize
//             {
//                 read_many_query_try_from_read_many_query_with_serialize_deserialize
//                 : _, code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryReadManyResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadManyResponseVariantsTvfrr404NotFound {
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryReadManyResponseVariantsTvfrr404NotFound>
//     for TryReadManyResponseVariants
// {
//     fn from(value: TryReadManyResponseVariantsTvfrr404NotFound) -> Self {
//         match value {
//             TryReadManyResponseVariantsTvfrr404NotFound::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadManyResponseVariantsTvfrr408RequestTimeout {
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryReadManyResponseVariantsTvfrr408RequestTimeout>
//     for TryReadManyResponseVariants
// {
//     fn from(value: TryReadManyResponseVariantsTvfrr408RequestTimeout) -> Self {
//         match value {
//             TryReadManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadManyResponseVariantsTvfrr400BadRequest {
//     ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, NotUniquePrimaryKey
//     {
//         not_unique_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniqueNameVec
//     {
//         not_unique_name_vec : Vec < std :: string :: String < >>,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniqueColorVec
//     {
//         not_unique_color_vec : Vec < std :: string :: String < >>,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, FailedToDeserializeQueryString
//     {
//         failed_to_deserialize_query_string : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ReadManyQueryTryFromReadManyQueryWithSerializeDeserialize
//     {
//         read_many_query_try_from_read_many_query_with_serialize_deserialize :
//         ReadManyQueryTryFromReadManyQueryWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryReadManyResponseVariantsTvfrr400BadRequest>
//     for TryReadManyResponseVariants
// {
//     fn from(value: TryReadManyResponseVariantsTvfrr400BadRequest) -> Self {
//         match value
//         {
//             TryReadManyResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryReadManyResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryReadManyResponseVariantsTvfrr400BadRequest ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryReadManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryReadManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryReadManyResponseVariantsTvfrr400BadRequest ::
//             NotUniquePrimaryKey { not_unique_primary_keys, code_occurence } =>
//             Self :: NotUniquePrimaryKey
//             { not_unique_primary_keys, code_occurence },
//             TryReadManyResponseVariantsTvfrr400BadRequest :: NotUniqueNameVec
//             { not_unique_name_vec, code_occurence } => Self ::
//             NotUniqueNameVec { not_unique_name_vec, code_occurence },
//             TryReadManyResponseVariantsTvfrr400BadRequest :: NotUniqueColorVec
//             { not_unique_color_vec, code_occurence } => Self ::
//             NotUniqueColorVec { not_unique_color_vec, code_occurence },
//             TryReadManyResponseVariantsTvfrr400BadRequest ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence } => Self ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence },
//             TryReadManyResponseVariantsTvfrr400BadRequest ::
//             ReadManyQueryTryFromReadManyQueryWithSerializeDeserialize
//             {
//                 read_many_query_try_from_read_many_query_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             ReadManyQueryTryFromReadManyQueryWithSerializeDeserialize
//             {
//                 read_many_query_try_from_read_many_query_with_serialize_deserialize,
//                 code_occurence
//             }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadManyResponseVariantsTvfrr500InternalServerError {
//     Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryReadManyResponseVariantsTvfrr500InternalServerError>
//     for TryReadManyResponseVariants
// {
//     fn from(value: TryReadManyResponseVariantsTvfrr500InternalServerError) -> Self {
//         match value {
//             TryReadManyResponseVariantsTvfrr500InternalServerError::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             } => Self::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             },
//             TryReadManyResponseVariantsTvfrr500InternalServerError::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             } => Self::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             },
//             TryReadManyResponseVariantsTvfrr500InternalServerError::Io {
//                 io_error,
//                 code_occurence,
//             } => Self::Io {
//                 io_error,
//                 code_occurence,
//             },
//             TryReadManyResponseVariantsTvfrr500InternalServerError::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             } => Self::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             },
//             TryReadManyResponseVariantsTvfrr500InternalServerError::Protocol {
//                 protocol,
//                 code_occurence,
//             } => Self::Protocol {
//                 protocol,
//                 code_occurence,
//             },
//             TryReadManyResponseVariantsTvfrr500InternalServerError::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             } => Self::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             },
//             TryReadManyResponseVariantsTvfrr500InternalServerError::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             } => Self::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             },
//             TryReadManyResponseVariantsTvfrr500InternalServerError::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             } => Self::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             },
//             TryReadManyResponseVariantsTvfrr500InternalServerError::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             } => Self::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             },
//             TryReadManyResponseVariantsTvfrr500InternalServerError::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             } => Self::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             },
//             TryReadManyResponseVariantsTvfrr500InternalServerError::Migrate {
//                 migrate,
//                 code_occurence,
//             } => Self::Migrate {
//                 migrate,
//                 code_occurence,
//             },
//             TryReadManyResponseVariantsTvfrr500InternalServerError::BindQuery {
//                 checked_add,
//                 code_occurence,
//             } => Self::BindQuery {
//                 checked_add,
//                 code_occurence,
//             },
//             TryReadManyResponseVariantsTvfrr500InternalServerError::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryReadManyResponseVariantsTvfrr200Ok {
//     Desirable(Vec<crate::repositories_types::tufa_server::routes::api::cats::DogOptions>),
// }
// impl std::convert::From<TryReadManyResponseVariantsTvfrr200Ok> for TryReadManyResponseVariants {
//     fn from(value: TryReadManyResponseVariantsTvfrr200Ok) -> Self {
//         match value {
//             TryReadManyResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
//         }
//     }
// }
// async fn try_from_response_try_read_many(
//     response: reqwest::Response,
// ) -> Result<
//     TryReadManyResponseVariants,
//     crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
// > {
//     let status_code = response.status();
//     let headers = response.headers().clone();
//     if status_code == http::StatusCode::OK {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryReadManyResponseVariantsTvfrr200Ok > (& response_text)
//             {
//                 Ok(value) => Ok(TryReadManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::BAD_REQUEST {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryReadManyResponseVariantsTvfrr400BadRequest > (& response_text)
//             {
//                 Ok(value) => Ok(TryReadManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryReadManyResponseVariantsTvfrr500InternalServerError >
//             (& response_text)
//             {
//                 Ok(value) => Ok(TryReadManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::NOT_FOUND {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryReadManyResponseVariantsTvfrr404NotFound > (& response_text)
//             {
//                 Ok(value) => Ok(TryReadManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else {
//         match response.text().await
//         {
//             Ok(response_text) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ResponseText(response_text)
//             },), Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ReqwestError(e),
//             },),
//         }
//     }
// }
// impl TryFrom<TryReadManyResponseVariants>
//     for Vec<crate::repositories_types::tufa_server::routes::api::cats::DogOptions>
// {
//     type Error = TryReadManyWithSerializeDeserialize;
//     fn try_from(value: TryReadManyResponseVariants) -> Result<Self, Self::Error> {
//         match value
//         {
//             TryReadManyResponseVariants :: Desirable(i) => Ok(i),
//             TryReadManyResponseVariants :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } =>
//             Err(TryReadManyWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }), TryReadManyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence }),
//             TryReadManyResponseVariants :: NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence }),
//             TryReadManyResponseVariants :: Configuration
//             { configuration_box_dyn_error, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence }),
//             TryReadManyResponseVariants :: Database
//             { box_dyn_database_error, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence }),
//             TryReadManyResponseVariants :: Io { io_error, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: Io
//             { io_error, code_occurence }), TryReadManyResponseVariants :: Tls
//             { box_dyn_error, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence }), TryReadManyResponseVariants ::
//             Protocol { protocol, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence }), TryReadManyResponseVariants ::
//             RowNotFound { row_not_found, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence }), TryReadManyResponseVariants ::
//             TypeNotFound { type_not_found, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence }), TryReadManyResponseVariants
//             :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence }),
//             TryReadManyResponseVariants :: ColumnNotFound
//             { column_not_found, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence }), TryReadManyResponseVariants
//             :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence }),
//             TryReadManyResponseVariants :: Decode
//             { decode_box_dyn_error, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence }),
//             TryReadManyResponseVariants :: PoolTimedOut
//             { pool_timed_out, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence }), TryReadManyResponseVariants
//             :: PoolClosed { pool_closed, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence }), TryReadManyResponseVariants ::
//             WorkerCrashed { worker_crashed, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence }), TryReadManyResponseVariants
//             :: Migrate { migrate, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence }), TryReadManyResponseVariants ::
//             NotUniquePrimaryKey { not_unique_primary_keys, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: NotUniquePrimaryKey
//             { not_unique_primary_keys, code_occurence }),
//             TryReadManyResponseVariants :: NotUniqueNameVec
//             { not_unique_name_vec, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: NotUniqueNameVec
//             { not_unique_name_vec, code_occurence }),
//             TryReadManyResponseVariants :: NotUniqueColorVec
//             { not_unique_color_vec, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: NotUniqueColorVec
//             { not_unique_color_vec, code_occurence }),
//             TryReadManyResponseVariants :: FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence }),
//             TryReadManyResponseVariants :: BindQuery
//             { checked_add, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: BindQuery
//             { checked_add, code_occurence }), TryReadManyResponseVariants ::
//             ReadManyQueryTryFromReadManyQueryWithSerializeDeserialize
//             {
//                 read_many_query_try_from_read_many_query_with_serialize_deserialize,
//                 code_occurence
//             } =>
//             Err(TryReadManyWithSerializeDeserialize ::
//             ReadManyQueryTryFromReadManyQueryWithSerializeDeserialize
//             {
//                 read_many_query_try_from_read_many_query_with_serialize_deserialize,
//                 code_occurence
//             }), TryReadManyResponseVariants :: UnexpectedCase
//             { unexpected_case, code_occurence } =>
//             Err(TryReadManyWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence })
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
// pub enum TryReadManyRequestError {
//     ExpectedType {
//         #[eo_display_with_serialize_deserialize]
//         expected_type: TryReadManyWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedStatusCode {
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_foreign_type]
//         response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToGetResponseText {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     DeserializeResponse {
//         #[eo_display]
//         serde: serde_json::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Reqwest {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// async fn tvfrr_extraction_logic_try_read_many<'a>(
//     future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
// ) -> Result<
//     Vec<crate::repositories_types::tufa_server::routes::api::cats::DogOptions>,
//     TryReadManyRequestError,
// > {
//     match future.await
//     {
//         Ok(response) => match try_from_response_try_read_many(response).await
//         {
//             Ok(variants) => match Vec :: < crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats :: DogOptions > ::
//             try_from(variants)
//             {
//                 Ok(value) => Ok(value), Err(e) =>
//                 Err(TryReadManyRequestError :: ExpectedType
//                 {
//                     expected_type : e, code_occurence : crate ::
//                     code_occurence_tufa_common! (),
//                 }),
//             }, Err(e) => match e
//             {
//                 crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: StatusCode
//                 { status_code, headers, response_text_result, } =>
//                 Err(TryReadManyRequestError :: UnexpectedStatusCode
//                 {
//                     status_code, headers, response_text_result, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: FailedToGetResponseText
//                 { reqwest, status_code, headers } =>
//                 Err(TryReadManyRequestError :: FailedToGetResponseText
//                 {
//                     reqwest, status_code, headers, code_occurence : crate ::
//                     code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde, status_code, headers, response_text, } =>
//                 Err(TryReadManyRequestError :: DeserializeResponse
//                 {
//                     serde, status_code, headers, response_text, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }),
//             },
//         }, Err(e) =>
//         Err(TryReadManyRequestError :: Reqwest
//         {
//             reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
//             (),
//         }),
//     }
// }
// pub enum TryReadManyStatusCodesChecker {
//     ProjectCommitExtractorNotEqualTvfrr400BadRequest,
//     ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
//     NoProjectCommitExtractorHeaderTvfrr400BadRequest,
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
//     NotUniquePrimaryKeyTvfrr400BadRequest,
//     NotUniqueNameVecTvfrr400BadRequest,
//     NotUniqueColorVecTvfrr400BadRequest,
//     FailedToDeserializeQueryStringTvfrr400BadRequest,
//     BindQueryTvfrr500InternalServerError,
//     ReadManyQueryTryFromReadManyQueryWithSerializeDeserializeTvfrr400BadRequest,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl axum::response::IntoResponse for TryReadManyResponseVariants {
//     fn into_response(self) -> axum::response::Response {
//         match & self
//         {
//             TryReadManyResponseVariants :: Desirable(_) =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: OK ; res
//             } TryReadManyResponseVariants :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyResponseVariants :: NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyResponseVariants :: Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyResponseVariants :: Database
//             { box_dyn_database_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyResponseVariants :: Io
//             { io_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: NOT_FOUND ; res
//             }, TryReadManyResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyResponseVariants :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyResponseVariants :: PoolTimedOut
//             { pool_timed_out : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: REQUEST_TIMEOUT ; res
//             }, TryReadManyResponseVariants :: PoolClosed
//             { pool_closed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyResponseVariants :: WorkerCrashed
//             { worker_crashed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyResponseVariants :: NotUniquePrimaryKey
//             { not_unique_primary_keys : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyResponseVariants :: NotUniqueNameVec
//             { not_unique_name_vec : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyResponseVariants :: NotUniqueColorVec
//             { not_unique_color_vec : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyResponseVariants :: FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyResponseVariants :: BindQuery
//             { checked_add : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryReadManyResponseVariants ::
//             ReadManyQueryTryFromReadManyQueryWithSerializeDeserialize
//             {
//                 read_many_query_try_from_read_many_query_with_serialize_deserialize
//                 : _, code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryReadManyResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }
//         }
//     }
// }
// ///////
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// pub enum TryUpdateOneResponseVariants {
//     Desirable(()), ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, RowNotFound
//     {
//         row_not_found : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolTimedOut
//     {
//         pool_timed_out : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, JsonDataError
//     {
//         json_data_error : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, JsonSyntaxError
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, MissingJsonContentType
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, BytesRejection
//     {
//         bytes_rejection : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, FailedToDeserializePathParams
//     {
//         failed_to_deserialize_path_params : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, MissingPathParams
//     {
//         missing_path_params : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoPayloadFields
//     {
//         no_payload_fields : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
//     {
//         update_one_path_try_from_update_one_path_with_serialize_deserialize :
//         UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryUpdateOne> for TryUpdateOneResponseVariants {
//     fn from(val: TryUpdateOne) -> Self {
//         match val.into_serialize_deserialize_version()
//         {
//             TryUpdateOneWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryUpdateOneWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryUpdateOneWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence } => Self ::
//             Configuration { configuration_box_dyn_error, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence }, TryUpdateOneWithSerializeDeserialize
//             :: Tls { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence }, TryUpdateOneWithSerializeDeserialize
//             :: RowNotFound { row_not_found, code_occurence } => Self ::
//             RowNotFound { row_not_found, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence } => Self :: PoolTimedOut
//             { pool_timed_out, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence } => Self :: WorkerCrashed
//             { worker_crashed, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence }, TryUpdateOneWithSerializeDeserialize
//             :: JsonDataError { json_data_error, code_occurence } => Self ::
//             JsonDataError { json_data_error, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: MissingJsonContentType
//             { json_syntax_error, code_occurence } => Self ::
//             MissingJsonContentType { json_syntax_error, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             TryUpdateOneWithSerializeDeserialize ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence } => Self ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: MissingPathParams
//             { missing_path_params, code_occurence } => Self ::
//             MissingPathParams { missing_path_params, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: BindQuery
//             { checked_add, code_occurence } => Self :: BindQuery
//             { checked_add, code_occurence },
//             TryUpdateOneWithSerializeDeserialize :: NoPayloadFields
//             { no_payload_fields, code_occurence } => Self :: NoPayloadFields
//             { no_payload_fields, code_occurence },
//             TryUpdateOneWithSerializeDeserialize ::
//             UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
//             {
//                 update_one_path_try_from_update_one_path_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
//             {
//                 update_one_path_try_from_update_one_path_with_serialize_deserialize,
//                 code_occurence
//             }, TryUpdateOneWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std::convert::From<&TryUpdateOneResponseVariants> for http::StatusCode {
//     fn from(value: &TryUpdateOneResponseVariants) -> Self {
//         match value
//         {
//             TryUpdateOneResponseVariants :: Desirable(_) => http :: StatusCode
//             :: OK, TryUpdateOneResponseVariants ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryUpdateOneResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryUpdateOneResponseVariants ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryUpdateOneResponseVariants ::
//             Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryUpdateOneResponseVariants
//             :: Database { box_dyn_database_error : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryUpdateOneResponseVariants :: Io
//             { io_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryUpdateOneResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryUpdateOneResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryUpdateOneResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
//             NOT_FOUND, TryUpdateOneResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryUpdateOneResponseVariants ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryUpdateOneResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryUpdateOneResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryUpdateOneResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryUpdateOneResponseVariants
//             :: PoolTimedOut { pool_timed_out : _, code_occurence : _ } => http
//             :: StatusCode :: REQUEST_TIMEOUT, TryUpdateOneResponseVariants ::
//             PoolClosed { pool_closed : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryUpdateOneResponseVariants
//             :: WorkerCrashed { worker_crashed : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryUpdateOneResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryUpdateOneResponseVariants ::
//             JsonDataError { json_data_error : _, code_occurence : _ } => http
//             :: StatusCode :: BAD_REQUEST, TryUpdateOneResponseVariants ::
//             JsonSyntaxError { json_syntax_error : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryUpdateOneResponseVariants ::
//             MissingJsonContentType
//             { json_syntax_error : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryUpdateOneResponseVariants ::
//             BytesRejection { bytes_rejection : _, code_occurence : _ } => http
//             :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryUpdateOneResponseVariants :: FailedToDeserializePathParams
//             { failed_to_deserialize_path_params : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryUpdateOneResponseVariants ::
//             MissingPathParams { missing_path_params : _, code_occurence : _ }
//             => http :: StatusCode :: BAD_REQUEST, TryUpdateOneResponseVariants
//             :: BindQuery { checked_add : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryUpdateOneResponseVariants
//             :: NoPayloadFields { no_payload_fields : _, code_occurence : _ }
//             => http :: StatusCode :: BAD_REQUEST, TryUpdateOneResponseVariants
//             :: UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
//             {
//                 update_one_path_try_from_update_one_path_with_serialize_deserialize
//                 : _, code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryUpdateOneResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryUpdateOneResponseVariantsTvfrr500InternalServerError {
//     Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, BytesRejection
//     {
//         bytes_rejection : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryUpdateOneResponseVariantsTvfrr500InternalServerError>
//     for TryUpdateOneResponseVariants
// {
//     fn from(value: TryUpdateOneResponseVariantsTvfrr500InternalServerError) -> Self {
//         match value {
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             } => Self::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             } => Self::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::Io {
//                 io_error,
//                 code_occurence,
//             } => Self::Io {
//                 io_error,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             } => Self::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::Protocol {
//                 protocol,
//                 code_occurence,
//             } => Self::Protocol {
//                 protocol,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             } => Self::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             } => Self::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             } => Self::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             } => Self::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             } => Self::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::Migrate {
//                 migrate,
//                 code_occurence,
//             } => Self::Migrate {
//                 migrate,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::BytesRejection {
//                 bytes_rejection,
//                 code_occurence,
//             } => Self::BytesRejection {
//                 bytes_rejection,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::BindQuery {
//                 checked_add,
//                 code_occurence,
//             } => Self::BindQuery {
//                 checked_add,
//                 code_occurence,
//             },
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryUpdateOneResponseVariantsTvfrr404NotFound {
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryUpdateOneResponseVariantsTvfrr404NotFound>
//     for TryUpdateOneResponseVariants
// {
//     fn from(value: TryUpdateOneResponseVariantsTvfrr404NotFound) -> Self {
//         match value {
//             TryUpdateOneResponseVariantsTvfrr404NotFound::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryUpdateOneResponseVariantsTvfrr408RequestTimeout {
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryUpdateOneResponseVariantsTvfrr408RequestTimeout>
//     for TryUpdateOneResponseVariants
// {
//     fn from(value: TryUpdateOneResponseVariantsTvfrr408RequestTimeout) -> Self {
//         match value {
//             TryUpdateOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryUpdateOneResponseVariantsTvfrr400BadRequest {
//     ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, JsonDataError
//     {
//         json_data_error : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, JsonSyntaxError
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, MissingJsonContentType
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, FailedToDeserializePathParams
//     {
//         failed_to_deserialize_path_params : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, MissingPathParams
//     {
//         missing_path_params : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, NoPayloadFields
//     {
//         no_payload_fields : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
//     {
//         update_one_path_try_from_update_one_path_with_serialize_deserialize :
//         UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryUpdateOneResponseVariantsTvfrr400BadRequest>
//     for TryUpdateOneResponseVariants
// {
//     fn from(value: TryUpdateOneResponseVariantsTvfrr400BadRequest) -> Self {
//         match value
//         {
//             TryUpdateOneResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryUpdateOneResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryUpdateOneResponseVariantsTvfrr400BadRequest ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryUpdateOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryUpdateOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryUpdateOneResponseVariantsTvfrr400BadRequest :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             TryUpdateOneResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             TryUpdateOneResponseVariantsTvfrr400BadRequest ::
//             MissingJsonContentType { json_syntax_error, code_occurence } =>
//             Self :: MissingJsonContentType
//             { json_syntax_error, code_occurence },
//             TryUpdateOneResponseVariantsTvfrr400BadRequest ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence } => Self ::
//             FailedToDeserializePathParams
//             { failed_to_deserialize_path_params, code_occurence },
//             TryUpdateOneResponseVariantsTvfrr400BadRequest ::
//             MissingPathParams { missing_path_params, code_occurence } => Self
//             :: MissingPathParams { missing_path_params, code_occurence },
//             TryUpdateOneResponseVariantsTvfrr400BadRequest :: NoPayloadFields
//             { no_payload_fields, code_occurence } => Self :: NoPayloadFields
//             { no_payload_fields, code_occurence },
//             TryUpdateOneResponseVariantsTvfrr400BadRequest ::
//             UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
//             {
//                 update_one_path_try_from_update_one_path_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
//             {
//                 update_one_path_try_from_update_one_path_with_serialize_deserialize,
//                 code_occurence
//             }
//         }
//     }
// }
// async fn try_from_response_try_update_one(
//     response: reqwest::Response,
// ) -> Result<
//     TryUpdateOneResponseVariants,
//     crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
// > {
//     let status_code = response.status();
//     let headers = response.headers().clone();
//     if status_code == http::StatusCode::OK {
//         Ok(TryUpdateOneResponseVariants::Desirable(()))
//     } else if status_code == http::StatusCode::BAD_REQUEST {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryUpdateOneResponseVariantsTvfrr400BadRequest > (& response_text)
//             {
//                 Ok(value) => Ok(TryUpdateOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryUpdateOneResponseVariantsTvfrr500InternalServerError >
//             (& response_text)
//             {
//                 Ok(value) => Ok(TryUpdateOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::NOT_FOUND {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryUpdateOneResponseVariantsTvfrr404NotFound > (& response_text)
//             {
//                 Ok(value) => Ok(TryUpdateOneResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else {
//         match response.text().await
//         {
//             Ok(response_text) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ResponseText(response_text)
//             },), Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ReqwestError(e),
//             },),
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
// pub enum TryUpdateOneRequestError {
//     ExpectedType {
//         #[eo_display_with_serialize_deserialize]
//         expected_type: TryUpdateOneWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedStatusCode {
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_foreign_type]
//         response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToGetResponseText {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     DeserializeResponse {
//         #[eo_display]
//         serde: serde_json::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Reqwest {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// async fn tvfrr_extraction_logic_try_update_one<'a>(
//     future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
// ) -> Result<(), TryUpdateOneRequestError> {
//     match future.await
//     {
//         Ok(response) => match try_from_response_try_update_one(response).await
//         {
//             Ok(_variants) => Ok(()), Err(e) => match e
//             {
//                 crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: StatusCode
//                 { status_code, headers, response_text_result, } =>
//                 Err(TryUpdateOneRequestError :: UnexpectedStatusCode
//                 {
//                     status_code, headers, response_text_result, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: FailedToGetResponseText
//                 { reqwest, status_code, headers } =>
//                 Err(TryUpdateOneRequestError :: FailedToGetResponseText
//                 {
//                     reqwest, status_code, headers, code_occurence : crate ::
//                     code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde, status_code, headers, response_text, } =>
//                 Err(TryUpdateOneRequestError :: DeserializeResponse
//                 {
//                     serde, status_code, headers, response_text, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }),
//             },
//         }, Err(e) =>
//         Err(TryUpdateOneRequestError :: Reqwest
//         {
//             reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
//             (),
//         }),
//     }
// }
// pub enum TryUpdateOneStatusCodesChecker {
//     ProjectCommitExtractorNotEqualTvfrr400BadRequest,
//     ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
//     NoProjectCommitExtractorHeaderTvfrr400BadRequest,
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
//     JsonDataErrorTvfrr400BadRequest,
//     JsonSyntaxErrorTvfrr400BadRequest,
//     MissingJsonContentTypeTvfrr400BadRequest,
//     BytesRejectionTvfrr500InternalServerError,
//     FailedToDeserializePathParamsTvfrr400BadRequest,
//     MissingPathParamsTvfrr400BadRequest,
//     BindQueryTvfrr500InternalServerError,
//     NoPayloadFieldsTvfrr400BadRequest,
//     UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeTvfrr400BadRequest,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl axum::response::IntoResponse for TryUpdateOneResponseVariants {
//     fn into_response(self) -> axum::response::Response {
//         match & self
//         {
//             TryUpdateOneResponseVariants :: Desirable(_) =>
//             { http :: StatusCode :: OK.into_response() }
//             TryUpdateOneResponseVariants :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateOneResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateOneResponseVariants :: NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateOneResponseVariants :: Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: Database
//             { box_dyn_database_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: Io
//             { io_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: NOT_FOUND ; res
//             }, TryUpdateOneResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateOneResponseVariants :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateOneResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: PoolTimedOut
//             { pool_timed_out : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: REQUEST_TIMEOUT ; res
//             }, TryUpdateOneResponseVariants :: PoolClosed
//             { pool_closed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: WorkerCrashed
//             { worker_crashed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: JsonDataError
//             { json_data_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateOneResponseVariants :: JsonSyntaxError
//             { json_syntax_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateOneResponseVariants :: MissingJsonContentType
//             { json_syntax_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateOneResponseVariants :: BytesRejection
//             { bytes_rejection : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: FailedToDeserializePathParams
//             { failed_to_deserialize_path_params : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateOneResponseVariants :: MissingPathParams
//             { missing_path_params : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateOneResponseVariants :: BindQuery
//             { checked_add : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateOneResponseVariants :: NoPayloadFields
//             { no_payload_fields : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateOneResponseVariants ::
//             UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
//             {
//                 update_one_path_try_from_update_one_path_with_serialize_deserialize
//                 : _, code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateOneResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }
//         }
//     }
// }
// /////
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// pub enum TryUpdateManyResponseVariants {
//     Desirable(()), ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, RowNotFound
//     {
//         row_not_found : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolTimedOut
//     {
//         pool_timed_out : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, JsonDataError
//     {
//         json_data_error : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, JsonSyntaxError
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, MissingJsonContentType
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, BytesRejection
//     {
//         bytes_rejection : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, NotUniquePrimaryKey
//     {
//         not_unique_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, CheckedAdd
//     {
//         checked_add : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, NoPayloadFields
//     {
//         no_payload_fields : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, CommitFailed
//     {
//         commit_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, NonExistingPrimaryKeys
//     {
//         non_existing_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, PrimaryKeyFromRowAndFailedRollback
//     {
//         primary_key_from_row : std :: string :: String, rollback_error : std
//         :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, NonExistingPrimaryKeysAndFailedRollback
//     {
//         non_existing_primary_keys : Vec < std :: string :: String >,
//         rollback_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, QueryAndRollbackFailed
//     {
//         query_error : std :: string :: String, rollback_error : std :: string
//         :: String, code_occurence : crate :: common :: code_occurence ::
//         CodeOccurence
//     },
//     UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
//     {
//         update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize
//         :
//         UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryUpdateMany> for TryUpdateManyResponseVariants {
//     fn from(val: TryUpdateMany) -> Self {
//         match val.into_serialize_deserialize_version()
//         {
//             TryUpdateManyWithSerializeDeserialize ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryUpdateManyWithSerializeDeserialize ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryUpdateManyWithSerializeDeserialize ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: Configuration
//             { configuration_box_dyn_error, code_occurence } => Self ::
//             Configuration { configuration_box_dyn_error, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: Database
//             { box_dyn_database_error, code_occurence } => Self :: Database
//             { box_dyn_database_error, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: RowNotFound
//             { row_not_found, code_occurence } => Self :: RowNotFound
//             { row_not_found, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: PoolTimedOut
//             { pool_timed_out, code_occurence } => Self :: PoolTimedOut
//             { pool_timed_out, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: WorkerCrashed
//             { worker_crashed, code_occurence } => Self :: WorkerCrashed
//             { worker_crashed, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence }, TryUpdateManyWithSerializeDeserialize
//             :: JsonDataError { json_data_error, code_occurence } => Self ::
//             JsonDataError { json_data_error, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: MissingJsonContentType
//             { json_syntax_error, code_occurence } => Self ::
//             MissingJsonContentType { json_syntax_error, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: NotUniquePrimaryKey
//             { not_unique_primary_keys, code_occurence } => Self ::
//             NotUniquePrimaryKey { not_unique_primary_keys, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: BindQuery
//             { checked_add, code_occurence } => Self :: BindQuery
//             { checked_add, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: CheckedAdd
//             { checked_add, code_occurence } => Self :: CheckedAdd
//             { checked_add, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: NoPayloadFields
//             { no_payload_fields, code_occurence } => Self :: NoPayloadFields
//             { no_payload_fields, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: CommitFailed
//             { commit_error, code_occurence } => Self :: CommitFailed
//             { commit_error, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: NonExistingPrimaryKeys
//             { non_existing_primary_keys, code_occurence } => Self ::
//             NonExistingPrimaryKeys
//             { non_existing_primary_keys, code_occurence },
//             TryUpdateManyWithSerializeDeserialize ::
//             PrimaryKeyFromRowAndFailedRollback
//             { primary_key_from_row, rollback_error, code_occurence } => Self
//             :: PrimaryKeyFromRowAndFailedRollback
//             { primary_key_from_row, rollback_error, code_occurence },
//             TryUpdateManyWithSerializeDeserialize ::
//             NonExistingPrimaryKeysAndFailedRollback
//             { non_existing_primary_keys, rollback_error, code_occurence } =>
//             Self :: NonExistingPrimaryKeysAndFailedRollback
//             { non_existing_primary_keys, rollback_error, code_occurence },
//             TryUpdateManyWithSerializeDeserialize :: QueryAndRollbackFailed
//             { query_error, rollback_error, code_occurence } => Self ::
//             QueryAndRollbackFailed
//             { query_error, rollback_error, code_occurence },
//             TryUpdateManyWithSerializeDeserialize ::
//             UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
//             {
//                 update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
//             {
//                 update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize,
//                 code_occurence
//             }, TryUpdateManyWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std::convert::From<&TryUpdateManyResponseVariants> for http::StatusCode {
//     fn from(value: &TryUpdateManyResponseVariants) -> Self {
//         match value
//         {
//             TryUpdateManyResponseVariants :: Desirable(_) => http ::
//             StatusCode :: OK, TryUpdateManyResponseVariants ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryUpdateManyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryUpdateManyResponseVariants
//             :: NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryUpdateManyResponseVariants ::
//             Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryUpdateManyResponseVariants
//             :: Database { box_dyn_database_error : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryUpdateManyResponseVariants :: Io
//             { io_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryUpdateManyResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryUpdateManyResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryUpdateManyResponseVariants ::
//             RowNotFound { row_not_found : _, code_occurence : _ } => http ::
//             StatusCode :: NOT_FOUND, TryUpdateManyResponseVariants ::
//             TypeNotFound { type_not_found : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryUpdateManyResponseVariants ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryUpdateManyResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } => http :: StatusCode
//             :: BAD_REQUEST, TryUpdateManyResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryUpdateManyResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryUpdateManyResponseVariants
//             :: PoolTimedOut { pool_timed_out : _, code_occurence : _ } => http
//             :: StatusCode :: REQUEST_TIMEOUT, TryUpdateManyResponseVariants ::
//             PoolClosed { pool_closed : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryUpdateManyResponseVariants
//             :: WorkerCrashed { worker_crashed : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryUpdateManyResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryUpdateManyResponseVariants ::
//             JsonDataError { json_data_error : _, code_occurence : _ } => http
//             :: StatusCode :: BAD_REQUEST, TryUpdateManyResponseVariants ::
//             JsonSyntaxError { json_syntax_error : _, code_occurence : _ } =>
//             http :: StatusCode :: BAD_REQUEST, TryUpdateManyResponseVariants
//             :: MissingJsonContentType
//             { json_syntax_error : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryUpdateManyResponseVariants ::
//             BytesRejection { bytes_rejection : _, code_occurence : _ } => http
//             :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryUpdateManyResponseVariants :: NotUniquePrimaryKey
//             { not_unique_primary_keys : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryUpdateManyResponseVariants ::
//             BindQuery { checked_add : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryUpdateManyResponseVariants
//             :: CheckedAdd { checked_add : _, code_occurence : _ } => http ::
//             StatusCode :: INTERNAL_SERVER_ERROR, TryUpdateManyResponseVariants
//             :: NoPayloadFields { no_payload_fields : _, code_occurence : _ }
//             => http :: StatusCode :: BAD_REQUEST,
//             TryUpdateManyResponseVariants :: CommitFailed
//             { commit_error : _, code_occurence : _ } => http :: StatusCode ::
//             INTERNAL_SERVER_ERROR, TryUpdateManyResponseVariants ::
//             NonExistingPrimaryKeys
//             { non_existing_primary_keys : _, code_occurence : _ } => http ::
//             StatusCode :: BAD_REQUEST, TryUpdateManyResponseVariants ::
//             PrimaryKeyFromRowAndFailedRollback
//             {
//                 primary_key_from_row : _, rollback_error : _, code_occurence :
//                 _
//             } => http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryUpdateManyResponseVariants ::
//             NonExistingPrimaryKeysAndFailedRollback
//             {
//                 non_existing_primary_keys : _, rollback_error : _,
//                 code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryUpdateManyResponseVariants :: QueryAndRollbackFailed
//             { query_error : _, rollback_error : _, code_occurence : _ } =>
//             http :: StatusCode :: INTERNAL_SERVER_ERROR,
//             TryUpdateManyResponseVariants ::
//             UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
//             {
//                 update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize
//                 : _, code_occurence : _
//             } => http :: StatusCode :: BAD_REQUEST,
//             TryUpdateManyResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } => http :: StatusCode
//             :: INTERNAL_SERVER_ERROR
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryUpdateManyResponseVariantsTvfrr400BadRequest {
//     ProjectCommitExtractorNotEqual
//     {
//         project_commit_not_equal : std :: string :: String < >,
//         project_commit_to_use : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, ProjectCommitExtractorToStrConversion
//     {
//         project_commit_to_str_conversion : std :: string :: String,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoProjectCommitExtractorHeader
//     {
//         no_project_commit_header : std :: string :: String < >, code_occurence
//         : crate :: common :: code_occurence :: CodeOccurence
//     }, TypeNotFound
//     {
//         type_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, ColumnNotFound
//     {
//         column_not_found : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, JsonDataError
//     {
//         json_data_error : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, JsonSyntaxError
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, MissingJsonContentType
//     {
//         json_syntax_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, NotUniquePrimaryKey
//     {
//         not_unique_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NoPayloadFields
//     {
//         no_payload_fields : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, NonExistingPrimaryKeys
//     {
//         non_existing_primary_keys : Vec < std :: string :: String >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, NonExistingPrimaryKeysAndFailedRollback
//     {
//         non_existing_primary_keys : Vec < std :: string :: String >,
//         rollback_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     },
//     UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
//     {
//         update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize
//         :
//         UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryUpdateManyResponseVariantsTvfrr400BadRequest>
//     for TryUpdateManyResponseVariants
// {
//     fn from(value: TryUpdateManyResponseVariantsTvfrr400BadRequest) -> Self {
//         match value
//         {
//             TryUpdateManyResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             } => Self :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal, project_commit_to_use,
//                 code_occurence
//             }, TryUpdateManyResponseVariantsTvfrr400BadRequest ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence } => Self ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr400BadRequest ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence } => Self ::
//             NoProjectCommitExtractorHeader
//             { no_project_commit_header, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr400BadRequest :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr400BadRequest ::
//             MissingJsonContentType { json_syntax_error, code_occurence } =>
//             Self :: MissingJsonContentType
//             { json_syntax_error, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr400BadRequest ::
//             NotUniquePrimaryKey { not_unique_primary_keys, code_occurence } =>
//             Self :: NotUniquePrimaryKey
//             { not_unique_primary_keys, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr400BadRequest :: NoPayloadFields
//             { no_payload_fields, code_occurence } => Self :: NoPayloadFields
//             { no_payload_fields, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr400BadRequest ::
//             NonExistingPrimaryKeys
//             { non_existing_primary_keys, code_occurence } => Self ::
//             NonExistingPrimaryKeys
//             { non_existing_primary_keys, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr400BadRequest ::
//             NonExistingPrimaryKeysAndFailedRollback
//             { non_existing_primary_keys, rollback_error, code_occurence } =>
//             Self :: NonExistingPrimaryKeysAndFailedRollback
//             { non_existing_primary_keys, rollback_error, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr400BadRequest ::
//             UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
//             {
//                 update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize,
//                 code_occurence
//             } => Self ::
//             UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
//             {
//                 update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize,
//                 code_occurence
//             }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryUpdateManyResponseVariantsTvfrr500InternalServerError {
//     Configuration
//     {
//         configuration_box_dyn_error : std :: string :: String < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, Database
//     {
//         box_dyn_database_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, Io
//     {
//         io_error : std :: string :: String, code_occurence : crate :: common
//         :: code_occurence :: CodeOccurence
//     }, Tls
//     {
//         box_dyn_error : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, Protocol
//     {
//         protocol : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, ColumnIndexOutOfBounds
//     {
//         column_index_out_of_bounds : usize < >, len : usize < >,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, ColumnDecode
//     {
//         column_decode_index : std :: string :: String < >, source_handle : std
//         :: string :: String < >, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, Decode
//     {
//         decode_box_dyn_error : std :: string :: String < >, code_occurence :
//         crate :: common :: code_occurence :: CodeOccurence
//     }, PoolClosed
//     {
//         pool_closed : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, WorkerCrashed
//     {
//         worker_crashed : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, Migrate
//     {
//         migrate : std :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, BytesRejection
//     {
//         bytes_rejection : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }, BindQuery
//     {
//         checked_add : crate :: server :: postgres :: bind_query ::
//         TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence : crate :: common :: code_occurence :: CodeOccurence
//     }, CheckedAdd
//     {
//         checked_add : std :: string :: String < >, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, CommitFailed
//     {
//         commit_error : std :: string :: String, code_occurence : crate ::
//         common :: code_occurence :: CodeOccurence
//     }, PrimaryKeyFromRowAndFailedRollback
//     {
//         primary_key_from_row : std :: string :: String, rollback_error : std
//         :: string :: String, code_occurence : crate :: common ::
//         code_occurence :: CodeOccurence
//     }, QueryAndRollbackFailed
//     {
//         query_error : std :: string :: String, rollback_error : std :: string
//         :: String, code_occurence : crate :: common :: code_occurence ::
//         CodeOccurence
//     }, UnexpectedCase
//     {
//         unexpected_case : std :: string :: String < >, code_occurence : crate
//         :: common :: code_occurence :: CodeOccurence
//     }
// }
// impl std::convert::From<TryUpdateManyResponseVariantsTvfrr500InternalServerError>
//     for TryUpdateManyResponseVariants
// {
//     fn from(value: TryUpdateManyResponseVariantsTvfrr500InternalServerError) -> Self {
//         match value
//         {
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             Configuration { configuration_box_dyn_error, code_occurence } =>
//             Self :: Configuration
//             { configuration_box_dyn_error, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             Database { box_dyn_database_error, code_occurence } => Self ::
//             Database { box_dyn_database_error, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError :: Io
//             { io_error, code_occurence } => Self :: Io
//             { io_error, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError :: Tls
//             { box_dyn_error, code_occurence } => Self :: Tls
//             { box_dyn_error, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             Protocol { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError :: Decode
//             { decode_box_dyn_error, code_occurence } => Self :: Decode
//             { decode_box_dyn_error, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             WorkerCrashed { worker_crashed, code_occurence } => Self ::
//             WorkerCrashed { worker_crashed, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             Migrate { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             BytesRejection { bytes_rejection, code_occurence } => Self ::
//             BytesRejection { bytes_rejection, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             BindQuery { checked_add, code_occurence } => Self :: BindQuery
//             { checked_add, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             CheckedAdd { checked_add, code_occurence } => Self :: CheckedAdd
//             { checked_add, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             CommitFailed { commit_error, code_occurence } => Self ::
//             CommitFailed { commit_error, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             PrimaryKeyFromRowAndFailedRollback
//             { primary_key_from_row, rollback_error, code_occurence } => Self
//             :: PrimaryKeyFromRowAndFailedRollback
//             { primary_key_from_row, rollback_error, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             QueryAndRollbackFailed
//             { query_error, rollback_error, code_occurence } => Self ::
//             QueryAndRollbackFailed
//             { query_error, rollback_error, code_occurence },
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
//             UnexpectedCase { unexpected_case, code_occurence } => Self ::
//             UnexpectedCase { unexpected_case, code_occurence }
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryUpdateManyResponseVariantsTvfrr404NotFound {
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryUpdateManyResponseVariantsTvfrr404NotFound>
//     for TryUpdateManyResponseVariants
// {
//     fn from(value: TryUpdateManyResponseVariantsTvfrr404NotFound) -> Self {
//         match value {
//             TryUpdateManyResponseVariantsTvfrr404NotFound::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryUpdateManyResponseVariantsTvfrr408RequestTimeout {
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::From<TryUpdateManyResponseVariantsTvfrr408RequestTimeout>
//     for TryUpdateManyResponseVariants
// {
//     fn from(value: TryUpdateManyResponseVariantsTvfrr408RequestTimeout) -> Self {
//         match value {
//             TryUpdateManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//         }
//     }
// }
// async fn try_from_response_try_update_many(
//     response: reqwest::Response,
// ) -> Result<
//     TryUpdateManyResponseVariants,
//     crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
// > {
//     let status_code = response.status();
//     let headers = response.headers().clone();
//     if status_code == http::StatusCode::OK {
//         Ok(TryUpdateManyResponseVariants::Desirable(()))
//     } else if status_code == http::StatusCode::BAD_REQUEST {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryUpdateManyResponseVariantsTvfrr400BadRequest >
//             (& response_text)
//             {
//                 Ok(value) => Ok(TryUpdateManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryUpdateManyResponseVariantsTvfrr500InternalServerError >
//             (& response_text)
//             {
//                 Ok(value) => Ok(TryUpdateManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else if status_code == http::StatusCode::NOT_FOUND {
//         match response.text().await
//         {
//             Ok(response_text) => match serde_json :: from_str :: <
//             TryUpdateManyResponseVariantsTvfrr404NotFound > (& response_text)
//             {
//                 Ok(value) => Ok(TryUpdateManyResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }, Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: FailedToGetResponseText
//             { reqwest : e, status_code, headers, }),
//         }
//     } else {
//         match response.text().await
//         {
//             Ok(response_text) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ResponseText(response_text)
//             },), Err(e) =>
//             Err(crate :: common :: api_request_unexpected_error ::
//             ApiRequestUnexpectedError :: StatusCode
//             {
//                 status_code, headers, response_text_result : crate :: common
//                 :: api_request_unexpected_error :: ResponseTextResult ::
//                 ReqwestError(e),
//             },),
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
// pub enum TryUpdateManyRequestError {
//     ExpectedType {
//         #[eo_display_with_serialize_deserialize]
//         expected_type: TryUpdateManyWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     UnexpectedStatusCode {
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_foreign_type]
//         response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     FailedToGetResponseText {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     DeserializeResponse {
//         #[eo_display]
//         serde: serde_json::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     Reqwest {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// async fn tvfrr_extraction_logic_try_update_many<'a>(
//     future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
// ) -> Result<(), TryUpdateManyRequestError> {
//     match future.await
//     {
//         Ok(response) => match
//         try_from_response_try_update_many(response).await
//         {
//             Ok(_variants) => Ok(()), Err(e) => match e
//             {
//                 crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: StatusCode
//                 { status_code, headers, response_text_result, } =>
//                 Err(TryUpdateManyRequestError :: UnexpectedStatusCode
//                 {
//                     status_code, headers, response_text_result, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: FailedToGetResponseText
//                 { reqwest, status_code, headers } =>
//                 Err(TryUpdateManyRequestError :: FailedToGetResponseText
//                 {
//                     reqwest, status_code, headers, code_occurence : crate ::
//                     code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde, status_code, headers, response_text, } =>
//                 Err(TryUpdateManyRequestError :: DeserializeResponse
//                 {
//                     serde, status_code, headers, response_text, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }),
//             },
//         }, Err(e) =>
//         Err(TryUpdateManyRequestError :: Reqwest
//         {
//             reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
//             (),
//         }),
//     }
// }
// pub enum TryUpdateManyStatusCodesChecker {
//     ProjectCommitExtractorNotEqualTvfrr400BadRequest,
//     ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
//     NoProjectCommitExtractorHeaderTvfrr400BadRequest,
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
//     JsonDataErrorTvfrr400BadRequest,
//     JsonSyntaxErrorTvfrr400BadRequest,
//     MissingJsonContentTypeTvfrr400BadRequest,
//     BytesRejectionTvfrr500InternalServerError,
//     NotUniquePrimaryKeyTvfrr400BadRequest,
//     BindQueryTvfrr500InternalServerError,
//     CheckedAddTvfrr500InternalServerError,
//     NoPayloadFieldsTvfrr400BadRequest,
//     CommitFailedTvfrr500InternalServerError,
//     NonExistingPrimaryKeysTvfrr400BadRequest,
//     PrimaryKeyFromRowAndFailedRollbackTvfrr500InternalServerError,
//     NonExistingPrimaryKeysAndFailedRollbackTvfrr400BadRequest,
//     QueryAndRollbackFailedTvfrr500InternalServerError,
//     UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeTvfrr400BadRequest,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl axum::response::IntoResponse for TryUpdateManyResponseVariants {
//     fn into_response(self) -> axum::response::Response {
//         match & self
//         {
//             TryUpdateManyResponseVariants :: Desirable(_) =>
//             { http :: StatusCode :: OK.into_response() }
//             TryUpdateManyResponseVariants :: ProjectCommitExtractorNotEqual
//             {
//                 project_commit_not_equal : _, project_commit_to_use : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants ::
//             ProjectCommitExtractorToStrConversion
//             { project_commit_to_str_conversion : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants :: NoProjectCommitExtractorHeader
//             { no_project_commit_header : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants :: Configuration
//             { configuration_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: Database
//             { box_dyn_database_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: Io
//             { io_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: Tls
//             { box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: Protocol
//             { protocol : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: RowNotFound
//             { row_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: NOT_FOUND ; res
//             }, TryUpdateManyResponseVariants :: TypeNotFound
//             { type_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: ColumnNotFound
//             { column_not_found : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants :: ColumnDecode
//             { column_decode_index : _, source_handle : _, code_occurence : _ }
//             =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: Decode
//             { decode_box_dyn_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: PoolTimedOut
//             { pool_timed_out : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: REQUEST_TIMEOUT ; res
//             }, TryUpdateManyResponseVariants :: PoolClosed
//             { pool_closed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: WorkerCrashed
//             { worker_crashed : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: Migrate
//             { migrate : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: JsonDataError
//             { json_data_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants :: JsonSyntaxError
//             { json_syntax_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants :: MissingJsonContentType
//             { json_syntax_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants :: BytesRejection
//             { bytes_rejection : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: NotUniquePrimaryKey
//             { not_unique_primary_keys : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants :: BindQuery
//             { checked_add : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: CheckedAdd
//             { checked_add : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: NoPayloadFields
//             { no_payload_fields : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants :: CommitFailed
//             { commit_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants :: NonExistingPrimaryKeys
//             { non_existing_primary_keys : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants ::
//             PrimaryKeyFromRowAndFailedRollback
//             {
//                 primary_key_from_row : _, rollback_error : _, code_occurence :
//                 _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants ::
//             NonExistingPrimaryKeysAndFailedRollback
//             {
//                 non_existing_primary_keys : _, rollback_error : _,
//                 code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants :: QueryAndRollbackFailed
//             { query_error : _, rollback_error : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }, TryUpdateManyResponseVariants ::
//             UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
//             {
//                 update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize
//                 : _, code_occurence : _
//             } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: BAD_REQUEST ; res
//             }, TryUpdateManyResponseVariants :: UnexpectedCase
//             { unexpected_case : _, code_occurence : _ } =>
//             {
//                 let mut res = axum :: Json(self).into_response() ; *
//                 res.status_mut() = http :: StatusCode :: INTERNAL_SERVER_ERROR
//                 ; res
//             }
//         }
//     }
// }
////////////////////////////////////////////////////////////////
// https://learn.microsoft.com/en-us/rest/api/storageservices/table-service-rest-api
