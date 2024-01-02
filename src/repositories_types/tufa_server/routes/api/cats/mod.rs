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

///////////////////

#[derive(Debug)]
pub struct ReadOneParameters {
    pub payload: ReadOnePayload,
}

#[derive(Debug, utoipa::ToSchema)]
pub struct ReadOnePayload {
    pub id: crate::server::postgres::uuid_wrapper::UuidWrapper,
    pub select: DogColumnSelect,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadOnePayloadWithSerializeDeserialize {
    id: crate::server::postgres::uuid_wrapper::PossibleUuidWrapper,
    select: DogColumnSelect,//std::string::String,
}

#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamed {
    NotUuid {
        #[eo_error_occurence]
        not_uuid: crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<ReadOnePayloadWithSerializeDeserialize> for ReadOnePayload {
    type Error = ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: ReadOnePayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let id = match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value.id) {
            Ok(value) => value,
            Err(e) => {
                return Err(Self::Error::NotUuid {
                    not_uuid: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        let select = value.select;
        Ok(Self { 
            id,
            select
        })
    }
}

impl std::convert::From<ReadOnePayload> for ReadOnePayloadWithSerializeDeserialize {
    fn from(value: ReadOnePayload) -> Self {
        let id = crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value.id);
        let select = value.select;
        Self {
            id,
            select,
        }
    }
}
// #[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct ReadOneQuery {
//     pub select: std::option::Option<DogColumnSelect>,
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// struct ReadOneQueryWithSerializeDeserialize {
//     select: std::option::Option<std::string::String>,
// }
// impl ReadOneQuery {
//     fn into_url_encoding_version(self) -> ReadOneQueryWithSerializeDeserialize {
//         let select = self.select.map(|value| {
//             crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
//                 value,
//             )
//         });
//         ReadOneQueryWithSerializeDeserialize { select }
//     }
// }
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryReadOneErrorNamed {
    RequestError {
        #[eo_error_occurence]
        request_error: TryReadOneRequestError,
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
pub enum TryReadOne {
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
    JsonDataError {
        #[eo_display_with_serialize_deserialize]
        json_data_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
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
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        read_one_payload_try_from_read_one_payload_with_serialize_deserialize: ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamed,
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
pub enum TryReadOneResponseVariants {
    Desirable(DogOptions),
    Configuration {
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonDataError
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
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
        read_one_payload_try_from_read_one_payload_with_serialize_deserialize: ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
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
impl std::convert::From<TryReadOne> for TryReadOneResponseVariants {
    fn from(value: TryReadOne) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryReadOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryReadOneWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence }, TryReadOneWithSerializeDeserialize
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryReadOneWithSerializeDeserialize :: Tls { tls, code_occurence }
            => Self :: Tls { tls, code_occurence },
            TryReadOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryReadOneWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryReadOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryReadOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryReadOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryReadOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryReadOneWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryReadOneWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryReadOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryReadOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryReadOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, 
            
            TryReadOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryReadOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryReadOneWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryReadOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            
            TryReadOneWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryReadOneWithSerializeDeserialize ::ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            } => Self :: ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            }, 
            TryReadOneWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryReadOneWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryReadOneWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryReadOneResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryReadOneResponseVariants) -> Self {
        match value {
            TryReadOneResponseVariants::Desirable(_) => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Configuration {
                configuration: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Database {
                database: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Io {
                io: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Tls {
                tls: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Protocol {
                protocol: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Decode {
                decode: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Migrate {
                migrate: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,

            //
            TryReadOneResponseVariants
            :: JsonDataError { json_data_error : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK,
            TryReadOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadOneResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryReadOneResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, 
            //
            TryReadOneResponseVariants::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::ProjectCommitExtractorNotEqual {
                project_commit_not_equal: _,
                project_commit_to_use: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::ProjectCommitExtractorToStrConversion {
                project_commit_to_str_conversion: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::NoProjectCommitExtractorHeader {
                no_project_commit_header: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadOneResponseVariantsTvfrr200Ok {
    Desirable(DogOptions),
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr200Ok> for TryReadOneResponseVariants {
    fn from(value: TryReadOneResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadOneResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr404NotFound> for TryReadOneResponseVariants {
    fn from(value: TryReadOneResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadOneResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr408RequestTimeout>
    for TryReadOneResponseVariants
{
    fn from(value: TryReadOneResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadOneResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    BytesRejection
    {
        bytes_rejection : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, 
    //
    Migrate {
        migrate: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr500InternalServerError>
    for TryReadOneResponseVariants
{
    fn from(value: TryReadOneResponseVariantsTvfrr500InternalServerError) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr500InternalServerError::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            TryReadOneResponseVariantsTvfrr500InternalServerError::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            //
TryReadOneResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            //
            TryReadOneResponseVariantsTvfrr500InternalServerError::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadOneResponseVariantsTvfrr400BadRequest {
    //
    JsonDataError
    {
        json_data_error : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, 
    JsonSyntaxError
    {
        json_syntax_error : std::string::String, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, 
    MissingJsonContentType
    {
        missing_json_content_type : std::string::String<>, code_occurence :
        crate::common::code_occurence::CodeOccurence
    }, 
    //
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
        read_one_payload_try_from_read_one_payload_with_serialize_deserialize: ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
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
impl std::convert::From<TryReadOneResponseVariantsTvfrr400BadRequest>
    for TryReadOneResponseVariants
{
    fn from(value: TryReadOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            //
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            JsonSyntaxError { json_syntax_error, code_occurence } => Self ::
            JsonSyntaxError { json_syntax_error, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            //
            TryReadOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            } => Self :: ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            }, 
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryReadOneResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
async fn try_from_response_try_read_one(
    response: reqwest::Response,
) -> Result<
    TryReadOneResponseVariants,
    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
> {
    let status_code = response.status();
    let headers = response.headers().clone();
    if status_code == http::StatusCode::OK {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryReadOneResponseVariantsTvfrr200Ok > (& response_text)
            {
                Ok(value) => Ok(TryReadOneResponseVariants :: from(value)),
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
            TryReadOneResponseVariantsTvfrr500InternalServerError >
            (& response_text)
            {
                Ok(value) => Ok(TryReadOneResponseVariants :: from(value)),
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
            TryReadOneResponseVariantsTvfrr404NotFound > (& response_text)
            {
                Ok(value) => Ok(TryReadOneResponseVariants :: from(value)),
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
            TryReadOneResponseVariantsTvfrr400BadRequest > (& response_text)
            {
                Ok(value) => Ok(TryReadOneResponseVariants :: from(value)),
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
impl TryFrom<TryReadOneResponseVariants> for DogOptions {
    type Error = TryReadOneWithSerializeDeserialize;
    fn try_from(value: TryReadOneResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryReadOneResponseVariants :: Desirable(i) => Ok(i),
            TryReadOneResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryReadOneResponseVariants ::
            Database { database, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Database
            { database, code_occurence }), TryReadOneResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Io
            { io, code_occurence }), TryReadOneResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryReadOneResponseVariants :: Protocol
            { protocol, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryReadOneResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryReadOneResponseVariants ::
            TypeNotFound { type_not_found, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryReadOneResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryReadOneResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }), TryReadOneResponseVariants
            :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryReadOneResponseVariants :: Decode { decode, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryReadOneResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryReadOneResponseVariants ::
            PoolClosed { pool_closed, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryReadOneResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryReadOneResponseVariants ::
            Migrate { migrate, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), 
            
            //
            TryReadOneResponseVariants :: JsonDataError
            { json_data_error, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }),
            TryReadOneResponseVariants :: JsonSyntaxError
            { json_syntax_error, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryReadOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryReadOneResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }),
            //
            
            TryReadOneResponseVariants ::
            UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }),
            TryReadOneResponseVariants ::
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryReadOneWithSerializeDeserialize ::
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            }), 
            TryReadOneResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryReadOneWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), TryReadOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            TryReadOneResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryReadOneRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryReadOneWithSerializeDeserialize,
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
async fn tvfrr_extraction_logic_try_read_one<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<DogOptions, TryReadOneRequestError> {
    match future.await
    {
        Ok(response) => match try_from_response_try_read_one(response).await
        {
            Ok(variants) => match DogOptions :: try_from(variants)
            {
                Ok(value) => Ok(value), Err(e) =>
                Err(TryReadOneRequestError :: ExpectedType
                {
                    expected_type : e, code_occurence : crate ::
                    code_occurence_tufa_common! (),
                }),
            }, Err(e) => match e
            {
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: StatusCode
                { status_code, headers, response_text_result, } =>
                Err(TryReadOneRequestError :: UnexpectedStatusCode
                {
                    status_code, headers, response_text_result, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: FailedToGetResponseText
                { reqwest, status_code, headers } =>
                Err(TryReadOneRequestError :: FailedToGetResponseText
                {
                    reqwest, status_code, headers, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryReadOneRequestError :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }),
            },
        }, Err(e) =>
        Err(TryReadOneRequestError :: Reqwest
        {
            reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
            (),
        }),
    }
}
pub enum TryReadOneStatusCodesChecker {
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
    //
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    //
    UnexpectedCaseTvfrr500InternalServerError,
    ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeTvfrr400BadRequest,
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryReadOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match &self {
            TryReadOneResponseVariants::Desirable(_) => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Configuration {
                configuration: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Database {
                database: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Io {
                io: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Tls {
                tls: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Protocol {
                protocol: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Decode {
                decode: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Migrate {
                migrate: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            //
            TryReadOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, 
            //
            TryReadOneResponseVariants::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ProjectCommitExtractorNotEqual {
                project_commit_not_equal: _,
                project_commit_to_use: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ProjectCommitExtractorToStrConversion {
                project_commit_to_str_conversion: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::NoProjectCommitExtractorHeader {
                no_project_commit_header: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
        }
    }
}
pub async fn try_read_one<'a>(
    server_location: &str,
    parameters: ReadOneParameters,
) -> Result<DogOptions, TryReadOneErrorNamed> {
    let payload = match serde_json::to_string(&ReadOnePayloadWithSerializeDeserialize::from(parameters.payload)) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryReadOneErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }
    };
    let url = format!(
        "{}/dogs/",
        server_location
    );
    match tvfrr_extraction_logic_try_read_one(
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
        Ok(value) => Ok(value),
        Err(e) => Err(TryReadOneErrorNamed::RequestError {
            request_error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}

#[utoipa::path(
    post, 
    path = "/dogs/read_one", 
    operation_id = "/dogs/read_one", 
    tag = "dogs", 
    request_body(content = ReadOnePayload, description = "todo", content_type = "application/json"),
    responses(
        (status = 200, description = "ok", body = TryReadOneResponseVariantsTvfrr200Ok, content_type = "application/json"),
        (status = 500, description = "internal server error", body = TryReadOneResponseVariantsTvfrr500InternalServerError, content_type = "application/json"),
        (status = 404, description = "not found", body = TryReadOneResponseVariantsTvfrr404NotFound, content_type = "application/json"),
        (status = 400, description = "bad request", body = TryReadOneResponseVariantsTvfrr400BadRequest, content_type = "application/json"),
        (status = 408, description = "request timeout", body = TryReadOneResponseVariantsTvfrr408RequestTimeout, content_type = "application/json")
    ),
)]
pub async fn read_one(
    app_info_state : axum :: extract :: State < crate :: repositories_types ::tufa_server :: routes :: api :: cats :: DynArcGetConfigGetPostgresPoolSendSync>,
    payload_extraction_result: Result<
        axum::Json<ReadOnePayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = ReadOneParameters {
        payload: match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
            ReadOnePayloadWithSerializeDeserialize,
            TryReadOneResponseVariants,
        >::try_extract_value(payload_extraction_result, &app_info_state) {
            Ok(value) => match ReadOnePayload::try_from(value) {
                Ok(value) => value, 
                Err(e) => {
                    let error = TryReadOne::ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
                        read_one_payload_try_from_read_one_payload_with_serialize_deserialize: e, 
                        code_occurence: crate::code_occurence_tufa_common!(),
                    }; 
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error, 
                        app_info_state.as_ref(),
                    );
                    return TryReadOneResponseVariants::from(error);
                }
            },
            Err(err) => { 
                return err; 
            }
        },
    };
    println!("{:#?}", parameters);
    {
        let select = parameters.payload.select;
        let query_string = {
            format!(
                "select {} from dogs where id = $1",
                crate::server::postgres::generate_query::GenerateQuery::generate_query(&select),
            )
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            let query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                parameters.payload.id,
                query,
            );
            query
        };
        let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let error = TryReadOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryReadOneResponseVariants::from(error);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let error = TryReadOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryReadOneResponseVariants::from(error);
            }
        };
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(row) => match select.options_try_from_sqlx_row(&row) {
                Ok(value) => TryReadOneResponseVariants::Desirable(value),
                Err(e) => {
                    let error = TryReadOne::from(e);
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryReadOneResponseVariants::from(error);
                }
            },
            Err(e) => {
                let error = TryReadOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryReadOneResponseVariants::from(error);
            }
        }
    }
}
impl
    std::convert::From<
        crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed,
    > for TryReadOne
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
