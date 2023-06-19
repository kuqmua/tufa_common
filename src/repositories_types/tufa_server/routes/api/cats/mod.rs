//todo maybe use builder pattern for route request functions?
pub static CATS: &str = "cats";
//todo server_location: std::string::String, 0 maybe change it to ip port
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Cat {
    pub id: i64, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
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
//todo - or maybe write or find some trait for url encode
impl std::string::ToString for GetQueryParameters {
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
pub enum GetErrorNamed<'a> {
    PostgresSelect {
        #[eo_error_occurence]
        postgres_select: crate::server::postgres::sqlx_postgres_error::SqlxPostgresErrorErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> From<GetErrorNamed<'a>> for actix_web::HttpResponse {
    fn from(val: GetErrorNamed<'a>) -> Self {
        match &val {
            GetErrorNamed::PostgresSelect { postgres_select: _, code_occurence: _ } => actix_web::HttpResponse::BadRequest()
                .json(actix_web::web::Json(val.into_serialize_deserialize_version())),
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

/////////////
pub async fn try_get<'a>(
    server_location: std::string::String,
    query_parameters: GetQueryParameters,
) -> Result<Vec<Cat>, TryGetErrorNamed<'a>> {
    let url = format!(
        "{server_location}/api/{CATS}/?{}",
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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetByIdErrorNamed<'a> {
    Bigserial {
        #[eo_error_occurence]
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresSelect {
        #[eo_error_occurence]
        postgres_select: crate::server::postgres::sqlx_postgres_error::SqlxPostgresErrorErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> From<GetByIdErrorNamed<'a>> for actix_web::HttpResponse {
    fn from(val: GetByIdErrorNamed<'a>) -> Self {
        match &val {
            GetByIdErrorNamed::Bigserial { bigserial: _, code_occurence: _ } => actix_web::HttpResponse::BadRequest()
                .json(actix_web::web::Json(val.into_serialize_deserialize_version())),
            GetByIdErrorNamed::PostgresSelect { postgres_select: _, code_occurence: _ } => actix_web::HttpResponse::InternalServerError()
                .json(actix_web::web::Json(val.into_serialize_deserialize_version())),
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
        Cat,
        Result<GetByIdExpectedErrorStatusCodesErrorUnnamed, GetByIdExpectedStatusCodesJsonConversionErrorNamed<'a>>,
    > {
        match self {
            GetByIdExpectedStatusCode::Ok => match response.json::<Cat>().await {
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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGetByIdErrorNamed<'a> {
    BelowZero {
        #[eo_display_with_serialize_deserialize]
        below_zero: i64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ExpectedServerError {
        #[eo_display_with_serialize_deserialize]
        expected_server_error: GetByIdExpectedErrorStatusCodesErrorUnnamed,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ExpectedStatusCodeBodyConversion {
        #[eo_display]
        expected_status_code: http::StatusCode,
        #[eo_error_occurence]
        conversion_error: GetByIdExpectedStatusCodesJsonConversionErrorNamed<'a>,
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
    path_parameters: GetByIdPathParameters,
) -> Result<Cat, TryGetByIdErrorNamed<'a>> {
    // todo maybe path_parameters already must be non zero?
    if let true = path_parameters.id.is_negative() {
        return Err(TryGetByIdErrorNamed::BelowZero {
            below_zero: path_parameters.id,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    let url = format!("{server_location}/api/{CATS}/{}", path_parameters.id);
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
            match GetByIdExpectedStatusCode::try_from(response.status()) {
                Ok(expected_status_code) => match expected_status_code.try_into_expected_type(response).await {
                    Ok(value) => Ok(value),
                    Err(error_result) => match error_result {
                        Ok(expected_server_error) => Err(TryGetByIdErrorNamed::ExpectedServerError {
                            expected_server_error,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }),
                        Err(conversion_error) => {
                            let e = TryGetByIdErrorNamed::ExpectedStatusCodeBodyConversion {
                                expected_status_code: response_status,
                                conversion_error,
                                code_occurence: crate::code_occurence_tufa_common!(),
                            };
                            Err(e)
                        },
                    },
                },
                Err(unexpected_status_code) => Err(TryGetByIdErrorNamed::UnexpectedStatusCode {
                    unexpected_status_code,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            }
        },
        Err(e) => Err(TryGetByIdErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
//////////////////////////////////////
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatToPost {
    pub name: String,
    pub color: String,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostErrorNamed<'a> {
    PostgresInsert {
        #[eo_error_occurence]
        postgres_insert: crate::server::postgres::sqlx_postgres_error::SqlxPostgresErrorErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> From<PostErrorNamed<'a>> for actix_web::HttpResponse {
    fn from(val: PostErrorNamed<'a>) -> Self {
        match &val {
            PostErrorNamed::PostgresInsert { postgres_insert: _, code_occurence: _ } => actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(val.into_serialize_deserialize_version())),
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
    let url = format!("{server_location}/api/{CATS}/");
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
//
//////////////////////////////////////
#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PutErrorNamed<'a> {
    Bigserial {
        #[eo_error_occurence]
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresInsertOrUpdate {
        #[eo_error_occurence]
        postgres_insert_or_update: crate::server::postgres::sqlx_postgres_error::SqlxPostgresErrorErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> From<PutErrorNamed<'a>> for actix_web::HttpResponse {
    fn from(val: PutErrorNamed<'a>) -> Self {
        match &val {
            PutErrorNamed::Bigserial { 
                bigserial: _, 
                code_occurence: _ 
            } => actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(val.into_serialize_deserialize_version())),
            PutErrorNamed::PostgresInsertOrUpdate { 
                postgres_insert_or_update: _, 
                code_occurence: _ 
            } => actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(val.into_serialize_deserialize_version())),
        }
    }
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
    let url = format!("{server_location}/api/{CATS}/");
    match reqwest::Client::new()
        .put(&url)
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
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum CatToPatch {
    IdName { id: i64, name: String },
    IdColor { id: i64, color: String },
}

impl CatToPatch {
    pub fn get_id(&self) -> &i64 {
        match self {
            CatToPatch::IdName { id, name: _name } => id,
            CatToPatch::IdColor { id, color: _color } => id,
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PatchErrorNamed<'a> {
    Bigserial {
        #[eo_error_occurence]
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresUpdate {
        #[eo_error_occurence]
        postgres_update: crate::server::postgres::sqlx_postgres_error::SqlxPostgresErrorErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> From<PatchErrorNamed<'a>> for actix_web::HttpResponse {
    fn from(val: PatchErrorNamed<'a>) -> Self {
        match &val {
            PatchErrorNamed::Bigserial { 
                bigserial: _, 
                code_occurence: _ 
            } => actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(val.into_serialize_deserialize_version())),
            PatchErrorNamed::PostgresUpdate { 
                postgres_update: _, 
                code_occurence: _ 
            } => actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(val.into_serialize_deserialize_version())),
        }
    }
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
    let id = cat.get_id();
    if let true = id.is_negative() {
        return Err(TryPatchErrorNamed::BelowZero {
            below_zero: *id,
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
    let url = format!("{server_location}/api/{CATS}/",);
    match reqwest::Client::new()
        .patch(&url)
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
    pub name: Option<String>,
    pub color: Option<String>,
}

impl std::string::ToString for DeleteQueryParameters {
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
pub enum DeleteErrorNamed<'a> {
    NoParameters {
        #[eo_display_with_serialize_deserialize]
        no_parameters: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresDelete {
        #[eo_error_occurence]
        postgres_delete: crate::server::postgres::sqlx_postgres_error::SqlxPostgresErrorErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> From<DeleteErrorNamed<'a>> for actix_web::HttpResponse {
    fn from(val: DeleteErrorNamed<'a>) -> Self {
        match &val {
            DeleteErrorNamed::NoParameters { 
                no_parameters: _, 
                code_occurence: _ 
            } => actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(val.into_serialize_deserialize_version())),
            DeleteErrorNamed::PostgresDelete { 
                postgres_delete: _, 
                code_occurence: _ 
            } => actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(val.into_serialize_deserialize_version())),
        }
    }
}

//
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
    query_parameters: DeleteQueryParameters,
) -> Result<(), TryDeleteErrorNamed<'a>> {
    let url = format!(
        "{server_location}/api/{CATS}/?{}",
        query_parameters.to_string()
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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum DeleteByIdErrorNamed<'a> {
    Bigserial {
        #[eo_error_occurence]
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresDelete {
        #[eo_error_occurence]
        postgres_delete: crate::server::postgres::sqlx_postgres_error::SqlxPostgresErrorErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> From<DeleteByIdErrorNamed<'a>> for actix_web::HttpResponse {
    fn from(val: DeleteByIdErrorNamed<'a>) -> Self {
        match &val {
            DeleteByIdErrorNamed::Bigserial { 
                bigserial: _, 
                code_occurence: _ 
            } => actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(val.into_serialize_deserialize_version())),
            DeleteByIdErrorNamed::PostgresDelete { 
                postgres_delete: _,
                code_occurence: _ 
            } => actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(val.into_serialize_deserialize_version())),
        }
    }
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
    let url = format!("{server_location}/api/{CATS}/{}", path_parameters.id);
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
//////////////////////////////////////
