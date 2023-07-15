#[derive(serde::Deserialize)]
pub struct GetQueryParameters {
    pub limit: Option<crate::server::postgres::rows_per_table::RowsPerTable>,
    pub name: Option<String>,
    pub color: Option<String>,
}
//todo - make a macro for it?
impl crate::common::url_encode::UrlEncode for GetQueryParameters {
    fn url_encode(&self) -> String {
        let parameters = match (&self.limit, &self.name, &self.color) {
            (None, None, None) => String::from(""),
            (None, None, Some(color)) => format!("color={}", urlencoding::encode(color)),
            (None, Some(name), None) => format!("name={}", urlencoding::encode(name)),
            (None, Some(name), Some(color)) => format!(
                "name={}&color={}",
                urlencoding::encode(name),
                urlencoding::encode(color)
            ),
            (Some(limit), None, None) => format!("limit={limit}"),
            (Some(limit), None, Some(color)) => format!(
                "limit={}&color={}",
                urlencoding::encode(&limit.to_string()),
                urlencoding::encode(color)
            ),
            (Some(limit), Some(name), None) => format!(
                "limit={}&name={}",
                urlencoding::encode(&limit.to_string()),
                urlencoding::encode(name)
            ),
            (Some(limit), Some(name), Some(color)) => {
                format!(
                    "limit={}&name={}&color={}",
                    urlencoding::encode(&limit.to_string()),
                    urlencoding::encode(name),
                    urlencoding::encode(color)
                )
            }
        };
        match parameters.is_empty() {
            true => String::from(""),
            false => format!("?{parameters}"),
        }
    }
}

/////////////////////////////////
pub type GetDesirableType = Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>;

#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::FromEnumWithLifetime,
    type_variants_from_reqwest_response::EnumStatusCodesChecker,
)]
#[type_variants_from_reqwest_response::from_enum_paths_with_lifetime(TryGetResponseVariants)] //todo maybe add lifetime here ?// GetHttpResponse
#[type_variants_from_reqwest_response::enum_status_codes_checker_from(
    crate::repositories_types::tufa_server::routes::api::cats::get::TryGet
)]
pub enum GetErrorNamed<'a> {
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
///////////////////////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    // type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
//     crate::repositories_types::tufa_server::routes::api::cats::get::GetDesirableType,
//     tvfrr_200_ok
// )]
pub enum TryGet<'a> {
    //#[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: &'a str,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //
    //#[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[non_exhaustive] case
    //#[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn try_get<'a>(
    server_location: &str,
    query_parameters: crate::repositories_types::tufa_server::routes::api::cats::get::GetQueryParameters,
) -> Result<
    crate::repositories_types::tufa_server::routes::api::cats::get::GetDesirableType,
    TryGetErrorNamed<'a>,
> {
    get_extraction_logic(
        reqwest::Client::new()
            .get(&format!(
                "{server_location}/api/{}/{}",
                crate::repositories_types::tufa_server::routes::api::cats::CATS,
                crate::common::url_encode::UrlEncode::url_encode(&query_parameters)
            ))
            .header(
                crate::common::git::project_git_info::PROJECT_COMMIT,
                crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                    .project_commit,
            )
            .send(),
    )
    .await
}

#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    into_actix_web_http_response::IntoActixWebHttpResponse,
)]
pub enum TryGetResponseVariants {
    DesirableType(crate::repositories_types::tufa_server::routes::api::cats::get::GetDesirableType),
    ProjectCommitExtractorNotEqual {
        project_commit_not_equal: std::string::String,
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    ProjectCommitExtractorToStrConversion {
        project_commit_to_str_conversion: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    NoProjectCommitExtractorHeader {
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
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
impl std::convert::From<&TryGetResponseVariants> for http::StatusCode {
    fn from(value: &TryGetResponseVariants) -> Self {
        match value {
            TryGetResponseVariants::DesirableType(_) => http::StatusCode::OK,
            TryGetResponseVariants::ProjectCommitExtractorNotEqual {
                project_commit_not_equal: _,
                project_commit_to_use: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            TryGetResponseVariants::ProjectCommitExtractorToStrConversion {
                project_commit_to_str_conversion: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            TryGetResponseVariants::NoProjectCommitExtractorHeader {
                no_project_commit_header: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            TryGetResponseVariants::Configuration {
                configuration_box_dyn_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            TryGetResponseVariants::Database {
                box_dyn_database_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            TryGetResponseVariants::Io {
                io_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            TryGetResponseVariants::Tls {
                box_dyn_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            TryGetResponseVariants::Protocol {
                protocol: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            TryGetResponseVariants::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => http::StatusCode::NOT_FOUND,
            TryGetResponseVariants::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            TryGetResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            TryGetResponseVariants::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            TryGetResponseVariants::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            TryGetResponseVariants::Decode {
                decode_box_dyn_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            TryGetResponseVariants::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => http::StatusCode::REQUEST_TIMEOUT,
            TryGetResponseVariants::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            TryGetResponseVariants::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            TryGetResponseVariants::Migrate {
                migrate: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            TryGetResponseVariants::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryGetResponseVariantsTvfrr500InternalServerError {
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
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
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
impl std::convert::From<TryGetResponseVariantsTvfrr500InternalServerError>
    for TryGetResponseVariants
{
    fn from(value: TryGetResponseVariantsTvfrr500InternalServerError) -> Self {
        match value {
            TryGetResponseVariantsTvfrr500InternalServerError::Configuration {
                configuration_box_dyn_error,
                code_occurence,
            } => Self::Configuration {
                configuration_box_dyn_error,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr500InternalServerError::Database {
                box_dyn_database_error,
                code_occurence,
            } => Self::Database {
                box_dyn_database_error,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr500InternalServerError::Io {
                io_error,
                code_occurence,
            } => Self::Io {
                io_error,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr500InternalServerError::Tls {
                box_dyn_error,
                code_occurence,
            } => Self::Tls {
                box_dyn_error,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr500InternalServerError::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr500InternalServerError::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr500InternalServerError::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr500InternalServerError::Decode {
                decode_box_dyn_error,
                code_occurence,
            } => Self::Decode {
                decode_box_dyn_error,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr500InternalServerError::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr500InternalServerError::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr500InternalServerError::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr500InternalServerError::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryGetResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
}
impl std::convert::From<TryGetResponseVariantsTvfrr404NotFound> for TryGetResponseVariants {
    fn from(value: TryGetResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryGetResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryGetResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
}
impl std::convert::From<TryGetResponseVariantsTvfrr408RequestTimeout> for TryGetResponseVariants {
    fn from(value: TryGetResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryGetResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryGetResponseVariantsTvfrr400BadRequest {
    ProjectCommitExtractorNotEqual {
        project_commit_not_equal: std::string::String,
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    ProjectCommitExtractorToStrConversion {
        project_commit_to_str_conversion: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    NoProjectCommitExtractorHeader {
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
}
impl std::convert::From<TryGetResponseVariantsTvfrr400BadRequest> for TryGetResponseVariants {
    fn from(value: TryGetResponseVariantsTvfrr400BadRequest) -> Self {
        match value {
            TryGetResponseVariantsTvfrr400BadRequest::ProjectCommitExtractorNotEqual {
                project_commit_not_equal,
                project_commit_to_use,
                code_occurence,
            } => Self::ProjectCommitExtractorNotEqual {
                project_commit_not_equal,
                project_commit_to_use,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr400BadRequest::ProjectCommitExtractorToStrConversion {
                project_commit_to_str_conversion,
                code_occurence,
            } => Self::ProjectCommitExtractorToStrConversion {
                project_commit_to_str_conversion,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr400BadRequest::NoProjectCommitExtractorHeader {
                no_project_commit_header,
                code_occurence,
            } => Self::NoProjectCommitExtractorHeader {
                no_project_commit_header,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr400BadRequest::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            TryGetResponseVariantsTvfrr400BadRequest::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde:: Serialize, serde :: Deserialize)]
enum TryGetResponseVariantsTvfrr200Ok {
    DesirableType(crate::repositories_types::tufa_server::routes::api::cats::get::GetDesirableType),
}
impl std::convert::From<TryGetResponseVariantsTvfrr200Ok> for TryGetResponseVariants {
    fn from(value: TryGetResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryGetResponseVariantsTvfrr200Ok::DesirableType(i) => Self::DesirableType(i),
        }
    }
}
impl std::convert::TryFrom<reqwest::Response> for TryGetResponseVariants {
    type Error = crate::common::api_request_unexpected_error::ApiRequestUnexpectedError;
    fn try_from(response: reqwest::Response) -> Result<Self, Self::Error> {
        let status_code = response.status();
        let headers = response.headers().clone();
        let response_text = futures::executor::block_on(response.text()).unwrap_or_else(|_| {
            std::string::String::from(
                crate::global_variables::hardcode::FAILED_TO_GET_RESPONSE_TEXT,
            )
        });
        if status_code == http::StatusCode::OK {
            match serde_json :: from_str :: < TryGetResponseVariantsTvfrr200Ok
            > (& response_text)
            {
                Ok(value) => Ok(TryGetResponseVariants :: from(value)), Err(e)
                =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }
        } else if status_code == http::StatusCode::BAD_REQUEST {
            match serde_json :: from_str :: <
            TryGetResponseVariantsTvfrr400BadRequest > (& response_text)
            {
                Ok(value) => Ok(TryGetResponseVariants :: from(value)), Err(e)
                =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text },),
            }
        } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
            match serde_json :: from_str :: <
            TryGetResponseVariantsTvfrr500InternalServerError >
            (& response_text)
            {
                Ok(value) => Ok(TryGetResponseVariants :: from(value)), Err(e)
                =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text },),
            }
        } else if status_code == http::StatusCode::NOT_FOUND {
            match serde_json :: from_str :: <
            TryGetResponseVariantsTvfrr404NotFound > (& response_text)
            {
                Ok(value) => Ok(TryGetResponseVariants :: from(value)), Err(e)
                =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text },),
            }
        } else {
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            { status_code, headers, response_text },)
        }
    }
}
impl TryFrom<TryGetResponseVariants>
    for crate::repositories_types::tufa_server::routes::api::cats::get::GetDesirableType
{
    type Error = TryGetWithSerializeDeserialize;
    fn try_from(value: TryGetResponseVariants) -> Result<Self, Self::Error> {
        match value {
            TryGetResponseVariants::DesirableType(i) => Ok(i),
            TryGetResponseVariants::ProjectCommitExtractorNotEqual {
                project_commit_not_equal,
                project_commit_to_use,
                code_occurence,
            } => Err(
                TryGetWithSerializeDeserialize::ProjectCommitExtractorNotEqual {
                    project_commit_not_equal,
                    project_commit_to_use,
                    code_occurence,
                },
            ),
            TryGetResponseVariants::ProjectCommitExtractorToStrConversion {
                project_commit_to_str_conversion,
                code_occurence,
            } => Err(
                TryGetWithSerializeDeserialize::ProjectCommitExtractorToStrConversion {
                    project_commit_to_str_conversion,
                    code_occurence,
                },
            ),
            TryGetResponseVariants::NoProjectCommitExtractorHeader {
                no_project_commit_header,
                code_occurence,
            } => Err(
                TryGetWithSerializeDeserialize::NoProjectCommitExtractorHeader {
                    no_project_commit_header,
                    code_occurence,
                },
            ),
            TryGetResponseVariants::Configuration {
                configuration_box_dyn_error,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::Configuration {
                configuration_box_dyn_error,
                code_occurence,
            }),
            TryGetResponseVariants::Database {
                box_dyn_database_error,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::Database {
                box_dyn_database_error,
                code_occurence,
            }),
            TryGetResponseVariants::Io {
                io_error,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::Io {
                io_error,
                code_occurence,
            }),
            TryGetResponseVariants::Tls {
                box_dyn_error,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::Tls {
                box_dyn_error,
                code_occurence,
            }),
            TryGetResponseVariants::Protocol {
                protocol,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            }),
            TryGetResponseVariants::RowNotFound {
                row_not_found,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            }),
            TryGetResponseVariants::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            }),
            TryGetResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            }),
            TryGetResponseVariants::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            }),
            TryGetResponseVariants::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            }),
            TryGetResponseVariants::Decode {
                decode_box_dyn_error,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::Decode {
                decode_box_dyn_error,
                code_occurence,
            }),
            TryGetResponseVariants::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            }),
            TryGetResponseVariants::PoolClosed {
                pool_closed,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            }),
            TryGetResponseVariants::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            }),
            TryGetResponseVariants::Migrate {
                migrate,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            }),
            TryGetResponseVariants::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Err(TryGetWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            }),
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryGetErrorNamed<'a> {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        get: TryGetWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
async fn get_extraction_logic<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<
    crate::repositories_types::tufa_server::routes::api::cats::get::GetDesirableType,
    TryGetErrorNamed<'a>,
> {
    match future.await
    {
        Ok(response) => match TryGetResponseVariants :: try_from(response)
        {
            Ok(variants) => match crate :: repositories_types :: tufa_server
            :: routes :: api :: cats :: get :: GetDesirableType ::
            try_from(variants)
            {
                Ok(value) => Ok(value), Err(e) =>
                Err(TryGetErrorNamed :: ExpectedType
                {
                    get : e, code_occurence : crate ::
                    code_occurence_tufa_common! (),
                }),
            }, Err(e) => match e
            {
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: StatusCode
                { status_code, headers, response_text, } =>
                Err(TryGetErrorNamed :: UnexpectedStatusCode
                {
                    status_code, headers, response_text, code_occurence : crate
                    :: code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryGetErrorNamed :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }),
            },
        }, Err(e) =>
        Err(TryGetErrorNamed :: Reqwest
        {
            reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
            (),
        }),
    }
}
pub enum TryGetStatusCodesChecker {
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
}
