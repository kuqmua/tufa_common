#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum GetHttpResponseVariants {
    Cats(Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>),
    //
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
    //
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GetHttpResponseOkVariants {
    Cats(Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>),
}

//
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GetHttpResponseBadRequestVariants {
    Cats(Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>),
    //
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
    //
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
}
//
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum GetHttpResponseInternalServerErrorVariants {
    //
    //
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

#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum GetHttpResponseNotFoundVariants {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
}

#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum GetHttpResponseRequestTimeoutVariants {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
}

// impl TryFrom<reqwest::Response> for GetHttpResponseVariants {
//     type Error = String; //todo

//     fn try_from(value: reqwest::Response) -> Result<Self, Self::Error> {
//         let status_code = value.status();
//         if status_code == http::StatusCode::OK {

//         }
//         else if {

//         }
//         else if {

//         }
//         else if {

//         }
//         else if {

//         }
//         else if {

//         }
//         else if {

//         }
//         else if {

//         }
//         else {

//         }
//         todo!()
//     }
// }

impl From<&GetHttpResponseVariants> for http::StatusCode {
    fn from(val: &GetHttpResponseVariants) -> Self {
        match val {
            GetHttpResponseVariants::Cats(_) => http::StatusCode::OK,

            GetHttpResponseVariants::ProjectCommitExtractorNotEqual {
                project_commit_not_equal: _,
                project_commit_to_use: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            GetHttpResponseVariants::ProjectCommitExtractorToStrConversion {
                project_commit_to_str_conversion: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            GetHttpResponseVariants::NoProjectCommitExtractorHeader {
                no_project_commit_header: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,

            GetHttpResponseVariants::Configuration {
                configuration_box_dyn_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants::Database {
                box_dyn_database_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants::Io {
                io_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants::Tls {
                box_dyn_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants::Protocol {
                protocol: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => http::StatusCode::NOT_FOUND,
            GetHttpResponseVariants::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            GetHttpResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            GetHttpResponseVariants::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants::Decode {
                decode_box_dyn_error: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => http::StatusCode::REQUEST_TIMEOUT,
            GetHttpResponseVariants::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants::Migrate {
                migrate: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            GetHttpResponseVariants::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl
    std::convert::From<
        crate::repositories_types::tufa_server::routes::api::cats::get::route::GetHttpResponse,
    > for GetHttpResponseVariants
{
    fn from(
        val: crate::repositories_types::tufa_server::routes::api::cats::get::route::GetHttpResponse,
    ) -> Self {
        match val {
            super::route::GetHttpResponse::Cats(cats) => Self::Cats(cats),
            super::route::GetHttpResponse::Configuration {
                configuration_box_dyn_error,
                code_occurence,
            } => Self::Configuration {
                configuration_box_dyn_error,
                code_occurence,
            },
            super::route::GetHttpResponse::Database {
                box_dyn_database_error,
                code_occurence,
            } => Self::Database {
                box_dyn_database_error,
                code_occurence,
            },
            super::route::GetHttpResponse::Io {
                io_error,
                code_occurence,
            } => Self::Io {
                io_error,
                code_occurence,
            },
            super::route::GetHttpResponse::Tls {
                box_dyn_error,
                code_occurence,
            } => Self::Tls {
                box_dyn_error,
                code_occurence,
            },
            super::route::GetHttpResponse::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            super::route::GetHttpResponse::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
            super::route::GetHttpResponse::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            super::route::GetHttpResponse::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            super::route::GetHttpResponse::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            super::route::GetHttpResponse::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            super::route::GetHttpResponse::Decode {
                decode_box_dyn_error,
                code_occurence,
            } => Self::Decode {
                decode_box_dyn_error,
                code_occurence,
            },
            super::route::GetHttpResponse::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
            super::route::GetHttpResponse::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            super::route::GetHttpResponse::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            super::route::GetHttpResponse::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            super::route::GetHttpResponse::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}

#[derive(
    Debug, thiserror::Error, error_occurence::ErrorOccurence, from_enum::FromEnumWithLifetime,
)]
#[from_enum::from_enum_paths_with_lifetime(GetHttpResponseVariants)]
pub enum TryGetErrorHttpResponse<'a> {
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

impl TryFrom<GetHttpResponseVariants>
    for Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>
{
    type Error = TryGetErrorHttpResponseWithSerializeDeserialize;
    fn try_from(
        value: GetHttpResponseVariants,
    ) -> Result<Self, TryGetErrorHttpResponseWithSerializeDeserialize> {
        match value {
            GetHttpResponseVariants::Cats(cats) => Ok(cats),
            //
            GetHttpResponseVariants::ProjectCommitExtractorNotEqual {
                project_commit_not_equal,
                project_commit_to_use,
                code_occurence,
            } => Err(TryGetErrorHttpResponseWithSerializeDeserialize::ProjectCommitExtractorNotEqual { project_commit_not_equal, project_commit_to_use, code_occurence }),
            GetHttpResponseVariants::ProjectCommitExtractorToStrConversion {
                project_commit_to_str_conversion,
                code_occurence,
            } => Err(TryGetErrorHttpResponseWithSerializeDeserialize::ProjectCommitExtractorToStrConversion { project_commit_to_str_conversion, code_occurence }),
            GetHttpResponseVariants::NoProjectCommitExtractorHeader {
                no_project_commit_header,
                code_occurence,
            } => Err(TryGetErrorHttpResponseWithSerializeDeserialize::NoProjectCommitExtractorHeader { no_project_commit_header, code_occurence }),
            //
            GetHttpResponseVariants::Configuration {
                configuration_box_dyn_error,
                code_occurence,
            } => Err(
                TryGetErrorHttpResponseWithSerializeDeserialize::Configuration {
                    configuration_box_dyn_error,
                    code_occurence,
                },
            ),
            GetHttpResponseVariants::Database {
                box_dyn_database_error,
                code_occurence,
            } => Err(TryGetErrorHttpResponseWithSerializeDeserialize::Database {
                box_dyn_database_error,
                code_occurence,
            }),
            GetHttpResponseVariants::Io {
                io_error,
                code_occurence,
            } => Err(TryGetErrorHttpResponseWithSerializeDeserialize::Io {
                io_error,
                code_occurence,
            }),
            GetHttpResponseVariants::Tls {
                box_dyn_error,
                code_occurence,
            } => Err(TryGetErrorHttpResponseWithSerializeDeserialize::Tls {
                box_dyn_error,
                code_occurence,
            }),
            GetHttpResponseVariants::Protocol {
                protocol,
                code_occurence,
            } => Err(TryGetErrorHttpResponseWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            }),
            GetHttpResponseVariants::RowNotFound {
                row_not_found,
                code_occurence,
            } => Err(
                TryGetErrorHttpResponseWithSerializeDeserialize::RowNotFound {
                    row_not_found,
                    code_occurence,
                },
            ),
            GetHttpResponseVariants::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Err(
                TryGetErrorHttpResponseWithSerializeDeserialize::TypeNotFound {
                    type_not_found,
                    code_occurence,
                },
            ),
            GetHttpResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Err(
                TryGetErrorHttpResponseWithSerializeDeserialize::ColumnIndexOutOfBounds {
                    column_index_out_of_bounds,
                    len,
                    code_occurence,
                },
            ),
            GetHttpResponseVariants::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Err(
                TryGetErrorHttpResponseWithSerializeDeserialize::ColumnNotFound {
                    column_not_found,
                    code_occurence,
                },
            ),
            GetHttpResponseVariants::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Err(
                TryGetErrorHttpResponseWithSerializeDeserialize::ColumnDecode {
                    column_decode_index,
                    source_handle,
                    code_occurence,
                },
            ),
            GetHttpResponseVariants::Decode {
                decode_box_dyn_error,
                code_occurence,
            } => Err(TryGetErrorHttpResponseWithSerializeDeserialize::Decode {
                decode_box_dyn_error,
                code_occurence,
            }),
            GetHttpResponseVariants::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Err(
                TryGetErrorHttpResponseWithSerializeDeserialize::PoolTimedOut {
                    pool_timed_out,
                    code_occurence,
                },
            ),
            GetHttpResponseVariants::PoolClosed {
                pool_closed,
                code_occurence,
            } => Err(
                TryGetErrorHttpResponseWithSerializeDeserialize::PoolClosed {
                    pool_closed,
                    code_occurence,
                },
            ),
            GetHttpResponseVariants::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Err(
                TryGetErrorHttpResponseWithSerializeDeserialize::WorkerCrashed {
                    worker_crashed,
                    code_occurence,
                },
            ),
            GetHttpResponseVariants::Migrate {
                migrate,
                code_occurence,
            } => Err(TryGetErrorHttpResponseWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            }),
            GetHttpResponseVariants::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Err(
                TryGetErrorHttpResponseWithSerializeDeserialize::UnexpectedCase {
                    unexpected_case,
                    code_occurence,
                },
            ),
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGetErrorNamed<'a> {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        get: TryGetErrorHttpResponseWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn try_get<'a>(
    server_location: std::string::String, //todo server_location: std::string::String, 0 maybe change it to ip port
    query_parameters: crate::repositories_types::tufa_server::routes::api::cats::get::GetQueryParameters,
) -> Result<Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>, TryGetErrorNamed<'a>>
{
    let url = format!(
        "{server_location}/api/{}/{}",
        crate::repositories_types::tufa_server::routes::api::cats::CATS,
        crate::common::url_encode::UrlEncode::url_encode(&query_parameters)
    );
    println!(
        "try_get_project_commit {}",
        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.project_commit
    );
    match reqwest::Client::new()
        .get(&url)
        .header(
            crate::common::git::project_git_info::PROJECT_COMMIT,
            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                .project_commit,
        )
        .send()
        .await
    {
        //todo - expected status code body deserialization logic
        Ok(response) => {
            //
            let status_code = response.status();
            if status_code == http::StatusCode::OK {
                match response
                    .json::<Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>>()
                    .await
                {
                    Ok(_) => todo!(),
                    Err(_) => todo!(),
                }
            } else if status_code == http::StatusCode::BAD_REQUEST {
                todo!()
            } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
                todo!()
            } else if status_code == http::StatusCode::NOT_FOUND {
                todo!()
            } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
                todo!()
            } else {
                todo!()
            }
            //
            match response.json::<GetHttpResponseVariants>().await {
                Ok(varinats) => match Vec::<
                    crate::repositories_types::tufa_server::routes::api::cats::Cat,
                >::try_from(varinats)
                {
                    Ok(value) => Ok(value),
                    Err(e) => Err(TryGetErrorNamed::ExpectedType {
                        get: e,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    }),
                },
                Err(e) => Err(TryGetErrorNamed::Reqwest {
                    reqwest: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            }
        }
        Err(e) => Err(TryGetErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
