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
    pub select: GetSelectHandle, // GetSelect
}

//todo - make a macro for it?
impl crate::common::url_encode::UrlEncode for GetQueryParameters {
    fn url_encode(&self) -> std::string::String {
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
        // let select_value = match self.select {
        //     GetSelect::Id => "id",
        //     GetSelect::Name => "name",
        //     GetSelect::Color => "color",
        //     GetSelect::IdName => "idname",
        //     GetSelect::IdColor => "idcolor",
        //     GetSelect::NameColor => "namecolor",
        //     GetSelect::IdNameColor => "idnamecolor",
        // };
        format!("?{parameters}&select={}", self.select.url_encode())
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
        // match self {
        //     GetSelectField::Id => write!(f, "id"),
        //     GetSelectField::Name => write!(f, "name"),
        //     GetSelectField::Color => write!(f, "color"),
        // }
    }
}

impl crate::common::url_encode::UrlEncode for GetSelectField {
    fn url_encode(&self) -> std::string::String {
        //format!("color={}", urlencoding::encode(color))
        // format!("?{parameters}&select={select_value}")
        urlencoding::encode(&self.to_string()).to_string()
        // match &self {
        //     GetSelectField::Id => urlencoding::encode("id").to_string(),
        //     GetSelectField::Name => urlencoding::encode("name").to_string(),
        //     GetSelectField::Color => urlencoding::encode("color").to_string(),
        // }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetSelectHandle {
    selected: Vec<GetSelectField>,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryInitGetSelect<'a> {
    RepetitionOfPossibleValues {
        #[eo_display_with_serialize_deserialize]
        max_length: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl GetSelectHandle {
    pub fn try_init<'a>(
        possible_selected: Vec<GetSelectField>,
    ) -> Result<Vec<GetSelectField>, TryInitGetSelect<'a>> {
        let len = possible_selected.len();
        if let true = len <= GetSelectField::get_length() {
            Ok(possible_selected)
        } else {
            Err(TryInitGetSelect::RepetitionOfPossibleValues {
                max_length: len,
                code_occurence: crate::code_occurence_tufa_common!(),
            })
        }
    }
}

impl crate::common::url_encode::UrlEncode for GetSelectHandle {
    fn url_encode(&self) -> std::string::String {
        let mut folded =
            self.selected
                .iter()
                .fold(std::string::String::from(""), |mut acc, elem| {
                    acc.push_str(&format!("{},", elem.url_encode()));
                    acc
                });
        match folded.is_empty() {
            true => folded,
            false => {
                folded.pop();
                folded
            }
        }
    }
}
