//todo server_location: std::string::String, 0 maybe change it to ip port
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Cat {
    pub id: i64,
    pub name: String,
    pub color: String,
}
//////////////////////////////////////
#[derive(serde::Deserialize)]
pub struct GetQueryParameters {
    pub limit: Option<crate::server::postgres::rows_per_table::RowsPerTable>,
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetErrorNamed<'a> {
    CheckApiUsage {
        #[eo_display_with_serialize_deserialize]
        project_commit: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CannotConvertProjectCommitToStr {
        #[eo_display_with_serialize_deserialize]
        cannot_convert_project_commit_to_str: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NoProjectCommitHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresSelect {
        #[eo_display]
        postgres_select: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(serde::Deserialize)]
pub struct TryGetQueryParameters {
    pub limit: Option<crate::server::postgres::rows_per_table::RowsPerTable>,
    pub name: Option<String>,
    pub color: Option<String>,
}

impl std::string::ToString for TryGetQueryParameters {
    fn to_string(&self) -> String {
        match (&self.limit, &self.name, &self.color) {
            (None, None, None) => String::from(""),
            (None, None, Some(color)) => format!("color={color}"),
            (None, Some(name), None) => format!("name={name}"),
            (None, Some(name), Some(color)) => format!("name={name}&color={color}"),
            (Some(limit), None, None) => format!("limit={limit}"),
            (Some(limit), None, Some(color)) => format!("limit={limit}&color={color}"),
            (Some(limit), Some(name), None) => format!("limit={limit}&name={name}"),
            (Some(limit), Some(name), Some(color)) => {
                format!("limit={limit}&name={name}&color={color}")
            }
        }
    }
}

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
    server_location: std::string::String,
    query_parameters: TryGetQueryParameters,
) -> Result<Vec<Cat>, TryGetErrorNamed<'a>> {
    let query_parameters_stringified = query_parameters.to_string();
    let additional_query_parameters = match query_parameters_stringified.is_empty() {
        true => String::from(""),
        false => format!("&{query_parameters_stringified}"),
    };
    let url = format!(
        "{server_location}/api/cats/?project_commit={}{additional_query_parameters}",
        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.project_commit
    );
    match reqwest::get(&url).await {
        Ok(response) => {
            let response_status = response.status();
            println!("try_get response status code {}", response.status());
            if response_status == http::StatusCode::OK {
                match response.json::<Vec<Cat>>().await {
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
//////////////////////////////////////
#[derive(serde::Deserialize)]
pub struct GetByIdPathParameters {
    pub id: i64,
}

#[derive(serde::Deserialize)]
pub struct GetByIdQueryParameters {
    pub project_commit: String,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetByIdErrorNamed<'a> {
    CheckApiUsage {
        #[eo_display_with_serialize_deserialize]
        project_commit: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Bigserial {
        #[eo_error_occurence]
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresSelect {
        #[eo_display]
        postgres_select: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
//

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGetByIdErrorNamed<'a> {
    BelowZero {
        #[eo_display_with_serialize_deserialize]
        below_zero: i64,
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
    path_parameters: GetByIdPathParameters,
) -> Result<Cat, TryGetByIdErrorNamed<'a>> {
    if let true = path_parameters.id.is_negative() {
        return Err(TryGetByIdErrorNamed::BelowZero {
            below_zero: path_parameters.id,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    let url = format!(
        "{server_location}/api/cats/{}?project_commit={}",
        path_parameters.id,
        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.project_commit
    );
    match reqwest::get(&url).await {
        Ok(r) => match r.json::<Cat>().await {
            Ok(vec_cats) => Ok(vec_cats),
            Err(e) => Err(TryGetByIdErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        },
        Err(e) => Err(TryGetByIdErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
//////////////////////////////////////
#[derive(serde::Deserialize)]
pub struct PostQueryParameters {
    pub project_commit: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatToPost {
    pub name: String,
    pub color: String,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostErrorNamed<'a> {
    CheckApiUsage {
        #[eo_display_with_serialize_deserialize]
        project_commit: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresInsert {
        #[eo_display]
        postgres_insert: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
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
    cat: CatToPost,
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
        "{server_location}/api/cats/?project_commit={}",
        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.project_commit
    );
    match reqwest::Client::new()
        .post(&url)
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
//
//////////////////////////////////////
#[derive(serde::Deserialize)]
pub struct PutQueryParameters {
    pub project_commit: String,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PutErrorNamed<'a> {
    CheckApiUsage {
        #[eo_display_with_serialize_deserialize]
        project_commit: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Bigserial {
        #[eo_error_occurence]
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresInsertOrUpdate {
        #[eo_display]
        postgres_insert_or_update: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
//

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryPutErrorNamed<'a> {
    BelowZero {
        #[eo_display_with_serialize_deserialize]
        below_zero: i64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
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

pub async fn try_put<'a>(
    server_location: std::string::String,
    cat: Cat,
) -> Result<(), TryPutErrorNamed<'a>> {
    if let true = cat.id.is_negative() {
        return Err(TryPutErrorNamed::BelowZero {
            below_zero: cat.id,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    let stringified_json = match serde_json::to_string(&cat) {
        Ok(stringified_json) => stringified_json,
        Err(e) => {
            return Err(TryPutErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }
    };
    let url = format!(
        "{server_location}/api/cats/?project_commit={}",
        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.project_commit
    );
    match reqwest::Client::new()
        .put(&url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(stringified_json)
        .send()
        .await
    {
        Ok(r) => {
            let response_status = r.status();
            match response_status == http::StatusCode::OK {
                true => Ok(()),
                false => Err(TryPutErrorNamed::UnexpectedStatusCode {
                    unexpected_status_code: response_status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            }
        }
        Err(e) => Err(TryPutErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
//////////////////////////////////////
#[derive(serde::Deserialize)]
pub struct PatchQueryParameters {
    pub project_commit: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatToPatch {
    pub id: i64,
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PatchErrorNamed<'a> {
    CheckApiUsage {
        #[eo_display_with_serialize_deserialize]
        project_commit: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Bigserial {
        #[eo_error_occurence]
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NoParameters {
        #[eo_display_with_serialize_deserialize]
        no_parameters: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PleaseUsePut {
        #[eo_display_with_serialize_deserialize]
        please_use_put: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresUpdate {
        #[eo_display]
        postgres_update: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryPatchErrorNamed<'a> {
    BelowZero {
        #[eo_display_with_serialize_deserialize]
        below_zero: i64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
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

pub async fn try_patch<'a>(
    server_location: std::string::String,
    cat: CatToPatch,
) -> Result<(), TryPatchErrorNamed<'a>> {
    if let true = cat.id.is_negative() {
        return Err(TryPatchErrorNamed::BelowZero {
            below_zero: cat.id,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    let stringified_json = match serde_json::to_string(&cat) {
        Ok(stringified_json) => stringified_json,
        Err(e) => {
            return Err(TryPatchErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }
    };
    let url = format!(
        "{server_location}/api/cats/?project_commit={}",
        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.project_commit
    );
    match reqwest::Client::new()
        .patch(&url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(stringified_json)
        .send()
        .await
    {
        Ok(r) => {
            let response_status = r.status();
            match response_status == http::StatusCode::OK {
                true => Ok(()),
                false => Err(TryPatchErrorNamed::UnexpectedStatusCode {
                    unexpected_status_code: response_status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            }
        }
        Err(e) => Err(TryPatchErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
//////////////////////////////////////
#[derive(serde::Deserialize)]
pub struct DeleteQueryParameters {
    pub project_commit: String,
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum DeleteErrorNamed<'a> {
    CheckApiUsage {
        #[eo_display_with_serialize_deserialize]
        project_commit: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NoParameters {
        #[eo_display_with_serialize_deserialize]
        no_parameters: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresDelete {
        #[eo_display]
        postgres_delete: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
//
#[derive(serde::Deserialize)]
pub struct TryDeleteQueryParameters {
    pub name: Option<String>,
    pub color: Option<String>,
}

impl std::string::ToString for TryDeleteQueryParameters {
    fn to_string(&self) -> String {
        match (&self.name, &self.color) {
            (None, None) => String::from(""),
            (None, Some(color)) => format!("color={color}"),
            (Some(name), None) => format!("name={name}"),
            (Some(name), Some(color)) => format!("name={name}&color={color}"),
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryDeleteErrorNamed<'a> {
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

pub async fn try_delete<'a>(
    server_location: std::string::String,
    query_parameters: TryDeleteQueryParameters,
) -> Result<(), TryDeleteErrorNamed<'a>> {
    let query_parameters_stringified = query_parameters.to_string();
    let additional_query_parameters = match query_parameters_stringified.is_empty() {
        true => String::from(""),
        false => format!("&{query_parameters_stringified}"),
    };
    let url = format!(
        "{server_location}/api/cats/?project_commit={}{additional_query_parameters}",
        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.project_commit
    );
    match reqwest::Client::new().delete(&url).send().await {
        Ok(r) => {
            let response_status = r.status();
            match response_status == http::StatusCode::OK {
                true => Ok(()),
                false => Err(TryDeleteErrorNamed::UnexpectedStatusCode {
                    unexpected_status_code: response_status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            }
        }
        Err(e) => Err(TryDeleteErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
//////////////////////////////////////
#[derive(serde::Deserialize)]
pub struct DeleteByIdPathParameters {
    pub id: i64,
}

#[derive(serde::Deserialize)]
pub struct DeleteByIdQueryParameters {
    pub project_commit: String,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum DeleteByIdErrorNamed<'a> {
    CheckApiUsage {
        #[eo_display_with_serialize_deserialize]
        project_commit: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Bigserial {
        #[eo_error_occurence]
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresDelete {
        #[eo_display]
        postgres_delete: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
//

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
    path_parameters: DeleteByIdPathParameters,
) -> Result<(), TryDeleteByIdErrorNamed<'a>> {
    if let true = path_parameters.id.is_negative() {
        return Err(TryDeleteByIdErrorNamed::BelowZero {
            below_zero: path_parameters.id,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    let url = format!(
        "{server_location}/api/cats/{}?project_commit={}",
        path_parameters.id,
        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.project_commit
    );
    match reqwest::Client::new().delete(&url).send().await {
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
//////////////////////////////////////
