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
#[generate_postgresql_crud::create_many_additional_http_status_codes_error_variants{}]
#[generate_postgresql_crud::create_one_additional_http_status_codes_error_variants{}]
#[generate_postgresql_crud::read_one_additional_http_status_codes_error_variants{}]
#[generate_postgresql_crud::read_many_with_body_additional_http_status_codes_error_variants{}]
#[generate_postgresql_crud::update_one_additional_http_status_codes_error_variants{}]
#[generate_postgresql_crud::update_many_additional_http_status_codes_error_variants{}]
#[generate_postgresql_crud::delete_one_additional_http_status_codes_error_variants{}]
#[generate_postgresql_crud::delete_many_with_body_additional_http_status_codes_error_variants{}]

#[generate_postgresql_crud::additional_http_status_codes_error_variants{
    #[path(crate::server::extractors::project_commit_extractor::)]
    enum ProjectCommitExtractorCheckErrorNamed {
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
    }
    // ;
    // enum SomethingErrorNamed {
    //     #[tvfrr_400_bad_request]
    //     SomethingVariant {
    //         #[eo_display_with_serialize_deserialize]
    //         something_field: std::string::String,
    //         code_occurence: crate::common::code_occurence::CodeOccurence,
    //     },
    // }
}]
pub struct Dog {
    #[generate_postgresql_crud_primary_key]
    pub id: sqlx::types::Uuid, //todo make it UuidWrapper todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    #[generate_postgresql_crud_varchar]
    pub name: std::string::String,
    #[generate_postgresql_crud_varchar]
    pub color: std::string::String,
}

// fn s() {
//     let f = crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed::NoProjectCommitExtractorHeader {
//             no_project_commit_header: std::string::String::from(""),
//             code_occurence: crate::code_occurence_tufa_common!(),
//         };
// }

#[derive(Debug)]
pub struct UpdateOneParameters {
    pub path: UpdateOnePath,
    pub payload: UpdateOnePayload,
}
#[derive(Debug)]
pub struct UpdateOnePath {
    pub id: crate::server::postgres::uuid_wrapper::UuidWrapper,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct UpdateOnePathWithSerializeDeserialize {
    id: crate::server::postgres::uuid_wrapper::PossibleUuidWrapper,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamed {
    NotUuid {
        #[eo_error_occurence]
        not_uuid:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<UpdateOnePathWithSerializeDeserialize> for UpdateOnePath {
    type Error = UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamed;
    fn try_from(value: UpdateOnePathWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let id = match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value.id) {
            Ok(value) => value,
            Err(e) => {
                return Err(Self::Error::NotUuid {
                    not_uuid: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        Ok(Self { id })
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct UpdateOnePayload {
    pub name: std::option::Option<std::string::String>,
    pub color: std::option::Option<std::string::String>,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryUpdateOneErrorNamed {
    RequestError {
        #[eo_error_occurence]
        request_error: TryUpdateOneRequestError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryUpdateOne {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToDeserializePathParams {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingPathParams {
        #[eo_display_with_serialize_deserialize]
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize {
        #[eo_error_occurence]
        update_one_path_try_from_update_one_path_with_serialize_deserialize:
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        #[eo_display]
        uuid_wrapper_try_from_possible_uuid_wrapper_in_server: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateOneResponseVariants {
    Desirable(crate :: server :: postgres :: uuid_wrapper ::
    PossibleUuidWrapper), Configuration
    {
        configuration_box_dyn_error : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Database
    {
        box_dyn_database_error : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Io
    {
        io_error : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Tls
    {
        box_dyn_error : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Protocol
    {
        protocol : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, RowNotFound
    {
        row_not_found : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, TypeNotFound
    {
        type_not_found : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize<>, len : usize<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std::string::String<>, source_handle :
        std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Decode
    {
        decode_box_dyn_error : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, PoolTimedOut
    {
        pool_timed_out : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, PoolClosed
    {
        pool_closed : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Migrate
    {
        migrate : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, FailedToDeserializePathParams
    {
        failed_to_deserialize_path_params : std::string::String<>,
        code_occurence : crate::common::code_occurence::CodeOccurence
    }, MissingPathParams
    {
        missing_path_params : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, JsonDataError
    {
        json_data_error : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, MissingJsonContentType
    {
        json_syntax_error : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, BindQuery
    {
        checked_add :
        crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate::common::code_occurence::CodeOccurence
    }, NoPayloadFields
    {
        no_payload_fields : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
    {
        update_one_path_try_from_update_one_path_with_serialize_deserialize :
        UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : crate::common::code_occurence::CodeOccurence
    }, UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        uuid_wrapper_try_from_possible_uuid_wrapper_in_server :
        std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std::string::String<>,
        project_commit_to_use : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std::string::String, code_occurence
        : crate::common::code_occurence::CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryUpdateOne> for TryUpdateOneResponseVariants {
    fn from(value: TryUpdateOne) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryUpdateOneWithSerializeDeserialize :: Configuration
            { configuration_box_dyn_error, code_occurence } => Self ::
            Configuration { configuration_box_dyn_error, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Database
            { box_dyn_database_error, code_occurence } => Self :: Database
            { box_dyn_database_error, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Io
            { io_error, code_occurence } => Self :: Io
            { io_error, code_occurence }, TryUpdateOneWithSerializeDeserialize
            :: Tls { box_dyn_error, code_occurence } => Self :: Tls
            { box_dyn_error, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryUpdateOneWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Decode
            { decode_box_dyn_error, code_occurence } => Self :: Decode
            { decode_box_dyn_error, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence } => Self :: PoolTimedOut
            { pool_timed_out, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryUpdateOneWithSerializeDeserialize
            :: UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryUpdateOneWithSerializeDeserialize ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence } => Self ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: MissingPathParams
            { missing_path_params, code_occurence } => Self ::
            MissingPathParams { missing_path_params, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: MissingJsonContentType
            { json_syntax_error, code_occurence } => Self ::
            MissingJsonContentType { json_syntax_error, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: BindQuery
            { checked_add, code_occurence } => Self :: BindQuery
            { checked_add, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryUpdateOneWithSerializeDeserialize ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize,
                code_occurence
            } => Self ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize,
                code_occurence
            }, TryUpdateOneWithSerializeDeserialize ::
            UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
                code_occurence
            }, TryUpdateOneWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryUpdateOneWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryUpdateOneWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryUpdateOneResponseVariants> for http::StatusCode {
    fn from(value: &TryUpdateOneResponseVariants) -> Self {
        match value
        {
            TryUpdateOneResponseVariants :: Desirable(_) => http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: Configuration
            { configuration_box_dyn_error : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: Database
            { box_dyn_database_error : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: Io
            { io_error : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryUpdateOneResponseVariants :: Tls
            { box_dyn_error : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryUpdateOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            Decode { decode_box_dyn_error : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryUpdateOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateOneResponseVariants ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            MissingPathParams { missing_path_params : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            JsonDataError { json_data_error : _, code_occurence : _ } => http
            :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            JsonSyntaxError { json_syntax_error : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            MissingJsonContentType
            { json_syntax_error : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: BindQuery
            { checked_add : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateOneResponseVariants :: NoPayloadFields
            { no_payload_fields : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryUpdateOneResponseVariants ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize
                : _, code_occurence : _
            } => http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                uuid_wrapper_try_from_possible_uuid_wrapper_in_server : _,
                code_occurence : _
            } => http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } => http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } => http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateOneResponseVariantsTvfrr200Ok {
    Desirable(crate::server::postgres::uuid_wrapper::PossibleUuidWrapper),
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr200Ok> for TryUpdateOneResponseVariants {
    fn from(value: TryUpdateOneResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryUpdateOneResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateOneResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr408RequestTimeout>
    for TryUpdateOneResponseVariants
{
    fn from(value: TryUpdateOneResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryUpdateOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
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
enum TryUpdateOneResponseVariantsTvfrr400BadRequest {
    TypeNotFound
    {
        type_not_found : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, FailedToDeserializePathParams
    {
        failed_to_deserialize_path_params : std::string::String<>,
        code_occurence : crate::common::code_occurence::CodeOccurence
    }, MissingPathParams
    {
        missing_path_params : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, JsonDataError
    {
        json_data_error : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, MissingJsonContentType
    {
        json_syntax_error : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, NoPayloadFields
    {
        no_payload_fields : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
    {
        update_one_path_try_from_update_one_path_with_serialize_deserialize :
        UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : crate::common::code_occurence::CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std::string::String<>,
        project_commit_to_use : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std::string::String, code_occurence
        : crate::common::code_occurence::CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr400BadRequest>
    for TryUpdateOneResponseVariants
{
    fn from(value: TryUpdateOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryUpdateOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence } => Self ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            MissingPathParams { missing_path_params, code_occurence } => Self
            :: MissingPathParams { missing_path_params, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType { json_syntax_error, code_occurence } =>
            Self :: MissingJsonContentType
            { json_syntax_error, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize,
                code_occurence
            } => Self ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize,
                code_occurence
            }, TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateOneResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr404NotFound>
    for TryUpdateOneResponseVariants
{
    fn from(value: TryUpdateOneResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryUpdateOneResponseVariantsTvfrr404NotFound::RowNotFound {
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
enum TryUpdateOneResponseVariantsTvfrr500InternalServerError {
    Configuration
    {
        configuration_box_dyn_error : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Database
    {
        box_dyn_database_error : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Io
    {
        io_error : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Tls
    {
        box_dyn_error : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Protocol
    {
        protocol : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize<>, len : usize<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std::string::String<>, source_handle :
        std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Decode
    {
        decode_box_dyn_error : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, PoolClosed
    {
        pool_closed : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Migrate
    {
        migrate : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, BindQuery
    {
        checked_add :
        crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate::common::code_occurence::CodeOccurence
    }, UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        uuid_wrapper_try_from_possible_uuid_wrapper_in_server :
        std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr500InternalServerError>
    for TryUpdateOneResponseVariants
{
    fn from(value: TryUpdateOneResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration_box_dyn_error, code_occurence } =>
            Self :: Configuration
            { configuration_box_dyn_error, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            Database { box_dyn_database_error, code_occurence } => Self ::
            Database { box_dyn_database_error, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError :: Io
            { io_error, code_occurence } => Self :: Io
            { io_error, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError :: Tls
            { box_dyn_error, code_occurence } => Self :: Tls
            { box_dyn_error, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError :: Decode
            { decode_box_dyn_error, code_occurence } => Self :: Decode
            { decode_box_dyn_error, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            BindQuery { checked_add, code_occurence } => Self :: BindQuery
            { checked_add, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
                code_occurence
            }
        }
    }
}
async fn try_from_response_try_update_one(
    response: reqwest::Response,
) -> Result<
    TryUpdateOneResponseVariants,
    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
> {
    let status_code = response.status();
    let headers = response.headers().clone();
    if status_code == http::StatusCode::OK {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryUpdateOneResponseVariantsTvfrr200Ok > (& response_text)
            {
                Ok(value) => Ok(TryUpdateOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryUpdateOneResponseVariantsTvfrr400BadRequest > (& response_text)
            {
                Ok(value) => Ok(TryUpdateOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryUpdateOneResponseVariantsTvfrr404NotFound > (& response_text)
            {
                Ok(value) => Ok(TryUpdateOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryUpdateOneResponseVariantsTvfrr500InternalServerError >
            (& response_text)
            {
                Ok(value) => Ok(TryUpdateOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else {
        match response.text().await
        {
            Ok(response_text) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ResponseText(response_text)
            },), Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ReqwestError(e),
            },),
        }
    }
}
impl TryFrom<TryUpdateOneResponseVariants>
    for crate::server::postgres::uuid_wrapper::PossibleUuidWrapper
{
    type Error = TryUpdateOneWithSerializeDeserialize;
    fn try_from(value: TryUpdateOneResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryUpdateOneResponseVariants :: Desirable(i) => Ok(i),
            TryUpdateOneResponseVariants :: Configuration
            { configuration_box_dyn_error, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Configuration
            { configuration_box_dyn_error, code_occurence }),
            TryUpdateOneResponseVariants :: Database
            { box_dyn_database_error, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Database
            { box_dyn_database_error, code_occurence }),
            TryUpdateOneResponseVariants :: Io { io_error, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Io
            { io_error, code_occurence }), TryUpdateOneResponseVariants :: Tls
            { box_dyn_error, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Tls
            { box_dyn_error, code_occurence }), TryUpdateOneResponseVariants
            :: Protocol { protocol, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryUpdateOneResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryUpdateOneResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryUpdateOneResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryUpdateOneResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryUpdateOneResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryUpdateOneResponseVariants :: Decode
            { decode_box_dyn_error, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Decode
            { decode_box_dyn_error, code_occurence }),
            TryUpdateOneResponseVariants :: PoolTimedOut
            { pool_timed_out, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryUpdateOneResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryUpdateOneResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryUpdateOneResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryUpdateOneResponseVariants ::
            UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }), TryUpdateOneResponseVariants
            :: FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence }),
            TryUpdateOneResponseVariants :: MissingPathParams
            { missing_path_params, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: MissingPathParams
            { missing_path_params, code_occurence }),
            TryUpdateOneResponseVariants :: JsonDataError
            { json_data_error, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }), TryUpdateOneResponseVariants
            :: JsonSyntaxError { json_syntax_error, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryUpdateOneResponseVariants :: MissingJsonContentType
            { json_syntax_error, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: MissingJsonContentType
            { json_syntax_error, code_occurence }),
            TryUpdateOneResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }), TryUpdateOneResponseVariants
            :: BindQuery { checked_add, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: BindQuery
            { checked_add, code_occurence }), TryUpdateOneResponseVariants ::
            NoPayloadFields { no_payload_fields, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence }),
            TryUpdateOneResponseVariants ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize,
                code_occurence
            }), TryUpdateOneResponseVariants ::
            UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
                code_occurence
            } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                uuid_wrapper_try_from_possible_uuid_wrapper_in_server,
                code_occurence
            }), TryUpdateOneResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), TryUpdateOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            TryUpdateOneResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryUpdateOneRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryUpdateOneWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: crate::common::code_occurence::CodeOccurence,
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
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
async fn tvfrr_extraction_logic_try_update_one<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper, TryUpdateOneRequestError> {
    match future.await
    {
        Ok(response) => match try_from_response_try_update_one(response).await
        {
            Ok(variants) => match crate :: server :: postgres :: uuid_wrapper
            :: PossibleUuidWrapper :: try_from(variants)
            {
                Ok(value) => Ok(value), Err(e) =>
                Err(TryUpdateOneRequestError :: ExpectedType
                {
                    expected_type : e, code_occurence : crate ::
                    code_occurence_tufa_common! (),
                }),
            }, Err(e) => match e
            {
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: StatusCode
                { status_code, headers, response_text_result, } =>
                Err(TryUpdateOneRequestError :: UnexpectedStatusCode
                {
                    status_code, headers, response_text_result, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: FailedToGetResponseText
                { reqwest, status_code, headers } =>
                Err(TryUpdateOneRequestError :: FailedToGetResponseText
                {
                    reqwest, status_code, headers, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryUpdateOneRequestError :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }),
            },
        }, Err(e) =>
        Err(TryUpdateOneRequestError :: Reqwest
        {
            reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
            (),
        }),
    }
}
pub enum TryUpdateOneStatusCodesChecker {
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
    FailedToDeserializePathParamsTvfrr400BadRequest,
    MissingPathParamsTvfrr400BadRequest,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    BindQueryTvfrr500InternalServerError,
    NoPayloadFieldsTvfrr400BadRequest,
    UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeTvfrr400BadRequest,
    UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryUpdateOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryUpdateOneResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            } TryUpdateOneResponseVariants :: Configuration
            { configuration_box_dyn_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Database
            { box_dyn_database_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Io
            { io_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Tls
            { box_dyn_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Decode
            { decode_box_dyn_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: FailedToDeserializePathParams
            { failed_to_deserialize_path_params : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: MissingPathParams
            { missing_path_params : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: MissingJsonContentType
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: BindQuery
            { checked_add : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: NoPayloadFields
            { no_payload_fields : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants ::
            UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                uuid_wrapper_try_from_possible_uuid_wrapper_in_server : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }
        }
    }
}
pub async fn try_update_one<'a>(
    server_location: &str,
    parameters: UpdateOneParameters,
) -> Result<(), TryUpdateOneErrorNamed> {
    let payload = match serde_json::to_string(&parameters.payload) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryUpdateOneErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }
    };
    let url = format!("{}/dogs/{}", server_location, parameters.path.id.to_inner());
    match tvfrr_extraction_logic_try_update_one(
        reqwest::Client::new()
            .patch(&url)
            .header(
                crate::common::git::project_git_info::PROJECT_COMMIT,
                crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                    .project_commit,
            )
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send(),
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(TryUpdateOneErrorNamed::RequestError {
            request_error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
pub async fn update_one<'a>(
    path_extraction_result: Result<
        axum::extract::Path<UpdateOnePathWithSerializeDeserialize>,
        axum::extract::rejection::PathRejection,
    >,
    app_info_state : axum :: extract :: State < crate ::
repositories_types :: tufa_server :: routes :: api :: cats ::
DynArcGetConfigGetPostgresPoolSendSync >,
    payload_extraction_result: Result<
        axum::Json<UpdateOnePayload>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = UpdateOneParameters {
        path: match crate::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
            UpdateOnePathWithSerializeDeserialize,
            TryUpdateOneResponseVariants,
        >::try_extract_value(path_extraction_result, &app_info_state)
        {
            Ok(value) => match UpdateOnePath::try_from(value) {
                Ok(value) => value,
                Err(e) => {
                    let error =
                        TryUpdateOne::UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize {
                            update_one_path_try_from_update_one_path_with_serialize_deserialize: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryUpdateOneResponseVariants::from(error);
                }
            },
            Err(err) => {
                return err;
            }
        },
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                UpdateOnePayload,
                TryUpdateOneResponseVariants,
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
        if let (None, None) = (&parameters.payload.name, &parameters.payload.color) {
            return TryUpdateOneResponseVariants::NoPayloadFields {
                no_payload_fields: std::string::String::from("no payload fields"),
                code_occurence: crate::code_occurence_tufa_common!(),
            };
        }
        let query_string = {
            let mut increment: u64 = 0;
            let mut query = std::string::String::from("update dogs set ");
            if let Some(value) = &parameters.payload.name {
                match crate::server::postgres::bind_query::BindQuery::try_increment(
                    value,
                    &mut increment,
                ) {
                    Ok(_) => {
                        query.push_str(&format!("name = ${increment}, "));
                    }
                    Err(e) => {
                        return TryUpdateOneResponseVariants::BindQuery {
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
                        query.push_str(&format!("color = ${increment}"));
                    }
                    Err(e) => {
                        return TryUpdateOneResponseVariants::BindQuery {
                            checked_add: e.into_serialize_deserialize_version(),
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                    }
                }
            }
            match crate::server::postgres::bind_query::BindQuery::try_increment(
                &parameters.path.id,
                &mut increment,
            ) {
                Ok(_) => {
                    query.push_str(&format!(" where id = ${increment}"));
                }
                Err(e) => {
                    return TryUpdateOneResponseVariants::BindQuery {
                        checked_add: e.into_serialize_deserialize_version(),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    };
                }
            }
            query.push_str(&format!(" returning id"));
            query
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            if let Some(value) = parameters.payload.name {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            if let Some(value) = parameters.payload.color {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                parameters.path.id,
                query,
            );
            query
        };
        let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let error = TryUpdateOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateOneResponseVariants::from(error);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let error = TryUpdateOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateOneResponseVariants::from(error);
            }
        };
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            // Ok(_) => TryUpdateOneResponseVariants::Desirable(()),//HERE
            Ok(value) => match {
                use sqlx::Row;
                value.try_get::<sqlx::types::Uuid, &str>("id")
            } {
                Ok(value) => TryUpdateOneResponseVariants::Desirable(
                    crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value),
                ),
                Err(e) => {
                    let error = TryUpdateOne ::
                    UpdatedButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
                    {
                        uuid_wrapper_try_from_possible_uuid_wrapper_in_server : e,
                        code_occurence : crate :: code_occurence_tufa_common! (),
                    } ;
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryUpdateOneResponseVariants::from(error);
                }
            },
            Err(e) => {
                let error = TryUpdateOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateOneResponseVariants::from(error);
            }
        }
    }
}
impl
    std::convert::From<
        crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed,
    > for TryUpdateOne
{
    fn from(
        value :
    crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed
            :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            },
            crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed
            :: NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
