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
    serde_derive::Serialize,
    serde_derive::Deserialize,
    utoipa::ToSchema,
    generate_postgresql_crud::GeneratePostgresqlCrud,
)]
#[generate_postgresql_crud::generate_postgresql_crud_route_name(cats)]
pub struct Cat {
    #[generate_postgresql_crud_primary_key]
    pub id: i64, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    pub name: String,
    pub color: String,
}

#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_201_created
)]
pub enum TryCreateBatch {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
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
    //
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
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

////////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_201_created
)]
pub enum TryCreate {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
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
    //
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
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

////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_200_ok
)]
pub enum TryDeleteById {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
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
    //
    #[tvfrr_400_bad_request]
    FailedToDeserializePathParams {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingPathParams {
        #[eo_display_with_serialize_deserialize]
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
///////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_200_ok
)]
pub enum TryDeleteWithBody {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
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
    //
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    NotUniquePrimeryKey {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_primary_keys: Vec<i64>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoPayloadParameters {
        #[eo_display_with_serialize_deserialize]
        no_payload_parameters: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NonExistingPrimaryKeys {
        #[eo_vec_display]
        non_existing_primary_keys: Vec<i64>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    //todo what status code should return if non_existing_primary_keys = 400, but transaction rollback failed = 500
    NonExistingPrimaryKeysAndFailedRollback {
        #[eo_vec_display]
        non_existing_primary_keys: Vec<i64>,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PrimaryKeyFromRowAndFailedRollback {
        #[eo_display]
        primary_key_from_row: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    CommitFailed {
        #[eo_display]
        commit_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    QueryAndRollbackFailed {
        #[eo_display]
        query_error: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
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

/////////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_200_ok
)]
pub enum TryDelete {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
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
    //
    #[tvfrr_400_bad_request]
    FailedToDeserializeQueryString {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_query_string: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    NotUniquePrimeryKey {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_primary_keys: Vec<i64>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    DeleteQueryTryFromUrlEncoding {
        #[eo_error_occurence]
        checked_add: DeleteQueryTryFromUrlEncodingErrorNamed, //todo renaming?
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoQueryParameters {
        #[eo_display_with_serialize_deserialize]
        no_query_parameters: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    CommitFailed {
        #[eo_display]
        commit_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NonExistingPrimaryKeys {
        #[eo_vec_display]
        non_existing_primary_keys: Vec<i64>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PrimaryKeyFromRowAndFailedRollback {
        #[eo_display]
        primary_key_from_row: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    //todo what status code should return if non_existing_primary_keys = 400, but transaction rollback failed = 500
    NonExistingPrimaryKeysAndFailedRollback {
        #[eo_vec_display]
        non_existing_primary_keys: Vec<i64>,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    QueryAndRollbackFailed {
        #[eo_display]
        query_error: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
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

/////////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    crate::repositories_types::tufa_server::routes::api::cats::CatOptions,
    tvfrr_200_ok
)]
pub enum TryReadById {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
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
    //
    #[tvfrr_400_bad_request]
    FailedToDeserializePathParams {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingPathParams {
        #[eo_display_with_serialize_deserialize]
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    FailedToDeserializeQueryString {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_query_string: std::string::String,
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

////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    Vec::<crate::repositories_types::tufa_server::routes::api::cats::CatOptions>,
    tvfrr_200_ok
)]
pub enum TryReadWithBody {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
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
    //
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    NotUniquePrimeryKey {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_primary_keys: Vec<i64>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
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

////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    Vec::<crate::repositories_types::tufa_server::routes::api::cats::CatOptions>,
    tvfrr_200_ok
)]
pub enum TryRead {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
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
    //
    #[tvfrr_400_bad_request]
    NotUniquePrimeryKey {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_primary_keys: Vec<i64>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    FailedToDeserializeQueryString {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_query_string: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //todo - no parameters case?
}

/////////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_200_ok
)]
pub enum TryUpdateById {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
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
    //
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display_with_serialize_deserialize]
        json_data_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    FailedToDeserializePathParams {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingPathParams {
        #[eo_display_with_serialize_deserialize]
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

/////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_200_ok
)]
pub enum TryUpdate {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
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
    //
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display_with_serialize_deserialize]
        json_data_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    NotUniquePrimeryKey {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_primary_keys: Vec<i64>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    CheckedAdd {
        #[eo_display_with_serialize_deserialize]
        checked_add: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    CommitFailed {
        #[eo_display]
        commit_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NonExistingPrimaryKeys {
        #[eo_vec_display]
        non_existing_primary_keys: Vec<i64>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PrimaryKeyFromRowAndFailedRollback {
        #[eo_display]
        primary_key_from_row: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    //todo what status code should return if non_existing_primary_keys = 400, but transaction rollback failed = 500
    NonExistingPrimaryKeysAndFailedRollback {
        #[eo_vec_display]
        non_existing_primary_keys: Vec<i64>,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    QueryAndRollbackFailed {
        #[eo_display]
        query_error: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
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
//////
pub async fn update<'a>(
    app_info_state : axum :: extract :: State < crate :: repositories_types ::
tufa_server :: routes :: api :: cats :: DynArcGetConfigGetPostgresPoolSendSync
>,
    payload_extraction_result: Result<
        axum::Json<Vec<UpdatePayloadElement>>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = UpdateParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                Vec<UpdatePayloadElement>,
                TryUpdateResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_info_state)
            {
                Ok(value) => value,
                Err(err) => {
                    return err;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        //
                {
                    let not_unique_primary_keys = {
                        let mut vec = Vec::with_capacity(parameters.payload.len());
                        let mut not_unique_primary_keys = Vec::with_capacity(parameters.payload.len());
                        for element in &parameters.payload {
                            let handle = element.id.to_inner();
                            match vec.contains(&handle) {
                                true => {
                                    not_unique_primary_keys.push(*element.id.to_inner());
                                },
                                false => {
                                    vec.push(element.id.to_inner());
                                }
                            }
                        }
                        not_unique_primary_keys
                    };
                    if let false = not_unique_primary_keys.is_empty() {
                        let error = TryUpdate::NotUniquePrimeryKey {
                            not_unique_primary_keys,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error, 
                            app_info_state.as_ref()
                        );
                        return TryUpdateResponseVariants::from(error);
                    }
                }
        //
        let expected_updated_primary_keys = {
            parameters
                .payload
                .iter()
                .map(|element| element.id.to_inner().clone())
                .collect::<Vec<i64>>()
        };
        let binded_query = {
            let query_string = {
                "update cats as t set name = data.name, color = data.color from (select * from unnest($1, $2, $3)) as data(id, name, color) where t.id = data.id returning data.id"
            };
            println!("{}", query_string);
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            let current_vec_len = parameters.payload.len();
            let (id_vec, name_vec, color_vec) = parameters.payload.into_iter().fold(
                (
                    Vec::with_capacity(current_vec_len),
                    Vec::with_capacity(current_vec_len),
                    Vec::with_capacity(current_vec_len),
                ),
                |mut acc, element| {
                    acc.0.push(element.id);
                    acc.1.push(element.name);
                    acc.2.push(element.color);
                    acc
                },
            );
            query = query.bind(
                id_vec
                    .into_iter()
                    .map(|element| element.clone().into_inner())
                    .collect::<Vec<i64>>(),
            );
            query = query.bind(name_vec);
            query = query.bind(color_vec);
            query
        };
        let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let error = TryUpdate::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateResponseVariants::from(error);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let error = TryUpdate::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateResponseVariants::from(error);
            }
        };
        let mut postgres_transaction = match {
            use sqlx::Acquire;
            pg_connection.begin()
        }
        .await
        {
            Ok(value) => value,
            Err(e) => {
                let error = TryUpdate::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateResponseVariants::from(error);
            }
        };
        let results_vec = {
            let mut results_vec = Vec::with_capacity(expected_updated_primary_keys.len());
            let mut option_error: Option<sqlx::Error> = None;
            {
                let mut rows = binded_query.fetch(postgres_transaction.as_mut());
                while let (Some(Some(row)), None) = (
                    match {
                        use futures::TryStreamExt;
                        rows.try_next()
                    }
                    .await
                    {
                        Ok(value) => Some(value),
                        Err(e) => {
                            option_error = Some(e);
                            None
                        }
                    },
                    &option_error,
                ) {
                    results_vec.push(row);
                }
            }
            if let Some(e) = option_error {
                match postgres_transaction.rollback().await {
                    Ok(_) => {
                        let error = TryUpdate::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryUpdateResponseVariants::from(error);
                    }
                    Err(rollback_error) => {
                        let error = TryUpdate::QueryAndRollbackFailed {
                            query_error: e,
                            rollback_error,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryUpdateResponseVariants::from(error);
                    }
                }
            }
            results_vec
        };
        let primary_key_vec = {
            let mut primary_key_vec = Vec::with_capacity(expected_updated_primary_keys.len());
            for element in results_vec {
                match primary_key_try_from_sqlx_row(&element) {
                    Ok(primary_key) => {
                        primary_key_vec.push(primary_key);
                    }
                    Err(e) => match postgres_transaction.rollback().await {
                        Ok(_) => {
                            let error = TryUpdate::from(e);
                            crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                &error,
                                app_info_state.as_ref(),
                            );
                            return TryUpdateResponseVariants::from(error);
                        }
                        Err(rollback_error) => {
                            let error = TryUpdate::PrimaryKeyFromRowAndFailedRollback {
                                primary_key_from_row: e,
                                rollback_error,
                                code_occurence: crate::code_occurence_tufa_common!(),
                            };
                            crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                &error,
                                app_info_state.as_ref(),
                            );
                            return TryUpdateResponseVariants::from(error);
                        }
                    },
                }
            }
            primary_key_vec
        };
        {
            let non_existing_primary_keys = {
                let len = expected_updated_primary_keys.len();
                expected_updated_primary_keys.into_iter().fold(
                    Vec::with_capacity(len),
                    |mut acc, element| {
                        if let false = primary_key_vec.contains(&element) {
                            acc.push(element);
                        }
                        acc
                    },
                )
            };
            if let false = non_existing_primary_keys.is_empty() {
                match postgres_transaction.rollback().await {
                    Ok(_) => {
                        let error = TryUpdate::NonExistingPrimaryKeys {
                            non_existing_primary_keys,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryUpdateResponseVariants::from(error);
                    }
                    Err(e) => {
                        let error = TryUpdate::NonExistingPrimaryKeysAndFailedRollback {
                            non_existing_primary_keys,
                            rollback_error: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryUpdateResponseVariants::from(error);
                    }
                }
            }
        }
        match postgres_transaction.commit().await {
            Ok(_) => TryUpdateResponseVariants::Desirable(()),
            Err(e) => {
                let error = TryUpdate::CommitFailed {
                    commit_error: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                };
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                TryUpdateResponseVariants::from(error)
            }
        }
    }
}