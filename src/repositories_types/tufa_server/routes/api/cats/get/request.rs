#[derive(Debug, serde::Serialize, serde::Deserialize
    // , type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse
)]
pub enum GetHttpResponseVariants {
    //#[tvfrr_desirable_type]
    //#[tvfrr_200_ok]
    Cats(Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>),
    //#[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        project_commit_not_equal: std::string::String,
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        project_commit_to_str_conversion: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //
    //#[tvfrr_500_internal_server_error]
    Configuration {
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_500_internal_server_error]
    Database {
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_500_internal_server_error]
    Io {
        io_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_500_internal_server_error]
    Tls {
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_500_internal_server_error]
    Protocol {
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_404_not_found]
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_400_bad_request]
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_400_bad_request]
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_500_internal_server_error]
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_500_internal_server_error]
    Decode {
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_408_request_timeout]
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_500_internal_server_error]
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_500_internal_server_error]
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_500_internal_server_error]
    Migrate {
        migrate: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //#[tvfrr_500_internal_server_error]
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
}
//todo what if not going to use upper enum but this? for status code logic
#[derive(
    Debug, thiserror::Error, error_occurence::ErrorOccurence, from_enum::FromEnumWithLifetime, type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseHandle
)]
#[from_enum::from_enum_paths_with_lifetime(GetHttpResponseVariants)]//Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>, http::StatusCode::OK
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
        Ok(response) => match GetHttpResponseVariants::try_from(response) {
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
impl std :: convert :: From < & GetHttpResponseVariants > for http ::
StatusCode
{
    fn from(value : & GetHttpResponseVariants) -> Self
    {
        match value
        {
            GetHttpResponseVariants :: Cats(_) => http :: StatusCode :: OK,
            GetHttpResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } => http :: StatusCode :: BAD_REQUEST, GetHttpResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            http :: StatusCode :: BAD_REQUEST, GetHttpResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } => http ::
            StatusCode :: BAD_REQUEST, GetHttpResponseVariants ::
            Configuration
            { configuration_box_dyn_error : _, code_occurence : _ } => http ::
            StatusCode :: INTERNAL_SERVER_ERROR, GetHttpResponseVariants ::
            Database { box_dyn_database_error : _, code_occurence : _ } =>
            http :: StatusCode :: INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants :: Io { io_error : _, code_occurence : _ }
            => http :: StatusCode :: INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants :: Tls
            { box_dyn_error : _, code_occurence : _ } => http :: StatusCode ::
            INTERNAL_SERVER_ERROR, GetHttpResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => http :: StatusCode ::
            INTERNAL_SERVER_ERROR, GetHttpResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
            NOT_FOUND, GetHttpResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => http :: StatusCode
            :: BAD_REQUEST, GetHttpResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            http :: StatusCode :: INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } => http :: StatusCode
            :: BAD_REQUEST, GetHttpResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => http :: StatusCode :: INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants :: Decode
            { decode_box_dyn_error : _, code_occurence : _ } => http ::
            StatusCode :: INTERNAL_SERVER_ERROR, GetHttpResponseVariants ::
            PoolTimedOut { pool_timed_out : _, code_occurence : _ } => http ::
            StatusCode :: REQUEST_TIMEOUT, GetHttpResponseVariants ::
            PoolClosed { pool_closed : _, code_occurence : _ } => http ::
            StatusCode :: INTERNAL_SERVER_ERROR, GetHttpResponseVariants ::
            WorkerCrashed { worker_crashed : _, code_occurence : _ } => http
            :: StatusCode :: INTERNAL_SERVER_ERROR, GetHttpResponseVariants ::
            Migrate { migrate : _, code_occurence : _ } => http :: StatusCode
            :: INTERNAL_SERVER_ERROR, GetHttpResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => http
            :: StatusCode :: INTERNAL_SERVER_ERROR
        }
    }
} #[derive(Debug, serde :: Serialize, serde :: Deserialize)] enum
GetHttpResponseVariantsTvfrr200Ok
{
    Cats(Vec < crate :: repositories_types :: tufa_server :: routes :: api ::
    cats :: Cat >)
} impl std :: convert :: From < GetHttpResponseVariantsTvfrr200Ok > for
GetHttpResponseVariants
{
    fn from(value : GetHttpResponseVariantsTvfrr200Ok) -> Self
    {
        match value
        { GetHttpResponseVariantsTvfrr200Ok :: Cats(i) => Self :: Cats(i) }
    }
} #[derive(Debug, serde :: Serialize, serde :: Deserialize)] enum
GetHttpResponseVariantsTvfrr400BadRequest
{
    ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std :: string :: String,
        project_commit_to_use : std :: string :: String, code_occurence :
        crate :: common :: code_occurence ::
        CodeOccurenceWithSerializeDeserialize
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std :: string :: String,
        code_occurence : crate :: common :: code_occurence ::
        CodeOccurenceWithSerializeDeserialize
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std :: string :: String, code_occurence :
        crate :: common :: code_occurence ::
        CodeOccurenceWithSerializeDeserialize
    }, TypeNotFound
    {
        type_not_found : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurenceWithSerializeDeserialize
    }, ColumnNotFound
    {
        column_not_found : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurenceWithSerializeDeserialize
    }
} impl std :: convert :: From < GetHttpResponseVariantsTvfrr400BadRequest >
for GetHttpResponseVariants
{
    fn from(value : GetHttpResponseVariantsTvfrr400BadRequest) -> Self
    {
        match value
        {
            GetHttpResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, GetHttpResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            GetHttpResponseVariantsTvfrr400BadRequest ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence },
            GetHttpResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            GetHttpResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence }
        }
    }
} #[derive(Debug, serde :: Serialize, serde :: Deserialize)] enum
GetHttpResponseVariantsTvfrr500InternalServerError
{
    Configuration
    {
        configuration_box_dyn_error : std :: string :: String, code_occurence
        : crate :: common :: code_occurence ::
        CodeOccurenceWithSerializeDeserialize
    }, Database
    {
        box_dyn_database_error : std :: string :: String, code_occurence :
        crate :: common :: code_occurence ::
        CodeOccurenceWithSerializeDeserialize
    }, Io
    {
        io_error : std :: string :: String, code_occurence : crate :: common
        :: code_occurence :: CodeOccurenceWithSerializeDeserialize
    }, Tls
    {
        box_dyn_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurenceWithSerializeDeserialize
    }, Protocol
    {
        protocol : std :: string :: String, code_occurence : crate :: common
        :: code_occurence :: CodeOccurenceWithSerializeDeserialize
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize, len : usize, code_occurence :
        crate :: common :: code_occurence ::
        CodeOccurenceWithSerializeDeserialize
    }, ColumnDecode
    {
        column_decode_index : std :: string :: String, source_handle : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurenceWithSerializeDeserialize
    }, Decode
    {
        decode_box_dyn_error : std :: string :: String, code_occurence : crate
        :: common :: code_occurence :: CodeOccurenceWithSerializeDeserialize
    }, PoolClosed
    {
        pool_closed : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurenceWithSerializeDeserialize
    }, WorkerCrashed
    {
        worker_crashed : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurenceWithSerializeDeserialize
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurenceWithSerializeDeserialize
    }, UnexpectedCase
    {
        unexpected_case : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurenceWithSerializeDeserialize
    }
} impl std :: convert :: From <
GetHttpResponseVariantsTvfrr500InternalServerError > for
GetHttpResponseVariants
{
    fn from(value : GetHttpResponseVariantsTvfrr500InternalServerError) ->
    Self
    {
        match value
        {
            GetHttpResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration_box_dyn_error, code_occurence } =>
            Self :: Configuration
            { configuration_box_dyn_error, code_occurence },
            GetHttpResponseVariantsTvfrr500InternalServerError :: Database
            { box_dyn_database_error, code_occurence } => Self :: Database
            { box_dyn_database_error, code_occurence },
            GetHttpResponseVariantsTvfrr500InternalServerError :: Io
            { io_error, code_occurence } => Self :: Io
            { io_error, code_occurence },
            GetHttpResponseVariantsTvfrr500InternalServerError :: Tls
            { box_dyn_error, code_occurence } => Self :: Tls
            { box_dyn_error, code_occurence },
            GetHttpResponseVariantsTvfrr500InternalServerError :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            GetHttpResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            GetHttpResponseVariantsTvfrr500InternalServerError :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            GetHttpResponseVariantsTvfrr500InternalServerError :: Decode
            { decode_box_dyn_error, code_occurence } => Self :: Decode
            { decode_box_dyn_error, code_occurence },
            GetHttpResponseVariantsTvfrr500InternalServerError :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            GetHttpResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            GetHttpResponseVariantsTvfrr500InternalServerError :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            GetHttpResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence }
        }
    }
} 


#[derive(Debug, serde :: Serialize, serde :: Deserialize)] enum
GetHttpResponseVariantsTvfrr404NotFound
{
    RowNotFound
    {
        row_not_found : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurenceWithSerializeDeserialize
    }
} 


impl std :: convert :: From < GetHttpResponseVariantsTvfrr404NotFound > for
GetHttpResponseVariants
{
    fn from(value : GetHttpResponseVariantsTvfrr404NotFound) -> Self
    {
        match value
        {
            GetHttpResponseVariantsTvfrr404NotFound :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence }
        }
    }
} #[derive(Debug, serde :: Serialize, serde :: Deserialize)] enum
GetHttpResponseVariantsTvfrr408RequestTimeout
{
    PoolTimedOut
    {
        pool_timed_out : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurenceWithSerializeDeserialize
    }
} 


impl std :: convert :: From < GetHttpResponseVariantsTvfrr408RequestTimeout
> for GetHttpResponseVariants
{
    fn from(value : GetHttpResponseVariantsTvfrr408RequestTimeout) -> Self
    {
        match value
        {
            GetHttpResponseVariantsTvfrr408RequestTimeout :: PoolTimedOut
            { pool_timed_out, code_occurence } => Self :: PoolTimedOut
            { pool_timed_out, code_occurence }
        }
    }
} 


impl std :: convert :: TryFrom < reqwest :: Response > for
GetHttpResponseVariants
{
    type Error = crate :: common :: api_request_unexpected_error ::
    ApiRequestUnexpectedError ; fn try_from(response : reqwest :: Response) ->
    Result < Self, Self :: Error >
    {
        let status_code = response.status() ; if status_code == http ::
        StatusCode :: OK
        {
            match futures :: executor ::
            block_on(response.json :: < GetHttpResponseVariantsTvfrr200Ok >
            ())
            {
                Ok(value) => Ok(GetHttpResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { reqwest : e, status_code, },),
            }
        } else if status_code == http :: StatusCode :: BAD_REQUEST
        {
            match futures :: executor ::
            block_on(response.json :: <
            GetHttpResponseVariantsTvfrr400BadRequest > (),)
            {
                Ok(value) => Ok(GetHttpResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { reqwest : e, status_code, },),
            }
        } else if status_code == http :: StatusCode :: INTERNAL_SERVER_ERROR
        {
            match futures :: executor ::
            block_on(response.json :: <
            GetHttpResponseVariantsTvfrr500InternalServerError > (),)
            {
                Ok(value) => Ok(GetHttpResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { reqwest : e, status_code, },),
            }
        } else if status_code == http :: StatusCode :: NOT_FOUND
        {
            match futures :: executor ::
            block_on(response.json :: <
            GetHttpResponseVariantsTvfrr404NotFound > (),)
            {
                Ok(value) => Ok(GetHttpResponseVariants :: from(value)),
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



impl TryFrom < GetHttpResponseVariants > for Vec < crate::repositories_types::tufa_server::routes::api :: cats :: Cat >
{
    type Error = TryGetHttpResponseVariantsWithSerializeDeserialize ; fn
    try_from(value : GetHttpResponseVariants,) -> Result < Self, Self :: Error
    >
    {
        match value
        {
            GetHttpResponseVariants :: Cats(i) => Ok(i),
            GetHttpResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), GetHttpResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            GetHttpResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }),
            GetHttpResponseVariants :: Configuration
            { configuration_box_dyn_error, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            Configuration { configuration_box_dyn_error, code_occurence }),
            GetHttpResponseVariants :: Database
            { box_dyn_database_error, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize :: Database
            { box_dyn_database_error, code_occurence }),
            GetHttpResponseVariants :: Io { io_error, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize :: Io
            { io_error, code_occurence }), GetHttpResponseVariants :: Tls
            { box_dyn_error, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize :: Tls
            { box_dyn_error, code_occurence }), GetHttpResponseVariants ::
            Protocol { protocol, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), GetHttpResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            RowNotFound { row_not_found, code_occurence }),
            GetHttpResponseVariants :: TypeNotFound
            { type_not_found, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            TypeNotFound { type_not_found, code_occurence }),
            GetHttpResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            GetHttpResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            ColumnNotFound { column_not_found, code_occurence }),
            GetHttpResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            GetHttpResponseVariants :: Decode
            { decode_box_dyn_error, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize :: Decode
            { decode_box_dyn_error, code_occurence }), GetHttpResponseVariants
            :: PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence }),
            GetHttpResponseVariants :: PoolClosed
            { pool_closed, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            PoolClosed { pool_closed, code_occurence }),
            GetHttpResponseVariants :: WorkerCrashed
            { worker_crashed, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            WorkerCrashed { worker_crashed, code_occurence }),
            GetHttpResponseVariants :: Migrate { migrate, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), GetHttpResponseVariants ::
            UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryGetHttpResponseVariantsWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence })
        }
    }
}