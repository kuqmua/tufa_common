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
    #[tvfrr_400_bad_request]
    NotUniqueNameVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_name_vec: Vec<crate::server::postgres::regex_filter::RegexFilter>,
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
pub async fn delete_with_body<'a>(
    app_info_state : axum :: extract :: State < crate :: repositories_types ::
tufa_server :: routes :: api :: cats :: DynArcGetConfigGetPostgresPoolSendSync
>,
    payload_extraction_result: Result<
        axum::Json<DeleteWithBodyPayload>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = DeleteWithBodyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                DeleteWithBodyPayload,
                TryDeleteWithBodyResponseVariants,
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
        if let (None, None, None) = (
            &parameters.payload.id,
            &parameters.payload.name,
            &parameters.payload.color,
        ) {
            return TryDeleteWithBodyResponseVariants::NoPayloadFields {
                no_payload_fields: std::string::String::from("no payload fields"),
                code_occurence: crate::code_occurence_tufa_common!(),
            };
        }
        match (
            &parameters.payload.id,
            &parameters.payload.name,
            &parameters.payload.color,
        ) {
            (Some(id), None, None) => {
                {
                    let not_unique_primary_keys = {
                        let mut vec = Vec::with_capacity(id.len());
                        let mut not_unique_primary_keys = Vec::with_capacity(id.len());
                        for element in id {
                            let handle = element.to_inner();
                            match vec.contains(&handle) {
                                true => {
                                    not_unique_primary_keys.push(*element.to_inner());
                                }
                                false => {
                                    vec.push(element.to_inner());
                                }
                            }
                        }
                        not_unique_primary_keys
                    };
                    if let false = not_unique_primary_keys.is_empty() {
                        let error = TryDeleteWithBody::NotUniquePrimeryKey {
                            not_unique_primary_keys,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteWithBodyResponseVariants::from(error);
                    }
                }
                let expected_updated_primary_keys = {
                    id.iter()
                        .map(|element| element.to_inner().clone())
                        .collect::<Vec<i64>>()
                };
                let binded_query = {
                    let query_string =
                        { "delete from cats where id in (select unnest($1)) returning id" };
                    println!("{}", query_string);
                    let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                    query = query.bind(
                        id.into_iter()
                            .map(|element| element.clone().into_inner())
                            .collect::<Vec<i64>>(),
                    );
                    query
                };
                let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDeleteWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteWithBodyResponseVariants::from(error);
                    }
                };
                let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDeleteWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteWithBodyResponseVariants::from(error);
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
                        let error = TryDeleteWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteWithBodyResponseVariants::from(error);
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
                                let error = TryDeleteWithBody::from(e);
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteWithBodyResponseVariants::from(error);
                            }
                            Err(rollback_error) => {
                                let error = TryDeleteWithBody::QueryAndRollbackFailed {
                                    query_error: e,
                                    rollback_error,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteWithBodyResponseVariants::from(error);
                            }
                        }
                    }
                    results_vec
                };
                let primary_key_vec = {
                    let mut primary_key_vec =
                        Vec::with_capacity(expected_updated_primary_keys.len());
                    for element in results_vec {
                        match primary_key_try_from_sqlx_row(&element) {
                            Ok(primary_key) => {
                                primary_key_vec.push(primary_key);
                            }
                            Err(e) => {
                                match postgres_transaction.rollback().await {
                                    Ok(_) => {
                                        let error = TryDeleteWithBody::from(e);
                                        crate :: common
                                    :: error_logs_logic :: error_log :: ErrorLog ::
                                    error_log(& error, app_info_state.as_ref(),) ;
                                        return TryDeleteWithBodyResponseVariants::from(error);
                                    }
                                    Err(rollback_error) => {
                                        let error =
                                            TryDeleteWithBody::PrimaryKeyFromRowAndFailedRollback {
                                                primary_key_from_row: e,
                                                rollback_error,
                                                code_occurence: crate::code_occurence_tufa_common!(),
                                            };
                                        crate :: common :: error_logs_logic :: error_log ::
                                    ErrorLog :: error_log(& error, app_info_state.as_ref(),) ;
                                        return TryDeleteWithBodyResponseVariants::from(error);
                                    }
                                }
                            }
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
                                let error = TryDeleteWithBody::NonExistingPrimaryKeys {
                                    non_existing_primary_keys,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteWithBodyResponseVariants::from(error);
                            }
                            Err(e) => {
                                let error =
                                    TryDeleteWithBody::NonExistingPrimaryKeysAndFailedRollback {
                                        non_existing_primary_keys,
                                        rollback_error: e,
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    };
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteWithBodyResponseVariants::from(error);
                            }
                        }
                    }
                }
                match postgres_transaction.commit().await {
                    Ok(_) => TryDeleteWithBodyResponseVariants::Desirable(()),
                    Err(e) => {
                        let error = TryDeleteWithBody::CommitFailed {
                            commit_error: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        TryDeleteWithBodyResponseVariants::from(error)
                    }
                }
            }
            _ => {
                if let Some(id) = &parameters.payload.id {
                    let not_unique_primary_keys = {
                        let mut vec = Vec::with_capacity(id.len());
                        let mut not_unique_primary_keys = Vec::with_capacity(id.len());
                        for element in id {
                            let handle = element.to_inner();
                            match vec.contains(&handle) {
                                true => {
                                    not_unique_primary_keys.push(*element.to_inner());
                                }
                                false => {
                                    vec.push(element.to_inner());
                                }
                            }
                        }
                        not_unique_primary_keys
                    };
                    if let false = not_unique_primary_keys.is_empty() {
                        let error = TryDeleteWithBody::NotUniquePrimeryKey {
                            not_unique_primary_keys,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteWithBodyResponseVariants::from(error);
                    }
                }
                //
                let name_handle = match parameters.payload.name {
                    Some(value) => {
                        let is_unique = {
                            let mut vec = Vec::with_capacity(value.len());
                            let mut is_unique = true;
                            for element in &value {
                                match vec.contains(&element) {
                                    true => {
                                        is_unique = false;
                                        break;
                                    }
                                    false => {
                                        vec.push(element);
                                    }
                                }
                            }
                            is_unique
                        };
                        println!("is_unique {:#?}", is_unique);
                        match is_unique {
                            true => Some(value),
                            false => {
                                let not_unique_name_vec = {
                                    let mut vec = Vec::with_capacity(value.len());
                                    let mut not_unique_name_vec = Vec::with_capacity(value.len());
                                    for element in value {
                                        match vec.contains(&element) {
                                            true => {
                                                not_unique_name_vec.push(element);
                                            }
                                            false => {
                                                vec.push(element);
                                            }
                                        }
                                    }
                                    not_unique_name_vec
                                };
                                let error = TryDeleteWithBody::NotUniqueNameVec {
                                    not_unique_name_vec,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteWithBodyResponseVariants::from(error);
                            }
                        }
                    },
                    None => None
                };
                // if let Some(value) = parameters.payload.color {
                        
                // }
                //
                let query_string = {
                    format!("delete from cats where {}", {
                        let mut increment: u64 = 0;
                        let mut additional_parameters = std::string::String::default();
                        if let Some(value) = &name_handle {
                            match crate::server::postgres::bind_query::BindQuery::try_increment(
                                value,
                                &mut increment,
                            ) {
                                Ok(_) => {
                                    let handle = format!("name = ${increment}");
                                    match additional_parameters.is_empty() {
                                        true => {
                                            additional_parameters.push_str(&handle);
                                        }
                                        false => {
                                            additional_parameters
                                                .push_str(&format!(" AND {handle}"));
                                        }
                                    }
                                }
                                Err(e) => {
                                    return TryDeleteWithBodyResponseVariants::BindQuery {
                                        checked_add: e.into_serialize_deserialize_version(),
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    };
                                }
                            }
                        }
                        if let Some(value) = &parameters.payload.color {
                            match crate::server::postgres::bind_query::BindQuery::try_increment(
                                value,
                                &mut increment,
                            ) {
                                Ok(_) => {
                                    let handle = format!("color = ${increment}");
                                    match additional_parameters.is_empty() {
                                        true => {
                                            additional_parameters.push_str(&handle);
                                        }
                                        false => {
                                            additional_parameters
                                                .push_str(&format!(" AND {handle}"));
                                        }
                                    }
                                }
                                Err(e) => {
                                    return TryDeleteWithBodyResponseVariants::BindQuery {
                                        checked_add: e.into_serialize_deserialize_version(),
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    };
                                }
                            }
                        }
                        if let Some(id) = &parameters.payload.id {
                            if let false = additional_parameters.is_empty() {
                                additional_parameters.push_str(" and");
                            }
                            additional_parameters.push_str(& format!
                            (" id in ({})",
                            {
                                let mut additional_parameters = std :: string :: String ::
                                default() ; for element in id
                                {
                                    match crate :: server :: postgres :: bind_query :: BindQuery
                                    :: try_increment(element, & mut increment,)
                                    {
                                        Ok(_) =>
                                        {
                                            additional_parameters.push_str(& format! ("${increment},"))
                                            ;
                                        } Err(e) =>
                                        {
                                            return TryDeleteWithBodyResponseVariants :: BindQuery
                                            {
                                                checked_add : e.into_serialize_deserialize_version(),
                                                code_occurence : crate :: code_occurence_tufa_common! ()
                                            } ;
                                        }
                                    }
                                } additional_parameters.pop() ; additional_parameters
                            })) ;
                        }
                        additional_parameters
                    })
                };
                println!("{}", query_string);
                let binded_query = {
                    let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                    if let Some(value) = name_handle {
                        query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                            value, query,
                        );
                    }
                    if let Some(value) = parameters.payload.color {
                        query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                            value, query,
                        );
                    }
                    if let Some(id) = parameters.payload.id {
                        for element in id {
                            query =
                                crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                                    element, query,
                                );
                        }
                    }
                    query
                };
                let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDeleteWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteWithBodyResponseVariants::from(error);
                    }
                };
                let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDeleteWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteWithBodyResponseVariants::from(error);
                    }
                };
                match binded_query.execute(pg_connection.as_mut()).await {
                    Ok(_) => TryDeleteWithBodyResponseVariants::Desirable(()),
                    Err(e) => {
                        let error = TryDeleteWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteWithBodyResponseVariants::from(error);
                    }
                }
            }
        }
    }
}