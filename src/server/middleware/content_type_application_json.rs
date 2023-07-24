#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    // type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
//todo maybe add logic for generation (axum::http::StatusCode::BAD_REQUEST, axum::Json()) to type_variants_from_reqwest_response_from_checker_paths ?
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
//     crate::repositories_types::tufa_server::routes::api::cats::get::TryGet,
//     crate::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetById,
//     crate::repositories_types::tufa_server::routes::api::cats::post::TryPost
// )]
pub enum ContentTypeApplicationJsonErrorNamed<'a> {
    // #[tvfrr_400_bad_request]
    ContentTypeHeaderNotEqualToApplicationJson {
        #[eo_display_with_serialize_deserialize]
        given: &'a str,
        #[eo_display_with_serialize_deserialize]
        must_be: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    // #[tvfrr_400_bad_request]
    NoContentTypeHeader {
        #[eo_display_with_serialize_deserialize]
        no_content_type_header: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn content_type_application_json<B>(
    req: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<axum::response::Response, axum::response::Response> {
    let application_json_name = "application/json";
    match req
        .headers()
        .get(http::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
    {
        Some(project_commit_checker_header) => match project_commit_checker_header == application_json_name
        {
            true => Ok(next.run(req).await),
            false => Err(axum::response::IntoResponse::into_response((
                axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                axum::Json(
                    ContentTypeApplicationJsonErrorNamed::ContentTypeHeaderNotEqualToApplicationJson {
                        given: project_commit_checker_header,
                        must_be: application_json_name,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    }.into_serialize_deserialize_version()
                ),
            )))
        },
        None => Err(axum::response::IntoResponse::into_response((
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(
                ContentTypeApplicationJsonErrorNamed::NoContentTypeHeader {
                    no_content_type_header: "no_content_type_header",
                    code_occurence: crate::code_occurence_tufa_common!(),
                }.into_serialize_deserialize_version()
            ),
        )))
    }
}
