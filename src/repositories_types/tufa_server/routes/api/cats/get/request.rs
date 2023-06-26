//
impl<'a> From<crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed<'a>> for actix_web::HttpResponse {
    fn from(val: crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed<'a>) -> Self {
        let mut actix_web_http_response: actix_web::HttpResponseBuilder = (&val).into();
        actix_web_http_response.json(actix_web::web::Json(
            val.into_serialize_deserialize_version(),
        ))
    }
}


impl<'a> From<&crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed<'a>> for actix_web::HttpResponseBuilder {
    fn from(val: &crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed<'a>) -> Self {
        match &val {
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Bigserial { bigserial: _, code_occurence: _ } => actix_web::HttpResponse::BadRequest(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Configuration {
                configuration_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Database {
                box_dyn_database_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Io {
                io_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Tls {
                box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Protocol {
                protocol: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::NotFound(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Decode {
                decode_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => actix_web::HttpResponse::RequestTimeout(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Migrate {
                migrate: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
        }
    }
}

#[derive(Debug)]
pub enum GetByIdExpectedStatusCode {
    Ok,
    BadRequest,
    InternalServerError,
}

impl std::convert::TryFrom<http::StatusCode> for GetByIdExpectedStatusCode {
    type Error = http::StatusCode;
    fn try_from(value: http::StatusCode) -> Result<Self, Self::Error> {
        if http::StatusCode::OK == value {
            Ok(Self::Ok)
        } else if http::StatusCode::BAD_REQUEST == value {
            Ok(Self::BadRequest)
        } else if http::StatusCode::INTERNAL_SERVER_ERROR == value {
            Ok(Self::InternalServerError)
        } else {
            Err(value)
        }
    }
}

impl GetByIdExpectedStatusCode {
    pub async fn try_into_expected_type<'a>(
        &self,
        response: reqwest::Response,
    ) -> Result<
        crate::repositories_types::tufa_server::routes::api::cats::Cat,
        Result<GetByIdExpectedErrorStatusCodesErrorUnnamed, GetByIdExpectedStatusCodesJsonConversionErrorNamed<'a>>,
    > {
        match self {
            GetByIdExpectedStatusCode::Ok => match response.json::<crate::repositories_types::tufa_server::routes::api::cats::Cat>().await {
                Ok(value) => Ok(value),
                Err(e) => Err(Err(GetByIdExpectedStatusCodesJsonConversionErrorNamed::Ok {
                    ok: e,
                    code_occurence: crate::code_occurence_tufa_common!()
                })),
            },
            GetByIdExpectedStatusCode::BadRequest => match response.json::<GetByIdStatusCodeBadRequestExpectedBodyTypeWithSerializeDeserialize>().await {
                Ok(bad_req) => Err(
                    Ok(
                        GetByIdExpectedErrorStatusCodesErrorUnnamed::BadRequest(bad_req)
                    )
                ),
                Err(e) => Err(
                    Err(
                        GetByIdExpectedStatusCodesJsonConversionErrorNamed::BadRequest { 
                            bad_request: e, 
                            code_occurence: crate::code_occurence_tufa_common!() 
                        }
                    )
                ),
            },
            GetByIdExpectedStatusCode::InternalServerError => match response.json::<GetByIdStatusCodeInternalServerErrorExpectedBodyTypeWithSerializeDeserialize>().await {
                Ok(internal_server_error) => Err(Ok(GetByIdExpectedErrorStatusCodesErrorUnnamed::InternalServerError(internal_server_error))),
                Err(e) => Err(Err(GetByIdExpectedStatusCodesJsonConversionErrorNamed::InternalServerError { 
                    internal_server_error: e, 
                    code_occurence: crate::code_occurence_tufa_common!() 
                })),
            },
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetByIdStatusCodeBadRequestExpectedBodyType<'a> {
    //todo struct concatination
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
    Bigserial {
        #[eo_error_occurence]
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetByIdStatusCodeInternalServerErrorExpectedBodyType<'a> {
    PostgresSelect {
        #[eo_display]
        postgres_select: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GetByIdExpectedErrorStatusCodesErrorUnnamed {
    BadRequest(GetByIdStatusCodeBadRequestExpectedBodyTypeWithSerializeDeserialize),
    InternalServerError(
        GetByIdStatusCodeInternalServerErrorExpectedBodyTypeWithSerializeDeserialize,
    ),
}

impl std::fmt::Display for GetByIdExpectedErrorStatusCodesErrorUnnamed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GetByIdExpectedErrorStatusCodesErrorUnnamed::BadRequest(e) => write!(f, "{e}"),
            GetByIdExpectedErrorStatusCodesErrorUnnamed::InternalServerError(e) => write!(f, "{e}"),
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetByIdExpectedStatusCodesJsonConversionErrorNamed<'a> {
    Ok {
        #[eo_display]
        ok: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    BadRequest {
        #[eo_display]
        bad_request: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    InternalServerError {
        #[eo_display]
        internal_server_error: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
//


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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGetErrorHttpResponse<'a> {
    //
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
        Ok(response) => match response.json::<GetHttpResponseVariants>().await {
            Ok(get_http_response) => match Vec::<
                crate::repositories_types::tufa_server::routes::api::cats::Cat,
            >::try_from(get_http_response)
            {
                Ok(vec_cats) => Ok(vec_cats),
                Err(e) => Err(TryGetErrorNamed::ExpectedType {
                    get: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            },
            Err(e) => Err(TryGetErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        },
        Err(e) => Err(TryGetErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
