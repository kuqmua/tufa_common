#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGetByIdErrorNamed<'a> {
    BelowZero {
        #[eo_display_with_serialize_deserialize]
        below_zero: i64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ExpectedServerError {
        #[eo_display_with_serialize_deserialize]
        expected_server_error: crate::repositories_types::tufa_server::routes::api::cats::get::request::GetByIdExpectedErrorStatusCodesErrorUnnamed,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ExpectedStatusCodeBodyConversion {
        #[eo_display]
        expected_status_code: http::StatusCode,
        #[eo_error_occurence]
        conversion_error: crate::repositories_types::tufa_server::routes::api::cats::get::request::GetByIdExpectedStatusCodesJsonConversionErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    UnexpectedStatusCode {
        //todo add headers? body? as Option<String>
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

pub async fn try_get_by_id<'a>(
    server_location: std::string::String,
    path_parameters: crate::repositories_types::tufa_server::routes::api::cats::get_by_id::GetByIdPathParameters,
) -> Result<crate::repositories_types::tufa_server::routes::api::cats::Cat, TryGetByIdErrorNamed<'a>>
{
    // todo maybe path_parameters already must be non zero?
    if let true = path_parameters.id.is_negative() {
        return Err(TryGetByIdErrorNamed::BelowZero {
            below_zero: path_parameters.id,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    let url = format!(
        "{server_location}/api/{}/{}",
        crate::repositories_types::tufa_server::routes::api::cats::CATS,
        path_parameters.id
    );
    // println!("{url}");
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
            match crate::repositories_types::tufa_server::routes::api::cats::get::request::GetByIdExpectedStatusCode::try_from(response.status()) {
                Ok(expected_status_code) => {
                    match expected_status_code.try_into_expected_type(response).await {
                        Ok(value) => Ok(value),
                        Err(error_result) => match error_result {
                            Ok(expected_server_error) => {
                                Err(TryGetByIdErrorNamed::ExpectedServerError {
                                    expected_server_error,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                })
                            }
                            Err(conversion_error) => {
                                let e = TryGetByIdErrorNamed::ExpectedStatusCodeBodyConversion {
                                    expected_status_code: response_status,
                                    conversion_error,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                                Err(e)
                            }
                        },
                    }
                }
                Err(unexpected_status_code) => Err(TryGetByIdErrorNamed::UnexpectedStatusCode {
                    unexpected_status_code,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            }
        }
        Err(e) => Err(TryGetByIdErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
