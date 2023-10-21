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
    NotUniquePrimaryKey {
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
    #[tvfrr_400_bad_request]
    NotUniqueColorVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_color_vec: Vec<crate::server::postgres::regex_filter::RegexFilter>,
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
    NotUniquePrimaryKey {
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
    #[tvfrr_400_bad_request]
    NotUniqueColorVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_color_vec: Vec<crate::server::postgres::regex_filter::RegexFilter>,
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
    NotUniquePrimaryKey {
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
    NotUniquePrimaryKey {
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
    NotUniquePrimaryKey {
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
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DeleteQuery {
    pub id: Option<crate :: server:: postgres :: bigserial_ids :: BigserialIds>,
    pub name: Option<crate::server::routes::helpers::strings_deserialized_from_string_splitted_by_comma::StringsDeserializedFromStringSplittedByComma>,
    pub color: Option<crate::server::routes::helpers::strings_deserialized_from_string_splitted_by_comma::StringsDeserializedFromStringSplittedByComma>,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum DeleteQueryTryFromUrlEncodingErrorNamed {
    IdIsEmpty {
        #[eo_display_with_serialize_deserialize]
        id_is_empty: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BigserialTryFromStr {
        #[eo_error_occurence]
        bigserial_try_from_str: crate::server::postgres::bigserial::BigserialTryFromStrErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
//
// impl std::convert::TryFrom<DeleteQueryForUrlEncoding> for DeleteQuery {
//     type Error = DeleteQueryTryFromUrlEncodingErrorNamed;
//     fn try_from(value: DeleteQueryForUrlEncoding) -> Result<Self, Self::Error> {
//         let id = {
//             match value.id {
//                 Some(id) => {
//                     let splitted = id.split(',').collect::<Vec<&str>>();
//                     let mut bigserial_vec = Vec::with_capacity(splitted.len());
//                     for splitted_element in splitted {
//                         match crate::server::postgres::bigserial::Bigserial::try_from(
//                             splitted_element,
//                         ) {
//                             Ok(bigserial) => {
//                                 bigserial_vec.push(bigserial);
//                             }
//                             Err(e) => {
//                                 return Err(
//                                     DeleteQueryTryFromUrlEncodingErrorNamed::BigserialTryFromStr {
//                                         bigserial_try_from_str: e,
//                                         code_occurence: crate::code_occurence_tufa_common!(),
//                                     },
//                                 );
//                             }
//                         }
//                     }
//                     match bigserial_vec.is_empty() {
//                         true => {
//                             return Err(DeleteQueryTryFromUrlEncodingErrorNamed::IdIsEmpty {
//                                 id_is_empty: std::string::String::from("id is empty"),
//                                 code_occurence: crate::code_occurence_tufa_common!(),
//                             });
//                         }
//                         false => Some(crate :: server:: postgres :: bigserial_ids :: BigserialIds(bigserial_vec)),
//                     }
//                 }
//                 None => None,
//             }
//         };
//         let name = { value.name };
//         let color = { value.color };
//         Ok(DeleteQuery { id, name, color })
//     }
// }
//
pub async fn delete<'a>(
    query_extraction_result: Result<
        axum::extract::Query<DeleteQuery>,
        axum::extract::rejection::QueryRejection,
    >,
    app_info_state : axum :: extract :: State < crate :: repositories_types ::
tufa_server :: routes :: api :: cats :: DynArcGetConfigGetPostgresPoolSendSync
>,
) -> impl axum::response::IntoResponse {
    let parameters = DeleteParameters {
        query:
            match crate::server::routes::helpers::query_extractor_error::QueryValueResultExtractor::<
                DeleteQuery,
                TryDeleteResponseVariants,
            >::try_extract_value(query_extraction_result, &app_info_state)
            {
                Ok(value) => {
                    println!("value {:#?}", value);
                    value
                },
                Err(err) => {
                    return err;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        if let (None, None, None) = (
            &parameters.query.id,
            &parameters.query.name,
            &parameters.query.color,
        ) {
            return TryDeleteResponseVariants::NoQueryParameters {
                no_query_parameters: std::string::String::from("no query parameters"),
                code_occurence: crate::code_occurence_tufa_common!(),
            };
        }
        match (
            &parameters.query.id,
            &parameters.query.name,
            &parameters.query.color,
        ) {
            (Some(id), None, None) => {
                {
                    // let not_unique_primary_keys = {
                    //     let mut vec = Vec::with_capacity(id.0.len());
                    //     let mut not_unique_primary_keys = Vec::with_capacity(id.0.len());
                    //     for element in id.0 {
                    //         let handle = element.to_inner();
                    //         match vec.contains(&handle) {
                    //             true => {
                    //                 not_unique_primary_keys.push(*element.to_inner());
                    //             }
                    //             false => {
                    //                 vec.push(element.to_inner());
                    //             }
                    //         }
                    //     }
                    //     not_unique_primary_keys
                    // };
                    let not_unique_primary_keys = {
                        let mut vec = Vec::with_capacity(id.0.len());
                        let mut not_unique_primary_keys = Vec::with_capacity(id.0.len());
                        for element in &id.0 {
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
                        let error = TryDelete::NotUniquePrimaryKey {
                            not_unique_primary_keys,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteResponseVariants::from(error);
                    }
                }
                let expected_updated_primary_keys = {
                    id.0.iter()
                        .map(|element| element.to_inner().clone())
                        .collect::<Vec<i64>>()
                };
                let binded_query = {
                    let query_string =
                        { "delete from cats where id in (select unnest($1)) returning id" };
                    println!("{}", query_string);
                    let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                    query = query.bind(
                        id.0.clone().into_iter()
                            .map(|element| element.clone().into_inner())
                            .collect::<Vec<i64>>(),
                    );
                    //

                    //
                    query
                };
                let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDelete::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteResponseVariants::from(error);
                    }
                };
                let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDelete::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteResponseVariants::from(error);
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
                        let error = TryDelete::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteResponseVariants::from(error);
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
                                let error = TryDelete::from(e);
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteResponseVariants::from(error);
                            }
                            Err(rollback_error) => {
                                let error = TryDelete::QueryAndRollbackFailed {
                                    query_error: e,
                                    rollback_error,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteResponseVariants::from(error);
                            }
                        }
                    }
                    results_vec
                };
                let primary_key_vec =
                    {
                        let mut primary_key_vec =
                            Vec::with_capacity(expected_updated_primary_keys.len());
                        for element in results_vec {
                            match primary_key_try_from_sqlx_row(&element) {
                                Ok(primary_key) => {
                                    primary_key_vec.push(primary_key);
                                }
                                Err(e) => match postgres_transaction.rollback().await {
                                    Ok(_) => {
                                        let error = TryDelete::from(e);
                                        crate :: common ::
                                    error_logs_logic :: error_log :: ErrorLog ::
                                    error_log(& error, app_info_state.as_ref(),) ;
                                        return TryDeleteResponseVariants::from(error);
                                    }
                                    Err(rollback_error) => {
                                        let error = TryDelete::PrimaryKeyFromRowAndFailedRollback {
                                            primary_key_from_row: e,
                                            rollback_error,
                                            code_occurence: crate::code_occurence_tufa_common!(),
                                        };
                                        crate :: common :: error_logs_logic :: error_log ::
                                    ErrorLog :: error_log(& error, app_info_state.as_ref(),) ;
                                        return TryDeleteResponseVariants::from(error);
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
                                let error = TryDelete::NonExistingPrimaryKeys {
                                    non_existing_primary_keys,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteResponseVariants::from(error);
                            }
                            Err(e) => {
                                let error = TryDelete::NonExistingPrimaryKeysAndFailedRollback {
                                    non_existing_primary_keys,
                                    rollback_error: e,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteResponseVariants::from(error);
                            }
                        }
                    }
                }
                match postgres_transaction.commit().await {
                    Ok(_) => TryDeleteResponseVariants::Desirable(()),
                    Err(e) => {
                        let error = TryDelete::CommitFailed {
                            commit_error: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        TryDeleteResponseVariants::from(error)
                    }
                }
            }
            _ => {
                if let Some(id) = &parameters.query.id {
                    let not_unique_primary_keys = {
                        let mut vec = Vec::with_capacity(id.0.len());
                        let mut not_unique_primary_keys = Vec::with_capacity(id.0.len());
                        for element in &id.0 {
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
                        let error = TryDelete::NotUniquePrimaryKey {
                            not_unique_primary_keys,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteResponseVariants::from(error);
                    }
                }
                // let name_handle = match parameters.query.name {
                //     Some(value) => {
                //         let is_unique = {
                //             let mut vec = Vec::with_capacity(value.len());
                //             let mut is_unique = true;
                //             for element in &value {
                //                 match vec.contains(&element) {
                //                     true => {
                //                         is_unique = false;
                //                         break;
                //                     }
                //                     false => {
                //                         vec.push(element);
                //                     }
                //                 }
                //             }
                //             is_unique
                //         };
                //         match is_unique {
                //             true => Some(value),
                //             false => {
                //                 let not_unique_name_vec = {
                //                     let mut vec = Vec::with_capacity(value.len());
                //                     let mut not_unique_name_vec = Vec::with_capacity(value.len());
                //                     for element in value {
                //                         match vec.contains(&element) {
                //                             true => {
                //                                 not_unique_name_vec.push(element);
                //                             }
                //                             false => {
                //                                 vec.push(element);
                //                             }
                //                         }
                //                     }
                //                     not_unique_name_vec
                //                 };
                //                 let error = TryDelete::NotUniqueNameVec {
                //                     not_unique_name_vec,
                //                     code_occurence: crate::code_occurence_tufa_common!(),
                //                 };
                //                 crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                //                     &error,
                //                     app_info_state.as_ref(),
                //                 );
                //                 return TryDeleteResponseVariants::from(error);
                //             }
                //         }
                //     }
                //     None => None,
                // };
                // let color_handle = match parameters.query.color {
                //     Some(value) => {
                //         let is_unique = {
                //             let mut vec = Vec::with_capacity(value.len());
                //             let mut is_unique = true;
                //             for element in &value {
                //                 match vec.contains(&element) {
                //                     true => {
                //                         is_unique = false;
                //                         break;
                //                     }
                //                     false => {
                //                         vec.push(element);
                //                     }
                //                 }
                //             }
                //             is_unique
                //         };
                //         match is_unique {
                //             true => Some(value),
                //             false => {
                //                 let not_unique_color_vec = {
                //                     let mut vec = Vec::with_capacity(value.len());
                //                     let mut not_unique_color_vec = Vec::with_capacity(value.len());
                //                     for element in value {
                //                         match vec.contains(&element) {
                //                             true => {
                //                                 not_unique_color_vec.push(element);
                //                             }
                //                             false => {
                //                                 vec.push(element);
                //                             }
                //                         }
                //                     }
                //                     not_unique_color_vec
                //                 };
                //                 let error = TryDelete::NotUniqueColorVec {
                //                     not_unique_color_vec,
                //                     code_occurence: crate::code_occurence_tufa_common!(),
                //                 };
                //                 crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                //                     &error,
                //                     app_info_state.as_ref(),
                //                 );
                //                 return TryDeleteResponseVariants::from(error);
                //             }
                //         }
                //     }
                //     None => None,
                // };
                let query_string = {
                    format!("delete from cats where {}", {
                        let mut increment: u64 = 0;
                        let mut additional_parameters = std::string::String::default();
                        if let Some(value) = &parameters.query.name {
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
                                    return TryDeleteResponseVariants::BindQuery {
                                        checked_add: e.into_serialize_deserialize_version(),
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    };
                                }
                            }
                        }
                        if let Some(value) = &parameters.query.color {
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
                                    return TryDeleteResponseVariants::BindQuery {
                                        checked_add: e.into_serialize_deserialize_version(),
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    };
                                }
                            }
                        }
                        if let Some(id) = &parameters.query.id {
                            if let false = additional_parameters.is_empty() {
                                additional_parameters.push_str(" and");
                            }
                            additional_parameters.push_str(& format!
                            (" id in ({})",
                            {
                                let mut additional_parameters = std :: string :: String ::
                                default() ; for element in &id.0
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
                                            return TryDeleteResponseVariants :: BindQuery
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
                    if let Some(value) = parameters.query.name {
                        query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                            value, query,
                        );
                    }
                    if let Some(value) = parameters.query.color {
                        query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                            value, query,
                        );
                    }
                    if let Some(id) = parameters.query.id {
                        for element in id.0 {
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
                        let error = TryDelete::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteResponseVariants::from(error);
                    }
                };
                let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDelete::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteResponseVariants::from(error);
                    }
                };
                match binded_query.execute(pg_connection.as_mut()).await {
                    Ok(_) => TryDeleteResponseVariants::Desirable(()),
                    Err(e) => {
                        let error = TryDelete::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteResponseVariants::from(error);
                    }
                }
            }
        }
    }
}
////-------------
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct
ReadQuery
{
    pub select : Option < CatColumnSelect >, 
    pub id : Option < crate :: server
    :: postgres :: bigserial_ids :: BigserialIds >, 
    pub name : Option < crate
    :: server :: routes :: helpers ::
    strings_deserialized_from_string_splitted_by_comma ::
    StringsDeserializedFromStringSplittedByComma >, 
    pub color : Option < crate
    :: server :: routes :: helpers ::
    strings_deserialized_from_string_splitted_by_comma ::
    StringsDeserializedFromStringSplittedByComma >, 
    pub order_by : Option <
    CatOrderByWrapper >, pub limit : crate :: server :: postgres ::
    postgres_bigint :: PostgresBigint, pub offset : Option < crate :: server
    :: postgres :: postgres_bigint :: PostgresBigint >,
}
//

//
pub async fn read(
    query_extraction_result: Result<
        axum::extract::Query<ReadQuery>,
        axum::extract::rejection::QueryRejection,
    >,
    app_info_state : axum ::
extract :: State < crate :: repositories_types :: tufa_server :: routes :: api
:: cats :: DynArcGetConfigGetPostgresPoolSendSync >,
) -> impl axum::response::IntoResponse {
    let parameters = ReadParameters {
        query:
            match crate::server::routes::helpers::query_extractor_error::QueryValueResultExtractor::<
                ReadQuery,
                TryReadResponseVariants,
            >::try_extract_value(query_extraction_result, &app_info_state)
            {
                Ok(value) => value,
                Err(err) => {
                    return err;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        if let Some(id) = &parameters.query.id {
            let not_unique_primary_keys = {
                let mut vec = Vec::with_capacity(id.0.len());
                let mut not_unique_primary_keys = Vec::with_capacity(id.0.len());
                for element in &id.0 {
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
                let error = TryRead::NotUniquePrimaryKey {
                    not_unique_primary_keys,
                    code_occurence: crate::code_occurence_tufa_common!(),
                };
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryReadResponseVariants::from(error);
            }
        }
        let select = CatColumnSelect::from(parameters.query.select.clone());
        let query_string = {
            format!(
                "select {} from cats {}",
                crate::server::postgres::generate_query::GenerateQuery::generate_query(&select),
                {
                    let mut increment: u64 = 0;
                    let mut additional_parameters = std::string::String::default();
                    if let Some(value) = &parameters.query.id {
                        let prefix = match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        };
                        match increment.checked_add(1) {
                            Some(value) => {
                                increment = value;
                            }
                            None => {
                                let e = crate :: server :: postgres :: bind_query ::
                            TryGenerateBindIncrementsErrorNamed :: CheckedAdd
                            {
                                checked_add : std :: string :: String ::
                                from("checked_add is None"), code_occurence : crate ::
                                code_occurence_tufa_common! (),
                            } ;
                                return TryReadResponseVariants::BindQuery {
                                    checked_add: e.into_serialize_deserialize_version(),
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                            }
                        }
                        additional_parameters
                            .push_str(&format!("{} id in (select unnest(${}))", prefix, increment));
                    }
                    if let Some(value) = &parameters.query.name {
                        let prefix = match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        };
                        let value = match crate :: server :: postgres :: bind_query :: BindQuery ::try_generate_bind_increments(value, & mut increment)
                    {
                        Ok(value) => value, Err(e) =>
                        {
                            return TryReadResponseVariants :: BindQuery
                            {
                                checked_add : e.into_serialize_deserialize_version(),
                                code_occurence : crate :: code_occurence_tufa_common! ()
                            };
                        },
                    };
                        additional_parameters
                            .push_str(&format!("{prefix} name = any(array[{value}])"));
                    }
                    if let Some(value) = &parameters.query.color {
                        let prefix = match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        };
                        let value = match
                    crate :: server :: postgres :: bind_query :: BindQuery ::
                    try_generate_bind_increments(value, & mut increment)
                    {
                        Ok(value) => value, Err(e) =>
                        {
                            return TryReadResponseVariants :: BindQuery
                            {
                                checked_add : e.into_serialize_deserialize_version(),
                                code_occurence : crate :: code_occurence_tufa_common! ()
                            } ;
                        },
                    } ;
                        additional_parameters
                            .push_str(&format!("{prefix} color = any(array[{value}])"));
                    }
                    if let Some(value) = &parameters.query.order_by {
                        let prefix = match additional_parameters.is_empty() {
                            true => "",
                            false => " ",
                        };
                        let order_stringified = match &value.0.order {
                            Some(order) => order.to_string(),
                            None => crate::server::postgres::order::Order::default().to_string(),
                        };
                        additional_parameters.push_str(&format!(
                            "{}order by {} {}",
                            prefix, value.0.column, order_stringified
                        ));
                    }
                    {
                        let prefix = match additional_parameters.is_empty() {
                            true => "",
                            false => " ",
                        };
                        let value = match crate ::
                    server :: postgres :: bind_query :: BindQuery ::
                    try_generate_bind_increments(& parameters.query.limit, & mut
                    increment)
                    {
                        Ok(value) => value, Err(e) =>
                        {
                            return TryReadResponseVariants :: BindQuery
                            {
                                checked_add : e.into_serialize_deserialize_version(),
                                code_occurence : crate :: code_occurence_tufa_common! ()
                            } ;
                        },
                    } ;
                        additional_parameters.push_str(&format!("{}limit {}", prefix, value));
                    }
                    if let Some(value) = &parameters.query.offset {
                        let prefix = match additional_parameters.is_empty() {
                            true => "",
                            false => " ",
                        };
                        let value = match crate ::
                    server :: postgres :: bind_query :: BindQuery ::
                    try_generate_bind_increments(value, & mut increment)
                    {
                        Ok(value) => value, Err(e) =>
                        {
                            return TryReadResponseVariants :: BindQuery
                            {
                                checked_add : e.into_serialize_deserialize_version(),
                                code_occurence : crate :: code_occurence_tufa_common! ()
                            } ;
                        },
                    } ;
                        additional_parameters.push_str(&format!("{}offset {}", prefix, value));
                    }
                    additional_parameters
                }
            )
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            if let Some(value) = parameters.query.id {
                query = query.bind(
                    value
                        .0
                        .into_iter()
                        .map(|element| element.clone().into_inner())
                        .collect::<Vec<i64>>(),
                );
            }
            if let Some(value) = parameters.query.name {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            if let Some(value) = parameters.query.color {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                parameters.query.limit,
                query,
            );
            if let Some(value) = parameters.query.offset {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            query
        };
        let vec_values = {
            let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
                Ok(value) => value,
                Err(e) => {
                    let error = TryRead::from(e);
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryReadResponseVariants::from(error);
                }
            };
            let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                Ok(value) => value,
                Err(e) => {
                    let error = TryRead::from(e);
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryReadResponseVariants::from(error);
                }
            };
            let mut rows = binded_query.fetch(pg_connection.as_mut());
            let mut vec_values = Vec::new();
            while let Some(row) = {
                match {
                    use futures::TryStreamExt;
                    rows.try_next()
                }
                .await
                {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryRead::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryReadResponseVariants::from(error);
                    }
                }
            } {
                match select.options_try_from_sqlx_row(&row) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        let error = TryRead::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryReadResponseVariants::from(error);
                    }
                }
            }
            vec_values
        };
        TryReadResponseVariants::Desirable(vec_values)
    }
}