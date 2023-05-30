#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Cat {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatToInsert {
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
pub struct SelectQueryParameters {
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
    match reqwest::get(&format!("{server_location}/api/cats/")).await {
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

pub static DEFAULT_SELECT_ALL_LIMIT: u32 = 10;

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetErrorNamed<'a> {
    PostgresSelect {
        #[eo_display]
        postgres_select: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(serde::Deserialize)]
pub struct SelectByIdPathParameters {
    pub id: i64,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresSelectErrorNamed<'a> {
    SelectCat {
        #[eo_display]
        select_cat: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum SelectByIdErrorNamed<'a> {
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
pub enum CreateErrorNamed<'a> {
    PostgresInsert {
        #[eo_display]
        postgres_insert: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub enum CreateRouteHttpResponseBuilder<'a> {
    Created,
    InternalServerError(CreateErrorNamed<'a>),
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

#[derive(serde::Deserialize)]
pub struct DeleteWhereQueryParameters {
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum DeleteWhereErrorNamed<'a> {
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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum UpsertErrorNamed<'a> {
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
