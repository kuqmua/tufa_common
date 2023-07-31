pub mod delete;
pub mod delete_by_id;
pub mod get;
pub mod get_by_id;
pub mod patch;
pub mod post;
pub mod put;

pub static CATS: &str = "cats";

pub trait GetConfigGetPostgresPool:
    crate::repositories_types::tufa_server::config::config_struct::GetConfig
    + crate::server::routes::helpers::get_postgres_pool::GetPostgresPool
{
}

pub type DynArcGetConfigGetPostgresPoolSendSync = std::sync::Arc<
    dyn crate::repositories_types::tufa_server::routes::api::cats::GetConfigGetPostgresPool
        + Send
        + Sync,
>;

///////////////////////////

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Cat {
    pub id: i64, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    pub name: String,
    pub color: String,
}

#[derive(serde::Deserialize)]
pub struct DeleteByIdPathParameters {
    pub id: i64,
}

#[derive(serde::Deserialize)]
pub struct DeleteQueryParameters {
    pub name: Option<String>,
    pub color: Option<String>,
}

impl crate::common::url_encode::UrlEncode for DeleteQueryParameters {
    fn url_encode(&self) -> String {
        let parameters = match (&self.name, &self.color) {
            (None, None) => String::from(""),
            (None, Some(color)) => format!("color={}", urlencoding::encode(color)),
            (Some(name), None) => format!("name={}", urlencoding::encode(name)),
            (Some(name), Some(color)) => format!(
                "name={}&color={}",
                urlencoding::encode(name),
                urlencoding::encode(color)
            ),
        };
        match parameters.is_empty() {
            true => String::from(""),
            false => format!("?{parameters}"),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct GetByIdPathParameters {
    pub id: i64,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum CatToPatch {
    IdName { id: i64, name: String },
    IdColor { id: i64, color: String },
}

impl crate::server::postgres::get_postgres_bigserial_id::GetPostgresBigserialId for CatToPatch {
    fn get_postgres_bigserial_id(&self) -> &i64 {
        match self {
            CatToPatch::IdName { id, name: _name } => id,
            CatToPatch::IdColor { id, color: _color } => id,
        }
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatToPost {
    pub name: String,
    pub color: String,
}

#[derive(serde::Deserialize)]
pub struct GetQueryParameters {
    pub limit: Option<crate::server::postgres::rows_per_table::RowsPerTable>,
    pub name: Option<std::string::String>,
    pub color: Option<std::string::String>,
}

//todo - make a macro for it?
impl crate::common::url_encode::UrlEncode for GetQueryParameters {
    fn url_encode(&self) -> String {
        let parameters = match (&self.limit, &self.name, &self.color) {
            (None, None, None) => String::from(""),
            (None, None, Some(color)) => format!("color={}", urlencoding::encode(color)),
            (None, Some(name), None) => format!("name={}", urlencoding::encode(name)),
            (None, Some(name), Some(color)) => format!(
                "name={}&color={}",
                urlencoding::encode(name),
                urlencoding::encode(color)
            ),
            (Some(limit), None, None) => format!("limit={limit}"),
            (Some(limit), None, Some(color)) => format!(
                "limit={}&color={}",
                urlencoding::encode(&limit.to_string()),
                urlencoding::encode(color)
            ),
            (Some(limit), Some(name), None) => format!(
                "limit={}&name={}",
                urlencoding::encode(&limit.to_string()),
                urlencoding::encode(name)
            ),
            (Some(limit), Some(name), Some(color)) => {
                format!(
                    "limit={}&name={}&color={}",
                    urlencoding::encode(&limit.to_string()),
                    urlencoding::encode(name),
                    urlencoding::encode(color)
                )
            }
        };
        match parameters.is_empty() {
            true => String::from(""),
            false => format!("?{parameters}"),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct GetQueryParametersSecond {
    pub limit: Option<crate::server::postgres::rows_per_table::RowsPerTable>,
    pub filter: Option<GetFilter>,
    pub select: Option<GetSelect>,
}

#[derive(serde::Deserialize)]
pub struct GetFilter {
    // pub ids: Option<Vec<i64>>,
    pub name: Option<std::string::String>,
    pub color: Option<std::string::String>,
}
//
#[derive(serde::Deserialize)]
pub enum GetSelect {
    #[serde(rename(deserialize = "id"))]
    Id,
    #[serde(rename(deserialize = "name"))]
    Name,
    #[serde(rename(deserialize = "color"))]
    Color,
    #[serde(rename(deserialize = "idname"))]
    IdName,
    #[serde(rename(deserialize = "idcolor"))]
    IdColor,
    #[serde(rename(deserialize = "namecolor"))]
    NameColor,
    #[serde(rename(deserialize = "idnamecolor"))]
    IdNameColor,
}
