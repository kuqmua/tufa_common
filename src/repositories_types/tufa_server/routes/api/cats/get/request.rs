//todo what if not going to use upper enum but this? for status code logic
#[derive(
    Debug, thiserror::Error, error_occurence::ErrorOccurence, from_enum::FromEnumWithLifetime, type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseHandle
)]
#[from_enum::from_enum_paths_with_lifetime(TryGetHttpResponseVariantsResponseVariants)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_handle_attribute(Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>, http::StatusCode::OK)]
pub enum TryGetHttpResponseVariants<'a> {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: &'a str,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //
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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGetErrorNamed<'a> {
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
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    
}

async fn get_extraction_logic<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>
) -> Result<Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>, TryGetErrorNamed<'a>> 
{
    match future.await {
        Ok(response) => match TryGetHttpResponseVariantsResponseVariants::try_from(response) {
            Ok(variants) => match Vec::<
                crate::repositories_types::tufa_server::routes::api::cats::Cat,
            >::try_from(variants)
            {
                Ok(value) => Ok(value),
                Err(e) => Err(TryGetErrorNamed::ExpectedType {
                    get: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            },
            Err(e) => match e {//todo impl from?
                crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::StatusCode { status_code } => Err(
                    TryGetErrorNamed::UnexpectedStatusCode { 
                        status_code, 
                        code_occurence: crate::code_occurence_tufa_common!() 
                    }
                ),
                crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::DeserializeBody { 
                    reqwest, 
                    status_code 
                } => Err(
                    TryGetErrorNamed::DeserializeResponse { 
                        reqwest, 
                        status_code, 
                        code_occurence: crate::code_occurence_tufa_common!() 
                    }
                ),
            },
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

/////////////
#[derive(Debug, serde :: Serialize, serde :: Deserialize)] enum
TryGetHttpResponseVariantsResponseVariantsTvfrr200Ok
{
    DesirableType(Vec < crate :: repositories_types :: tufa_server :: routes :: api ::
    cats :: Cat >)
} 

impl std :: convert :: From < TryGetHttpResponseVariantsResponseVariantsTvfrr200Ok > for
TryGetHttpResponseVariantsResponseVariants
{
    fn from(value : TryGetHttpResponseVariantsResponseVariantsTvfrr200Ok) -> Self
    {
        match value
        { TryGetHttpResponseVariantsResponseVariantsTvfrr200Ok :: DesirableType(i) => Self :: DesirableType(i) }
    }
} 

impl std :: convert :: TryFrom < reqwest :: Response > for
TryGetHttpResponseVariantsResponseVariants
{
    type Error = crate :: common :: api_request_unexpected_error ::
    ApiRequestUnexpectedError ; fn try_from(response : reqwest :: Response) ->
    Result < Self, Self :: Error >
    {
        let status_code = response.status() ; if status_code == http ::
        StatusCode :: OK
        {
            match futures :: executor ::
            block_on(response.json :: < TryGetHttpResponseVariantsResponseVariantsTvfrr200Ok >
            ())
            {
                Ok(value) => Ok(TryGetHttpResponseVariantsResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { reqwest : e, status_code, },),
            }
        } else if status_code == http :: StatusCode :: BAD_REQUEST
        {
            match futures :: executor ::
            block_on(response.json :: <
            TryGetHttpResponseVariantsResponseVariantsTvfrr400BadRequest > (),)
            {
                Ok(value) => Ok(TryGetHttpResponseVariantsResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { reqwest : e, status_code, },),
            }
        } else if status_code == http :: StatusCode :: INTERNAL_SERVER_ERROR
        {
            match futures :: executor ::
            block_on(response.json :: <
            TryGetHttpResponseVariantsResponseVariantsTvfrr500InternalServerError > (),)
            {
                Ok(value) => Ok(TryGetHttpResponseVariantsResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { reqwest : e, status_code, },),
            }
        } else if status_code == http :: StatusCode :: NOT_FOUND
        {
            match futures :: executor ::
            block_on(response.json :: <
            TryGetHttpResponseVariantsResponseVariantsTvfrr404NotFound > (),)
            {
                Ok(value) => Ok(TryGetHttpResponseVariantsResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { reqwest : e, status_code, },),
            }
        } else
        {
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode { status_code, },)
        }
    }
} 



impl TryFrom < TryGetHttpResponseVariantsResponseVariants > for Vec < crate::repositories_types::tufa_server::routes::api :: cats :: Cat >
{
    type Error = TryGetHttpResponseVariantsWithSerializeDeserialize ; fn
    try_from(value : TryGetHttpResponseVariantsResponseVariants,) -> Result < Self, Self :: Error
    >
    {
        match value
        {
            TryGetHttpResponseVariantsResponseVariants :: DesirableType(i) => Ok(i),
            TryGetHttpResponseVariantsResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), TryGetHttpResponseVariantsResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            TryGetHttpResponseVariantsResponseVariants:: NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }),
            TryGetHttpResponseVariantsResponseVariants:: Configuration
            { configuration_box_dyn_error, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            Configuration { configuration_box_dyn_error, code_occurence }),
            TryGetHttpResponseVariantsResponseVariants:: Database
            { box_dyn_database_error, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize :: Database
            { box_dyn_database_error, code_occurence }),
            TryGetHttpResponseVariantsResponseVariants:: Io { io_error, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize :: Io
            { io_error, code_occurence }), TryGetHttpResponseVariantsResponseVariants:: Tls
            { box_dyn_error, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize :: Tls
            { box_dyn_error, code_occurence }), TryGetHttpResponseVariantsResponseVariants::
            Protocol { protocol, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryGetHttpResponseVariantsResponseVariants::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            RowNotFound { row_not_found, code_occurence }),
            TryGetHttpResponseVariantsResponseVariants:: TypeNotFound
            { type_not_found, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            TypeNotFound { type_not_found, code_occurence }),
            TryGetHttpResponseVariantsResponseVariants:: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryGetHttpResponseVariantsResponseVariants:: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            ColumnNotFound { column_not_found, code_occurence }),
            TryGetHttpResponseVariantsResponseVariants:: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryGetHttpResponseVariantsResponseVariants:: Decode
            { decode_box_dyn_error, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize :: Decode
            { decode_box_dyn_error, code_occurence }), TryGetHttpResponseVariantsResponseVariants
            :: PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence }),
            TryGetHttpResponseVariantsResponseVariants:: PoolClosed
            { pool_closed, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            PoolClosed { pool_closed, code_occurence }),
            TryGetHttpResponseVariantsResponseVariants:: WorkerCrashed
            { worker_crashed, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            WorkerCrashed { worker_crashed, code_occurence }),
            TryGetHttpResponseVariantsResponseVariants:: Migrate { migrate, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryGetHttpResponseVariantsResponseVariants::
            UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence })
        }
    }
}