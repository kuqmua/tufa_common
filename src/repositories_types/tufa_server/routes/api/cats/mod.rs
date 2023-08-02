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
    + crate::common::config::config_fields::GetSourcePlaceType
    + crate::common::config::config_fields::GetTimezone
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

//
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl std::convert::From<Cat> for CatOptions {
    fn from(value: Cat) -> Self {
        CatOptions {
            id: Some(value.id),
            name: Some(value.name),
            color: Some(value.color),
        }
    }
}
impl std::convert::From<CatId> for CatOptions {
    fn from(value: CatId) -> Self {
        CatOptions {
            id: Some(value.id),
            name: None,
            color: None,
        }
    }
}
impl std::convert::From<CatName> for CatOptions {
    fn from(value: CatName) -> Self {
        CatOptions {
            id: None,
            name: Some(value.name),
            color: None,
        }
    }
}
impl std::convert::From<CatColor> for CatOptions {
    fn from(value: CatColor) -> Self {
        CatOptions {
            id: None,
            name: None,
            color: Some(value.color),
        }
    }
}
impl std::convert::From<CatIdName> for CatOptions {
    fn from(value: CatIdName) -> Self {
        CatOptions {
            id: Some(value.id),
            name: Some(value.name),
            color: None,
        }
    }
}
impl std::convert::From<CatIdColor> for CatOptions {
    fn from(value: CatIdColor) -> Self {
        CatOptions {
            id: Some(value.id),
            name: None,
            color: Some(value.color),
        }
    }
}
impl std::convert::From<CatNameColor> for CatOptions {
    fn from(value: CatNameColor) -> Self {
        CatOptions {
            id: None,
            name: Some(value.name),
            color: Some(value.color),
        }
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatId {
    pub id: i64,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatName {
    pub name: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatColor {
    pub color: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatIdName {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatIdColor {
    pub id: i64,
    pub color: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatNameColor {
    pub name: String,
    pub color: String,
}

//

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

#[derive(Debug, serde::Deserialize)]
pub struct GetQueryParameters {
    pub limit: Option<crate::server::postgres::rows_per_table::RowsPerTable>,
    pub name: Option<std::string::String>,
    pub color: Option<std::string::String>,
    pub select: GetSelect,
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
        let select_value = match self.select {
            GetSelect::Id => "id",
            GetSelect::Name => "name",
            GetSelect::Color => "color",
            GetSelect::IdName => "idname",
            GetSelect::IdColor => "idcolor",
            GetSelect::NameColor => "namecolor",
            GetSelect::IdNameColor => "idnamecolor",
        };
        format!("?{parameters}&select={select_value}")
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
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GetSelect {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id,
    #[serde(rename(serialize = "name", deserialize = "name"))]
    Name,
    #[serde(rename(serialize = "color", deserialize = "color"))]
    Color,
    #[serde(rename(serialize = "idname", deserialize = "idname"))]
    IdName,
    #[serde(rename(serialize = "idcolor", deserialize = "idcolor"))]
    IdColor,
    #[serde(rename(serialize = "namecolor", deserialize = "namecolor"))]
    NameColor,
    #[serde(rename(serialize = "idnamecolor", deserialize = "idnamecolor"))]
    IdNameColor,
}

impl std::fmt::Display for GetSelect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetSelect::Id => write!(f, "id"),
            GetSelect::Name => write!(f, "name"),
            GetSelect::Color => write!(f, "color"),
            GetSelect::IdName => write!(f, "id,name"),
            GetSelect::IdColor => write!(f, "id,color"),
            GetSelect::NameColor => write!(f, "name,color"),
            GetSelect::IdNameColor => write!(f, "id,name,color"),
        }
    }
}
// pub trait Something {
//     fn get_static_str(&self) -> &'static str;
// }

// impl Something for
