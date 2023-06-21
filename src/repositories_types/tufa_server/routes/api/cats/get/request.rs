#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGetErrorNamed<'a> {
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn try_get<'a>(
    server_location: std::string::String, //todo server_location: std::string::String, 0 maybe change it to ip port
    query_parameters: crate::repositories_types::tufa_server::routes::api::cats::get::GetQueryParameters,
) -> Result<Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>, TryGetErrorNamed<'a>>
{
    let url = format!(
        "{server_location}/api/{}/?{}",
        crate::repositories_types::tufa_server::routes::api::cats::CATS,
        query_parameters.to_string()
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
            let response_status = response.status();
            println!("try_get response status code {}", response.status());
            if response_status == http::StatusCode::OK {
                match response
                    .json::<Vec<crate::repositories_types::tufa_server::routes::api::cats::Cat>>()
                    .await
                {
                    Ok(vec_cats) => Ok(vec_cats),
                    Err(e) => Err(TryGetErrorNamed::Reqwest {
                        reqwest: e,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    }),
                }
            } else if response_status == http::StatusCode::BAD_REQUEST
                || response_status == http::StatusCode::INTERNAL_SERVER_ERROR
            {
                //todo - for each possibel status code try response.json::<TYPE>().await for different types ? then wrap it to enum?
                todo!()
            } else {
                Err(TryGetErrorNamed::UnexpectedStatusCode {
                    status_code: response_status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                })
            }
        }
        Err(e) => Err(TryGetErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
