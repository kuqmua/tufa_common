type PostDesirableType = ();

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatToPost {
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
    PostDesirableType,
    tvfrr_201_created
)]
pub enum TryPost<'a> {
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

pub async fn try_post<'a>(
    server_location: &str,
    cat: crate::repositories_types::tufa_server::routes::api::cats::post::CatToPost,
) -> Result<PostDesirableType, TryPostErrorNamed<'a>> {
    let stringified_json = serde_json::to_string(&cat).unwrap();
    // match serde_json::to_string(&cat) {
    //     Ok(stringified_json) => stringified_json,
    //     Err(e) => {
    //         return Err(TryPostErrorNamed::SerdeJsonToString {
    //             serde_json_to_string: e,
    //             code_occurence: crate::code_occurence_tufa_common!(),
    //         });
    //     }
    // }
    extraction_logic(
        reqwest::Client::new()
            .post(&format!(
                "{server_location}/api/{}/",
                crate::repositories_types::tufa_server::routes::api::cats::CATS
            ))
            .header(
                crate::common::git::project_git_info::PROJECT_COMMIT,
                crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                    .project_commit,
            )
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(stringified_json)
            .send(),
    )
    .await
}

//////////////
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// pub enum TryPostResponseVariants {
//     DesirableType(PostDesirableType),
//     ProjectCommitExtractorNotEqual {
//         project_commit_not_equal: std::string::String,
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     ProjectCommitExtractorToStrConversion {
//         project_commit_to_str_conversion: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     NoProjectCommitExtractorHeader {
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Configuration {
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Database {
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Io {
//         io_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Tls {
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Protocol {
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     TypeNotFound {
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     ColumnIndexOutOfBounds {
//         column_index_out_of_bounds: usize,
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     ColumnNotFound {
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     ColumnDecode {
//         column_decode_index: std::string::String,
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Decode {
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     PoolClosed {
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     WorkerCrashed {
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Migrate {
//         migrate: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     UnexpectedCase {
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
// }
// impl<'a> std::convert::From<TryPost<'a>> for TryPostResponseVariants {
//     fn from(val: TryPost<'a>) -> Self {
//         match val.into_serialize_deserialize_version() {
//             TryPostWithSerializeDeserialize::ProjectCommitExtractorNotEqual {
//                 project_commit_not_equal,
//                 project_commit_to_use,
//                 code_occurence,
//             } => Self::ProjectCommitExtractorNotEqual {
//                 project_commit_not_equal,
//                 project_commit_to_use,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::ProjectCommitExtractorToStrConversion {
//                 project_commit_to_str_conversion,
//                 code_occurence,
//             } => Self::ProjectCommitExtractorToStrConversion {
//                 project_commit_to_str_conversion,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::NoProjectCommitExtractorHeader {
//                 no_project_commit_header,
//                 code_occurence,
//             } => Self::NoProjectCommitExtractorHeader {
//                 no_project_commit_header,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             } => Self::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             } => Self::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::Io {
//                 io_error,
//                 code_occurence,
//             } => Self::Io {
//                 io_error,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             } => Self::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::Protocol {
//                 protocol,
//                 code_occurence,
//             } => Self::Protocol {
//                 protocol,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             } => Self::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             } => Self::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             } => Self::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             } => Self::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             } => Self::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             } => Self::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             } => Self::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::Migrate {
//                 migrate,
//                 code_occurence,
//             } => Self::Migrate {
//                 migrate,
//                 code_occurence,
//             },
//             TryPostWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// impl From<TryPostResponseVariants> for actix_web::HttpResponse {
//     fn from(val: TryPostResponseVariants) -> Self {
//         let mut actix_web_http_response = actix_web::HttpResponseBuilder::new((&val).into());
//         actix_web_http_response.json(actix_web::web::Json(val))
//     }
// }
// impl std::convert::From<&TryPostResponseVariants> for http::StatusCode {
//     fn from(value: &TryPostResponseVariants) -> Self {
//         match value {
//             TryPostResponseVariants::DesirableType(_) => http::StatusCode::CREATED,
//             TryPostResponseVariants::ProjectCommitExtractorNotEqual {
//                 project_commit_not_equal: _,
//                 project_commit_to_use: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryPostResponseVariants::ProjectCommitExtractorToStrConversion {
//                 project_commit_to_str_conversion: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryPostResponseVariants::NoProjectCommitExtractorHeader {
//                 no_project_commit_header: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryPostResponseVariants::Configuration {
//                 configuration_box_dyn_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryPostResponseVariants::Database {
//                 box_dyn_database_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryPostResponseVariants::Io {
//                 io_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryPostResponseVariants::Tls {
//                 box_dyn_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryPostResponseVariants::Protocol {
//                 protocol: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryPostResponseVariants::RowNotFound {
//                 row_not_found: _,
//                 code_occurence: _,
//             } => http::StatusCode::NOT_FOUND,
//             TryPostResponseVariants::TypeNotFound {
//                 type_not_found: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryPostResponseVariants::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds: _,
//                 len: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryPostResponseVariants::ColumnNotFound {
//                 column_not_found: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             TryPostResponseVariants::ColumnDecode {
//                 column_decode_index: _,
//                 source_handle: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryPostResponseVariants::Decode {
//                 decode_box_dyn_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryPostResponseVariants::PoolTimedOut {
//                 pool_timed_out: _,
//                 code_occurence: _,
//             } => http::StatusCode::REQUEST_TIMEOUT,
//             TryPostResponseVariants::PoolClosed {
//                 pool_closed: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryPostResponseVariants::WorkerCrashed {
//                 worker_crashed: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryPostResponseVariants::Migrate {
//                 migrate: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             TryPostResponseVariants::UnexpectedCase {
//                 unexpected_case: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryPostResponseVariantsTvfrr400BadRequest {
//     ProjectCommitExtractorNotEqual {
//         project_commit_not_equal: std::string::String,
//         project_commit_to_use: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     ProjectCommitExtractorToStrConversion {
//         project_commit_to_str_conversion: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     NoProjectCommitExtractorHeader {
//         no_project_commit_header: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     TypeNotFound {
//         type_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     ColumnNotFound {
//         column_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
// }
// impl std::convert::From<TryPostResponseVariantsTvfrr400BadRequest> for TryPostResponseVariants {
//     fn from(value: TryPostResponseVariantsTvfrr400BadRequest) -> Self {
//         match value {
//             TryPostResponseVariantsTvfrr400BadRequest::ProjectCommitExtractorNotEqual {
//                 project_commit_not_equal,
//                 project_commit_to_use,
//                 code_occurence,
//             } => Self::ProjectCommitExtractorNotEqual {
//                 project_commit_not_equal,
//                 project_commit_to_use,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr400BadRequest::ProjectCommitExtractorToStrConversion {
//                 project_commit_to_str_conversion,
//                 code_occurence,
//             } => Self::ProjectCommitExtractorToStrConversion {
//                 project_commit_to_str_conversion,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr400BadRequest::NoProjectCommitExtractorHeader {
//                 no_project_commit_header,
//                 code_occurence,
//             } => Self::NoProjectCommitExtractorHeader {
//                 no_project_commit_header,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr400BadRequest::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             } => Self::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr400BadRequest::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             } => Self::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryPostResponseVariantsTvfrr500InternalServerError {
//     Configuration {
//         configuration_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Database {
//         box_dyn_database_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Io {
//         io_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Tls {
//         box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Protocol {
//         protocol: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     ColumnIndexOutOfBounds {
//         column_index_out_of_bounds: usize,
//         len: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     ColumnDecode {
//         column_decode_index: std::string::String,
//         source_handle: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Decode {
//         decode_box_dyn_error: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     PoolClosed {
//         pool_closed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     WorkerCrashed {
//         worker_crashed: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     Migrate {
//         migrate: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
//     UnexpectedCase {
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
// }
// impl std::convert::From<TryPostResponseVariantsTvfrr500InternalServerError>
//     for TryPostResponseVariants
// {
//     fn from(value: TryPostResponseVariantsTvfrr500InternalServerError) -> Self {
//         match value {
//             TryPostResponseVariantsTvfrr500InternalServerError::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             } => Self::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr500InternalServerError::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             } => Self::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr500InternalServerError::Io {
//                 io_error,
//                 code_occurence,
//             } => Self::Io {
//                 io_error,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr500InternalServerError::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             } => Self::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr500InternalServerError::Protocol {
//                 protocol,
//                 code_occurence,
//             } => Self::Protocol {
//                 protocol,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr500InternalServerError::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             } => Self::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr500InternalServerError::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             } => Self::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr500InternalServerError::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             } => Self::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr500InternalServerError::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             } => Self::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr500InternalServerError::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             } => Self::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr500InternalServerError::Migrate {
//                 migrate,
//                 code_occurence,
//             } => Self::Migrate {
//                 migrate,
//                 code_occurence,
//             },
//             TryPostResponseVariantsTvfrr500InternalServerError::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryPostResponseVariantsTvfrr404NotFound {
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
// }
// impl std::convert::From<TryPostResponseVariantsTvfrr404NotFound> for TryPostResponseVariants {
//     fn from(value: TryPostResponseVariantsTvfrr404NotFound) -> Self {
//         match value {
//             TryPostResponseVariantsTvfrr404NotFound::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryPostResponseVariantsTvfrr408RequestTimeout {
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
//     },
// }
// impl std::convert::From<TryPostResponseVariantsTvfrr408RequestTimeout> for TryPostResponseVariants {
//     fn from(value: TryPostResponseVariantsTvfrr408RequestTimeout) -> Self {
//         match value {
//             TryPostResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             },
//         }
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// enum TryPostResponseVariantsTvfrr201Created {
//     DesirableType(PostDesirableType),
// }
// impl std::convert::From<TryPostResponseVariantsTvfrr201Created> for TryPostResponseVariants {
//     fn from(value: TryPostResponseVariantsTvfrr201Created) -> Self {
//         match value {
//             TryPostResponseVariantsTvfrr201Created::DesirableType(i) => Self::DesirableType(i),
//         }
//     }
// }

// async fn try_from_response(
//     response: reqwest::Response,
// ) -> Result<
//     TryPostResponseVariants,
//     crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
// > {
//     let status_code = response.status();
//     let headers = response.headers().clone();
//     let response_text = response.text().await.unwrap_or_else(|_| {
//         std::string::String::from(crate::global_variables::hardcode::FAILED_TO_GET_RESPONSE_TEXT)
//     });
//     if status_code == http::StatusCode::CREATED {
//         match serde_json :: from_str :: <
//             TryPostResponseVariantsTvfrr201Created > (& response_text)
//             {
//                 Ok(value) => Ok(TryPostResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text }),
//             }
//     } else if status_code == http::StatusCode::BAD_REQUEST {
//         match serde_json :: from_str :: <
//             TryPostResponseVariantsTvfrr400BadRequest > (& response_text)
//             {
//                 Ok(value) => Ok(TryPostResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text },),
//             }
//     } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
//         match serde_json :: from_str :: <
//             TryPostResponseVariantsTvfrr500InternalServerError >
//             (& response_text)
//             {
//                 Ok(value) => Ok(TryPostResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text },),
//             }
//     } else if status_code == http::StatusCode::NOT_FOUND {
//         match serde_json :: from_str :: <
//             TryPostResponseVariantsTvfrr404NotFound > (& response_text)
//             {
//                 Ok(value) => Ok(TryPostResponseVariants :: from(value)),
//                 Err(e) =>
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde : e, status_code, headers, response_text },),
//             }
//     } else {
//         Err(
//             crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::StatusCode {
//                 status_code,
//                 headers,
//                 response_text,
//             },
//         )
//     }
// }
// // impl std::convert::TryFrom<reqwest::Response> for TryPostResponseVariants {
// //     type Error = crate::common::api_request_unexpected_error::ApiRequestUnexpectedError;
// //     fn try_from(response: reqwest::Response) -> Result<Self, Self::Error> {
// //         let status_code = response.status();
// //         let headers = response.headers().clone();
// //         let response_text = futures::executor::block_on(response.text()).unwrap_or_else(|_| {
// //             std::string::String::from(
// //                 crate::global_variables::hardcode::FAILED_TO_GET_RESPONSE_TEXT,
// //             )
// //         });
// //         if status_code == http::StatusCode::CREATED {
// //             match serde_json :: from_str :: <
// //             TryPostResponseVariantsTvfrr201Created > (& response_text)
// //             {
// //                 Ok(value) => Ok(TryPostResponseVariants :: from(value)),
// //                 Err(e) =>
// //                 Err(crate :: common :: api_request_unexpected_error ::
// //                 ApiRequestUnexpectedError :: DeserializeBody
// //                 { serde : e, status_code, headers, response_text }),
// //             }
// //         } else if status_code == http::StatusCode::BAD_REQUEST {
// //             match serde_json :: from_str :: <
// //             TryPostResponseVariantsTvfrr400BadRequest > (& response_text)
// //             {
// //                 Ok(value) => Ok(TryPostResponseVariants :: from(value)),
// //                 Err(e) =>
// //                 Err(crate :: common :: api_request_unexpected_error ::
// //                 ApiRequestUnexpectedError :: DeserializeBody
// //                 { serde : e, status_code, headers, response_text },),
// //             }
// //         } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
// //             match serde_json :: from_str :: <
// //             TryPostResponseVariantsTvfrr500InternalServerError >
// //             (& response_text)
// //             {
// //                 Ok(value) => Ok(TryPostResponseVariants :: from(value)),
// //                 Err(e) =>
// //                 Err(crate :: common :: api_request_unexpected_error ::
// //                 ApiRequestUnexpectedError :: DeserializeBody
// //                 { serde : e, status_code, headers, response_text },),
// //             }
// //         } else if status_code == http::StatusCode::NOT_FOUND {
// //             match serde_json :: from_str :: <
// //             TryPostResponseVariantsTvfrr404NotFound > (& response_text)
// //             {
// //                 Ok(value) => Ok(TryPostResponseVariants :: from(value)),
// //                 Err(e) =>
// //                 Err(crate :: common :: api_request_unexpected_error ::
// //                 ApiRequestUnexpectedError :: DeserializeBody
// //                 { serde : e, status_code, headers, response_text },),
// //             }
// //         } else {
// //             Err(crate :: common :: api_request_unexpected_error ::
// //             ApiRequestUnexpectedError :: StatusCode
// //             { status_code, headers, response_text },)
// //         }
// //     }
// // }
// impl TryFrom<TryPostResponseVariants> for PostDesirableType {
//     type Error = TryPostWithSerializeDeserialize;
//     fn try_from(value: TryPostResponseVariants) -> Result<Self, Self::Error> {
//         match value {
//             TryPostResponseVariants::DesirableType(i) => Ok(i),
//             TryPostResponseVariants::ProjectCommitExtractorNotEqual {
//                 project_commit_not_equal,
//                 project_commit_to_use,
//                 code_occurence,
//             } => Err(
//                 TryPostWithSerializeDeserialize::ProjectCommitExtractorNotEqual {
//                     project_commit_not_equal,
//                     project_commit_to_use,
//                     code_occurence,
//                 },
//             ),
//             TryPostResponseVariants::ProjectCommitExtractorToStrConversion {
//                 project_commit_to_str_conversion,
//                 code_occurence,
//             } => Err(
//                 TryPostWithSerializeDeserialize::ProjectCommitExtractorToStrConversion {
//                     project_commit_to_str_conversion,
//                     code_occurence,
//                 },
//             ),
//             TryPostResponseVariants::NoProjectCommitExtractorHeader {
//                 no_project_commit_header,
//                 code_occurence,
//             } => Err(
//                 TryPostWithSerializeDeserialize::NoProjectCommitExtractorHeader {
//                     no_project_commit_header,
//                     code_occurence,
//                 },
//             ),
//             TryPostResponseVariants::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::Configuration {
//                 configuration_box_dyn_error,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::Database {
//                 box_dyn_database_error,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::Io {
//                 io_error,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::Io {
//                 io_error,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::Tls {
//                 box_dyn_error,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::Protocol {
//                 protocol,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::Protocol {
//                 protocol,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::Decode {
//                 decode_box_dyn_error,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::Migrate {
//                 migrate,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::Migrate {
//                 migrate,
//                 code_occurence,
//             }),
//             TryPostResponseVariants::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Err(TryPostWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             }),
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
// pub enum TryPostErrorNamed<'a> {
//     ExpectedType {
//         #[eo_display_with_serialize_deserialize]
//         get: TryPostWithSerializeDeserialize,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     UnexpectedStatusCode {
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     DeserializeResponse {
//         #[eo_display]
//         serde: serde_json::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     Reqwest {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }
// async fn extraction_logic<'a>(
//     future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
// ) -> Result<PostDesirableType, TryPostErrorNamed<'a>> {
//     match future.await
//     {
//         Ok(response) => match try_from_response(response).await
//         // TryPostResponseVariants :: try_from(response)
//         {
//             Ok(variants) => match PostDesirableType :: try_from(variants)
//             {
//                 Ok(value) => Ok(value), Err(e) =>
//                 Err(TryPostErrorNamed :: ExpectedType
//                 {
//                     get : e, code_occurence : crate ::
//                     code_occurence_tufa_common! (),
//                 }),
//             }, Err(e) => match e
//             {
//                 crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: StatusCode
//                 { status_code, headers, response_text, } =>
//                 Err(TryPostErrorNamed :: UnexpectedStatusCode
//                 {
//                     status_code, headers, response_text, code_occurence : crate
//                     :: code_occurence_tufa_common! ()
//                 }), crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: DeserializeBody
//                 { serde, status_code, headers, response_text, } =>
//                 Err(TryPostErrorNamed :: DeserializeResponse
//                 {
//                     serde, status_code, headers, response_text, code_occurence :
//                     crate :: code_occurence_tufa_common! ()
//                 }),
//             },
//         }, Err(e) =>
//         Err(TryPostErrorNamed :: Reqwest
//         {
//             reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
//             (),
//         }),
//     }
// }
// pub enum TryPostStatusCodesChecker {
//     ProjectCommitExtractorNotEqualTvfrr400BadRequest,
//     ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
//     NoProjectCommitExtractorHeaderTvfrr400BadRequest,
//     ConfigurationTvfrr500InternalServerError,
//     DatabaseTvfrr500InternalServerError,
//     IoTvfrr500InternalServerError,
//     TlsTvfrr500InternalServerError,
//     ProtocolTvfrr500InternalServerError,
//     RowNotFoundTvfrr404NotFound,
//     TypeNotFoundTvfrr400BadRequest,
//     ColumnIndexOutOfBoundsTvfrr500InternalServerError,
//     ColumnNotFoundTvfrr400BadRequest,
//     ColumnDecodeTvfrr500InternalServerError,
//     DecodeTvfrr500InternalServerError,
//     PoolTimedOutTvfrr408RequestTimeout,
//     PoolClosedTvfrr500InternalServerError,
//     WorkerCrashedTvfrr500InternalServerError,
//     MigrateTvfrr500InternalServerError,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
