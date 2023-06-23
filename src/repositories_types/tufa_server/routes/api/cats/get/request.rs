#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGetErrorNamed<'a> {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        get: crate::repositories_types::tufa_server::routes::api::cats::get::route::GetErrorNamedWithSerializeDeserialize,
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
        Ok(response) => {
            match response
                .json::<crate::repositories_types::tufa_server::routes::api::cats::get::route::GetHttpResponse>()
                .await
            {
                Ok(get_http_response) => {
                    println!("{get_http_response:#?}");
                    match Vec::<crate::repositories_types::tufa_server::routes::api::cats::Cat>::try_from(get_http_response) {
                        Ok(vec_cats) => Ok(vec_cats),
                        Err(e) => Err(TryGetErrorNamed::ExpectedType { get: e, code_occurence: crate::code_occurence_tufa_common!() }),
                    }
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
