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

#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct CreateManyParameters {
    pub payload: std::vec::Vec<CreateManyPayloadElement>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa::ToSchema)]
pub struct CreateManyPayloadElement {
    pub name: std::string::String,
    pub color: std::string::String,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryCreateManyErrorNamed {
    RequestError {
        #[eo_error_occurence]
        request_error: TryCreateManyRequestError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient {
        #[eo_vec_error_occurence]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client:
            std::vec::Vec<
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed,
            >,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed {
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient(
        crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
    ),
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryCreateMany {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
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
        decode: std::string::String,
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
        missing_json_content_type: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        #[eo_display]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            sqlx::Error,
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
pub enum TryCreateManyResponseVariants {
    Desirable(std :: vec :: Vec :: < crate :: server :: postgres ::
    uuid_wrapper :: PossibleUuidWrapper >), Configuration
    {
        configuration : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Database
    {
        database : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Io
    {
        io : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Tls
    {
        tls : std::string::String<>, code_occurence :
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
        decode : std::string::String<>, code_occurence :
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
        missing_json_content_type : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, BindQuery
    {
        bind_query :
        crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate::common::code_occurence::CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std::string::String, code_occurence :
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
impl std::convert::From<TryCreateMany> for TryCreateManyResponseVariants {
    fn from(value: TryCreateMany) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryCreateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Io { io, code_occurence }
            => Self :: Io { io, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryCreateManyWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryCreateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryCreateManyWithSerializeDeserialize
            :: UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryCreateManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryCreateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateManyWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryCreateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryCreateManyWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }, TryCreateManyWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryCreateManyWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryCreateManyWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryCreateManyResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryCreateManyResponseVariants) -> Self {
        match value
        {
            TryCreateManyResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Configuration { configuration : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Database { database : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            TypeNotFound { type_not_found : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } => axum :: http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            WorkerCrashed { worker_crashed : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Migrate { migrate : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            JsonDataError { json_data_error : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            JsonSyntaxError { json_syntax_error : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            BindQuery { bind_query : _, code_occurence : _ } => axum :: http
            :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } => axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa::ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr201Created {
    Desirable(std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>),
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr201Created>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr201Created) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr201Created::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa::ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr408RequestTimeout>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa::ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr404NotFound>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa::ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr400BadRequest {
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        project_commit_not_equal: std::string::String,
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        project_commit_to_str_conversion: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr400BadRequest>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryCreateManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryCreateManyResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa::ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr500InternalServerError {
    Configuration
    {
        configuration : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Database
    {
        database : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Io
    {
        io : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, Tls
    {
        tls : std::string::String<>, code_occurence :
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
        decode : std::string::String<>, code_occurence :
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
        bind_query :
        crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate::common::code_occurence::CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr500InternalServerError>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }
        }
    }
}
async fn try_from_response_try_create_many(
    response: reqwest::Response,
) -> Result<
    TryCreateManyResponseVariants,
    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
> {
    let status_code = response.status();
    let headers = response.headers().clone();
    if status_code == http::StatusCode::CREATED {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryCreateManyResponseVariantsTvfrr201Created > (& response_text)
            {
                Ok(value) => Ok(TryCreateManyResponseVariants :: from(value)),
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
            TryCreateManyResponseVariantsTvfrr404NotFound > (& response_text)
            {
                Ok(value) => Ok(TryCreateManyResponseVariants :: from(value)),
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
            TryCreateManyResponseVariantsTvfrr500InternalServerError >
            (& response_text)
            {
                Ok(value) => Ok(TryCreateManyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryCreateManyResponseVariantsTvfrr408RequestTimeout >
            (& response_text)
            {
                Ok(value) => Ok(TryCreateManyResponseVariants :: from(value)),
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
impl TryFrom<TryCreateManyResponseVariants>
    for std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>
{
    type Error = TryCreateManyWithSerializeDeserialize;
    fn try_from(value: TryCreateManyResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryCreateManyResponseVariants :: Desirable(i) => Ok(i),
            TryCreateManyResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryCreateManyResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Database
            { database, code_occurence }), TryCreateManyResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Io
            { io, code_occurence }), TryCreateManyResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryCreateManyResponseVariants ::
            Protocol { protocol, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryCreateManyResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryCreateManyResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryCreateManyResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryCreateManyResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryCreateManyResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryCreateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryCreateManyResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryCreateManyResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryCreateManyResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryCreateManyResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryCreateManyResponseVariants ::
            UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }),
            TryCreateManyResponseVariants :: JsonDataError
            { json_data_error, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }),
            TryCreateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryCreateManyResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }),
            TryCreateManyResponseVariants :: BindQuery
            { bind_query, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }), TryCreateManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }), TryCreateManyResponseVariants ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), TryCreateManyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            TryCreateManyResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryCreateManyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryCreateManyWithSerializeDeserialize,
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
async fn tvfrr_extraction_logic_try_create_many<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<
    std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>,
    TryCreateManyRequestError,
> {
    match future.await
    {
        Ok(response) => match
        try_from_response_try_create_many(response).await
        {
            Ok(variants) => match std :: vec :: Vec :: < crate :: server ::
            postgres :: uuid_wrapper :: PossibleUuidWrapper > ::
            try_from(variants)
            {
                Ok(value) => Ok(value), Err(e) =>
                Err(TryCreateManyRequestError :: ExpectedType
                {
                    expected_type : e, code_occurence : crate ::
                    code_occurence_tufa_common! (),
                }),
            }, Err(e) => match e
            {
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: StatusCode
                { status_code, headers, response_text_result, } =>
                Err(TryCreateManyRequestError :: UnexpectedStatusCode
                {
                    status_code, headers, response_text_result, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: FailedToGetResponseText
                { reqwest, status_code, headers } =>
                Err(TryCreateManyRequestError :: FailedToGetResponseText
                {
                    reqwest, status_code, headers, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryCreateManyRequestError :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }),
            },
        }, Err(e) =>
        Err(TryCreateManyRequestError :: Reqwest
        {
            reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
            (),
        }),
    }
}
pub enum TryCreateManyStatusCodesChecker {
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
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    BindQueryTvfrr500InternalServerError,
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryCreateManyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryCreateManyResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            } TryCreateManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }
        }
    }
}
pub async fn try_create_many<'a>(
    server_location: &str,
    parameters: CreateManyParameters,
) -> Result<
    std::vec::Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
    TryCreateManyErrorNamed,
> {
    let payload = match serde_json::to_string(&parameters.payload) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryCreateManyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }
    };
    let url = format!("{}/dogs/batch", server_location,);
    match tvfrr_extraction_logic_try_create_many(
        reqwest::Client::new()
            .post(&url)
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
        Ok(value) => {
            let mut vec_values = std::vec::Vec::with_capacity(value.len());
            let mut vec_errors = std::vec::Vec::with_capacity(value.len());
            for element in value {
                match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(element) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        vec_errors.push(OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed
                        ::
                        OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient(e))
                        ;
                    }
                }
            }
            if let false = vec_errors.is_empty() {
                return
                Err(TryCreateManyErrorNamed ::
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient
                {
                    operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client
                    : vec_errors, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }) ;
            }
            Ok(vec_values)
        }
        Err(e) => Err(TryCreateManyErrorNamed::RequestError {
            request_error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
// TryCreateManyResponseVariantsTvfrr201Created
// TryCreateManyResponseVariantsTvfrr408RequestTimeout
// TryCreateManyResponseVariantsTvfrr404NotFound
// TryCreateManyResponseVariantsTvfrr400BadRequest
// TryCreateManyResponseVariantsTvfrr500InternalServerError
#[utoipa::path(
    post,
    path = "api/dogs/create_many",
    operation_id = "api/dogs/create_many",
    tag = "dogs",
    responses(
        (status = 201, description = "Created", body = [TryCreateManyResponseVariantsTvfrr201Created], content_type = "application/json"),
        (status = 408, description = "RequestTimeout", body = [TryCreateManyResponseVariantsTvfrr408RequestTimeout], content_type = "application/json"),
        (status = 404, description = "NotFound", body = [TryCreateManyResponseVariantsTvfrr404NotFound], content_type = "application/json"),
        (status = 400, description = "BadRequest", body = [TryCreateManyResponseVariantsTvfrr400BadRequest], content_type = "application/json"),
        (status = 500, description = "InternalServerError", body = [TryCreateManyResponseVariantsTvfrr500InternalServerError], content_type = "application/json")
    ),
    request_body(content = [CreateManyPayloadElement], description = "Pet to store the database", content_type = "application/json"),
)]
pub async fn create_many(
    app_info_state : axum :: extract :: State < crate ::
repositories_types :: tufa_server :: routes :: api :: cats ::
DynArcGetConfigGetPostgresPoolSendSync >,
    payload_extraction_result: Result<
        axum::Json<std::vec::Vec<CreateManyPayloadElement>>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = CreateManyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                std::vec::Vec<CreateManyPayloadElement>,
                TryCreateManyResponseVariants,
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
        let query_string = {
            "insert into dogs (name, color) select name, color from unnest($1, $2) as a(name, color) returning id"
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            let current_vec_len = parameters.payload.len();
            let (name_vec, color_vec) = parameters.payload.into_iter().fold(
                (
                    std::vec::Vec::with_capacity(current_vec_len),
                    std::vec::Vec::with_capacity(current_vec_len),
                ),
                |mut acc, element| {
                    acc.0.push(element.name);
                    acc.1.push(element.color);
                    acc
                },
            );
            query = query.bind(name_vec);
            query = query.bind(color_vec);
            query
        };
        let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let error = TryCreateMany::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryCreateManyResponseVariants::from(error);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let error = TryCreateMany::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryCreateManyResponseVariants::from(error);
            }
        };
        let mut rows = binded_query.fetch(pg_connection.as_mut());
        let mut vec_values = std::vec::Vec::new();
        while let Some(row) = {
            match {
                use futures::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => value,
                Err(e) => {
                    let error = TryCreateMany::from(e);
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryCreateManyResponseVariants::from(error);
                }
            }
        } {
            match {
                use sqlx::Row;
                row.try_get::<sqlx::types::Uuid, &str>("id")
            } {
                Ok(value) => {
                    vec_values.push(
                        crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value),
                    );
                }
                Err(e) => {
                    let error = TryCreateMany ::
                    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
                    {
                        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                        : e, code_occurence : crate :: code_occurence_tufa_common!
                        (),
                    } ;
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryCreateManyResponseVariants::from(error);
                }
            }
        }
        TryCreateManyResponseVariants::Desirable(vec_values)
    }
}
impl
    std::convert::From<
        crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed,
    > for TryCreateMany
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
