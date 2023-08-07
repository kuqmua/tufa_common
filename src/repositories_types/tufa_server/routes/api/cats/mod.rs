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
    pub id: i64, //crate::server::postgres::bigserial::Bigserial //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    pub name: String,
    pub color: String,
}

//
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>, //crate::server::postgres::bigserial::Bigserial //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
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
    pub id: i64, //crate::server::postgres::bigserial::Bigserial
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
    pub id: i64, //crate::server::postgres::bigserial::Bigserial
    pub name: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatIdColor {
    pub id: i64, //crate::server::postgres::bigserial::Bigserial
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
    pub id: i64, //crate::server::postgres::bigserial::Bigserial
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

#[derive(Debug, serde::Deserialize)]
pub struct GetByIdPathParameters {
    // #[serde(deserialize_with = "deserialize_bigserialss")]
    pub id: crate::server::postgres::bigserial::Bigserial, //
}

// fn deserialize_bigserialss<'de, D>(deserializer: D) -> Result<i64, D::Error>
// where
//     D: serde::de::Deserializer<'de>,
// {
//     use serde::Deserialize;
//     let possible_bigserial = i64::deserialize(deserializer)?;
//     match possible_bigserial.is_positive() {
//         true => Ok(possible_bigserial),
//         false => Err(
//             serde::de::Error::custom(&format!(
//                 "invalid type: Postgresql Bigserial `{possible_bigserial}`, expected Postgresql Bigserial as rust i64, there 1 <= *your value* <= 9223372036854775807(only positive part of rust i64)"
//             )),
//         )
//     }
// }

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum CatToPatch {
    IdName {
        id: i64, //crate::server::postgres::bigserial::Bigserial
        name: String,
    },
    IdColor {
        id: i64, //crate::server::postgres::bigserial::Bigserial
        color: String,
    },
}

impl crate::server::postgres::bigserial::GetPostgresBigserialId for CatToPatch {
    fn get_postgres_bigserial_id(&self) -> &i64 {
        //crate::server::postgres::bigserial::Bigserial
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
    pub id: Option<i64>, //crate::server::postgres::bigserial::Bigserial //todo declare a postgres id type as i64 and reuse it later
    pub name: Option<std::string::String>,
    pub color: Option<std::string::String>,
    pub select: Option<GetSelect>,
}

//todo - make a macro for it?
impl crate::common::url_encode::UrlEncode for GetQueryParameters {
    fn url_encode(&self) -> std::string::String {
        let mut stringified_query_parameters = String::from("?");
        if let Some(limit) = &self.limit {
            let query_parameter_handle =
                format!("limit={}", urlencoding::encode(&limit.to_string())); //todo -maybe write macro for it
            match stringified_query_parameters.len() > 1 {
                true => {
                    stringified_query_parameters.push_str(&query_parameter_handle);
                }
                false => {
                    stringified_query_parameters.push_str(&format!("&{query_parameter_handle}"));
                }
            }
        }
        if let Some(id) = &self.id {
            let query_parameter_handle = format!("id={}", urlencoding::encode(&id.to_string()));
            match stringified_query_parameters.len() > 1 {
                true => {
                    stringified_query_parameters.push_str(&query_parameter_handle);
                }
                false => {
                    stringified_query_parameters.push_str(&format!("&{query_parameter_handle}"));
                }
            }
        }
        if let Some(name) = &self.name {
            let query_parameter_handle = format!("name={}", urlencoding::encode(name));
            match stringified_query_parameters.len() > 1 {
                true => {
                    stringified_query_parameters.push_str(&query_parameter_handle);
                }
                false => {
                    stringified_query_parameters.push_str(&format!("&{query_parameter_handle}"));
                }
            }
        }
        if let Some(color) = &self.color {
            let query_parameter_handle = format!("color={}", urlencoding::encode(color));
            match stringified_query_parameters.len() > 1 {
                true => {
                    stringified_query_parameters.push_str(&query_parameter_handle);
                }
                false => {
                    stringified_query_parameters.push_str(&format!("&{query_parameter_handle}"));
                }
            }
        }
        if let Some(select) = &self.select {
            let query_parameter_handle = format!("select={}", select.url_encode()); //urlencoding::encode(select)
            match stringified_query_parameters.len() > 1 {
                true => {
                    stringified_query_parameters.push_str(&query_parameter_handle);
                }
                false => {
                    stringified_query_parameters.push_str(&format!("&{query_parameter_handle}"));
                }
            }
        }
        stringified_query_parameters
    }
}

// #[derive(serde::Deserialize)]
// pub struct GetQueryParametersSecond {
//     pub limit: Option<crate::server::postgres::rows_per_table::RowsPerTable>,
//     pub filter: Option<GetFilter>,
//     pub select: Option<GetSelect>,
// }

// #[derive(serde::Deserialize)]
// pub struct GetFilter {
//     // pub ids: Option<Vec<i64>>,
//     pub name: Option<std::string::String>,
//     pub color: Option<std::string::String>,
// }

////////////////////
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    enum_extension::EnumExtension,
    strum_macros::EnumIter,
    PartialEq,
    Eq,
)]
pub enum GetSelectField {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id,
    #[serde(rename(serialize = "name", deserialize = "name"))]
    Name,
    #[serde(rename(serialize = "color", deserialize = "color"))]
    Color,
}

impl std::fmt::Display for GetSelectField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::to_lower_snake_case(self))
    }
}

impl crate::common::url_encode::UrlEncode for GetSelectField {
    fn url_encode(&self) -> std::string::String {
        urlencoding::encode(&self.to_string()).to_string()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GetSelect {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id,
    #[serde(rename(serialize = "name", deserialize = "name"))]
    Name,
    #[serde(rename(serialize = "color", deserialize = "color"))]
    Color,
    #[serde(rename(serialize = "id,name", deserialize = "id,name"))]
    IdName,
    #[serde(rename(serialize = "id,color", deserialize = "id,color"))]
    IdColor,
    #[serde(rename(serialize = "name,color", deserialize = "name,color"))]
    NameColor,
    #[serde(rename(serialize = "id,name,color", deserialize = "id,name,color"))]
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

impl crate::common::url_encode::UrlEncode for GetSelect {
    fn url_encode(&self) -> std::string::String {
        urlencoding::encode(&self.to_string()).to_string()
    }
}
