#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryDeleteByIdErrorNamed<'a> {
    BelowZero {
        #[eo_display_with_serialize_deserialize]
        below_zero: i64,
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

pub async fn try_delete_by_id<'a>(
    server_location: std::string::String,
    path_parameters: crate::repositories_types::tufa_server::routes::api::cats::delete_by_id::DeleteByIdPathParameters,
) -> Result<(), TryDeleteByIdErrorNamed<'a>> {
    if let true = path_parameters.id.is_negative() {
        return Err(TryDeleteByIdErrorNamed::BelowZero {
            below_zero: path_parameters.id,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    let url = format!(
        "{server_location}/api/{}/{}",
        crate::repositories_types::tufa_server::routes::api::cats::CATS,
        path_parameters.id
    );
    match reqwest::Client::new()
        .delete(&url)
        .header(
            crate::common::git::project_git_info::PROJECT_COMMIT,
            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                .project_commit,
        )
        .send()
        .await
    {
        Ok(r) => {
            let response_status = r.status();
            match response_status == http::StatusCode::OK {
                true => Ok(()),
                false => Err(TryDeleteByIdErrorNamed::UnexpectedStatusCode {
                    unexpected_status_code: response_status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            }
        }
        Err(e) => Err(TryDeleteByIdErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
