// pub enum CreateRouteHttpResponseBuilder<'a> {
//     Created,
//     InternalServerError(PostErrorNamed<'a>),
// }
pub static DEFAULT_SELECT_ALL_LIMIT: u32 = 10;
pub static API_USAGE_CHECKER: u64 = 18446744073709551615; //todo not a str coz dont want to deal with lifetimes yet //todo use github commit instead - just for testing need to change it it every time after commit in browser
pub static API_USAGE_CHECKER_DOES_NOT_MATCH_MESSAGE: &str = "please use special http request function from https://github.com/kuqmua/tufa_project for this API";

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Cat {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatToPost {
    pub name: String,
    pub color: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatToPatch {
    pub id: i64,
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct GetQueryParameters {
    pub check: u64,
    pub limit: Option<crate::server::postgres::rows_per_table::RowsPerTable>,
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGetCatsErrorNamed<'a> {
    Reqwest {
        #[eo_display_foreign_type]
        reqwest_get: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    DeserializeJson {
        #[eo_display_foreign_type]
        reqwest_get: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn try_get_cats<'a>(
    server_location: std::string::String,
) -> Result<Vec<Cat>, TryGetCatsErrorNamed<'a>> {
    //todo - add some constant in request and route for check if sent constant is queal to route constant so code users must use only one request implementation from there. (make constant private)
    //todo in the future add hash of git commit and private constant
    match reqwest::get(&format!(
        "{server_location}/api/cats/?check={API_USAGE_CHECKER}"
    ))
    .await
    {
        Ok(r) => match r.json::<Vec<Cat>>().await {
            Ok(vec_cats) => Ok(vec_cats),
            Err(e) => Err(TryGetCatsErrorNamed::DeserializeJson {
                reqwest_get: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        },
        Err(e) => Err(TryGetCatsErrorNamed::Reqwest {
            reqwest_get: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetErrorNamed<'a> {
    CheckApiUsage {
        #[eo_display_with_serialize_deserialize]
        check: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresSelect {
        #[eo_display]
        postgres_select: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresSelectErrorNamed<'a> {
    SelectCat {
        #[eo_display]
        select_cat: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

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
        #[eo_display]
        postgres_select: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostErrorNamed<'a> {
    PostgresInsert {
        #[eo_display]
        postgres_insert: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PutErrorNamed<'a> {
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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PatchErrorNamed<'a> {
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

#[derive(serde::Deserialize)]
pub struct DeleteQueryParameters {
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum DeleteErrorNamed<'a> {
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
        #[eo_display]
        postgres_delete: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
