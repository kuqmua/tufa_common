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
