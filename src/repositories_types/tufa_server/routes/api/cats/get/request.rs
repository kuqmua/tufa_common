#[derive(Debug, serde::Serialize, serde::Deserialize, type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse)]
pub enum GetHttpResponseVariants {
    #[tvtrr_desirable_type]
    #[tvfrr_200_ok]
    Cats(Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>),
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        project_commit_not_equal: std::string::String,
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        project_commit_to_str_conversion: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //
    #[tvfrr_500_internal_server_error]
    Configuration {
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        io_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        migrate: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
}
//todo what if not going to use upper enum but this? for status code logic
#[derive(
    Debug, thiserror::Error, error_occurence::ErrorOccurence, from_enum::FromEnumWithLifetime,
)]
#[from_enum::from_enum_paths_with_lifetime(GetHttpResponseVariants)]
pub enum TryGetHttpResponseVariants<'a> {
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: &'a str,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //
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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGetErrorNamed<'a> {
    TryFrom {
        #[eo_error_occurence]
        try_from: TryFromGetErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryFromGetErrorNamed<'a> {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        get: TryGetHttpResponseVariantsWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    UnexpectedStatusCode { 
        #[eo_display]
        status_code: http::StatusCode,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    DeserializeResponse {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub struct ExpectedGetType {
    pub value: Vec::<crate::repositories_types::tufa_server::routes::api::cats::Cat>
}

#[allow(clippy::result_large_err)]
impl<'a> ExpectedGetType {
    fn try_from(
        value: reqwest::Response,
    ) -> Result<Self, TryFromGetErrorNamed<'a>> {
        match GetHttpResponseVariants::try_from(value) {
            Ok(variants) => match Vec::<
                crate::repositories_types::tufa_server::routes::api::cats::Cat,
            >::try_from(variants)
            {
                Ok(value) => Ok(Self { value }),
                Err(e) => Err(TryFromGetErrorNamed::ExpectedType {
                    get: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            },
            Err(e) => match e {//todo impl from?
                crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::StatusCode { status_code } => Err(
                    TryFromGetErrorNamed::UnexpectedStatusCode { 
                        status_code, 
                        code_occurence: crate::code_occurence_tufa_common!() 
                    }
                ),
                crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::DeserializeBody { 
                    reqwest, 
                    status_code 
                } => Err(
                    TryFromGetErrorNamed::DeserializeResponse { 
                        reqwest, 
                        status_code, 
                        code_occurence: crate::code_occurence_tufa_common!() 
                    }
                ),
            },
        }
    }
}

async fn get_extraction_logic<'a>(future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>) -> Result<Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>, TryGetErrorNamed<'a>> {
    match future.await {
        Ok(response) => match ExpectedGetType::try_from(response) {
            Ok(exp) => Ok(exp.value),
            Err(e) => Err(TryGetErrorNamed::TryFrom { 
                try_from: e, 
                code_occurence: crate::code_occurence_tufa_common!() 
            }),
        },
        Err(e) => Err(TryGetErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}

pub async fn try_get<'a>(
    server_location: std::string::String, //todo server_location: std::string::String, 0 maybe change it to ip port
    query_parameters: crate::repositories_types::tufa_server::routes::api::cats::get::GetQueryParameters,
) -> Result<Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>, TryGetErrorNamed<'a>>
{
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
        .send()
    ).await
}