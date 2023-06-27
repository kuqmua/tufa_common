#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum PostHttpResponseVariants {
    Ok,
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

#[derive(
    Debug, thiserror::Error, error_occurence::ErrorOccurence, from_enum::FromEnumWithLifetime,
)]
#[from_enum::from_enum_paths_with_lifetime(PostHttpResponseVariants)]
pub enum TryPostErrorHttpResponse<'a> {
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

///////////////////////////

impl<'a>
    From<crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed<'a>>
    for actix_web::HttpResponse
{
    fn from(
        val: crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed<
            'a,
        >,
    ) -> Self {
        let mut actix_web_http_response: actix_web::HttpResponseBuilder = (&val).into();
        actix_web_http_response.json(actix_web::web::Json(
            val.into_serialize_deserialize_version(),
        ))
    }
}

impl<'a>
    From<
        &crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed<'a>,
    > for actix_web::HttpResponseBuilder
{
    fn from(
        val: &crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed<'a>,
    ) -> Self {
        match &val {
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::Configuration {
                configuration_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::Database {
                box_dyn_database_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::Io {
                io_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::Tls {
                box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::Protocol {
                protocol: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::NotFound(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => actix_web::HttpResponse::BadRequest(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::Decode {
                decode_box_dyn_error: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => actix_web::HttpResponse::RequestTimeout(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::Migrate {
                migrate: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
            crate::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => actix_web::HttpResponse::InternalServerError(),
        }
    }
}
//

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryPostErrorNamed<'a> {
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    UnexpectedStatusCode {
        #[eo_display]
        unexpected_status_code: http::StatusCode,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn try_post<'a>(
    server_location: std::string::String,
    cat: crate::repositories_types::tufa_server::routes::api::cats::post::CatToPost,
) -> Result<(), TryPostErrorNamed<'a>> {
    let stringified_json = match serde_json::to_string(&cat) {
        Ok(stringified_json) => stringified_json,
        Err(e) => {
            return Err(TryPostErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }
    };
    let url = format!(
        "{server_location}/api/{}/",
        crate::repositories_types::tufa_server::routes::api::cats::CATS
    );
    match reqwest::Client::new()
        .post(&url)
        .header(
            crate::common::git::project_git_info::PROJECT_COMMIT,
            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                .project_commit,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(stringified_json)
        .send()
        .await
    {
        Ok(r) => {
            let response_status = r.status();
            match response_status == http::StatusCode::CREATED {
                true => Ok(()),
                false => Err(TryPostErrorNamed::UnexpectedStatusCode {
                    unexpected_status_code: response_status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            }
        }
        Err(e) => Err(TryPostErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
