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

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Cat {
    pub id: i64, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    pub name: String,
    pub color: String,
}

//
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
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
impl std::convert::From<CatIdNameColor> for CatOptions {
    fn from(value: CatIdNameColor) -> Self {
        CatOptions {
            id: Some(value.id),
            name: Some(value.name),
            color: Some(value.color),
        }
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatId {
    pub id: i64,
}
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatName {
    pub name: String,
}
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatColor {
    pub color: String,
}
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatIdName {
    pub id: i64,
    pub name: String,
}
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatIdColor {
    pub id: i64,
    pub color: String,
}
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatNameColor {
    pub name: String,
    pub color: String,
}
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatIdNameColor {
    pub id: i64,
    pub name: String,
    pub color: String,
}

//

#[derive(serde::Deserialize)]
pub struct DeleteByIdPathParameters {
    pub id: crate::server::postgres::bigserial::Bigserial,
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
    pub id: crate::server::postgres::bigserial::Bigserial,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum CatToPatch {
    IdName {
        id: crate::server::postgres::bigserial::Bigserial,
        name: std::string::String,
    },
    IdColor {
        id: crate::server::postgres::bigserial::Bigserial,
        color: std::string::String,
    },
}

impl crate::server::postgres::bigserial::GetPostgresBigserialId for CatToPatch {
    fn get_postgres_bigserial_id(&self) -> &crate::server::postgres::bigserial::Bigserial {
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

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatToPut {
    pub id: crate::server::postgres::bigserial::Bigserial, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    pub name: String,
    pub color: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BigserialIds(
    #[serde(deserialize_with = "deserialize_bigserial_ids")]
    pub  Vec<crate::server::postgres::bigserial::Bigserial>,
);

fn deserialize_bigserial_ids<'de, D>(
    deserializer: D,
) -> Result<Vec<crate::server::postgres::bigserial::Bigserial>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let (vec_values, mut stringified_parse_fails, mut stringified_bigserial_fails) = {
        use serde::Deserialize;
        String::deserialize(deserializer)?
    }
    .split(',')
    .fold(
        (
            Vec::new(),
            std::string::String::from(""),
            std::string::String::from(""),
        ),
        |mut acc, element| {
            match element.parse::<i64>() {
                Ok(value) => {
                    match crate::server::postgres::bigserial::Bigserial::try_from_i64(value) {
                        Ok(bigserial) => {
                            acc.0.push(bigserial);
                        }
                        Err(_) => {
                            acc.1.push_str(&format!("{element},"));
                        }
                    }
                }
                Err(_) => {
                    acc.1.push_str(&format!("{element},"));
                }
            }
            acc
        },
    );
    let default_message = "invalid type (expected array Postgresql Bigserial as rust Vec<i64>):";
    let stringified_parse_fails_message = "failed to parse each element into rust i64";
    let stringified_bigserial_fails_message = "failed to convert each element into Postgresql Bigserial - must be in range 1 <= *your value* <= 9223372036854775807(only positive part of rust i64)";
    match (
        stringified_parse_fails.is_empty(),
        stringified_bigserial_fails.is_empty(),
    ) {
        (true, true) => Ok(vec_values),
        (true, false) => {
            stringified_bigserial_fails.pop();
            Err(serde::de::Error::custom(
                &format!(
                    "{default_message} `{stringified_bigserial_fails}`, {stringified_bigserial_fails_message}")
                )
            )
        }
        (false, true) => {
            stringified_parse_fails.pop();
            Err(serde::de::Error::custom(&format!(
                "{default_message} `{stringified_parse_fails}`, {stringified_parse_fails_message}"
            )))
        }
        (false, false) => {
            stringified_parse_fails.pop();
            stringified_bigserial_fails.pop();
            Err(serde::de::Error::custom(
                &format!(
                    "{default_message} 1) `{stringified_parse_fails}`, {stringified_parse_fails_message}. 2) `{stringified_bigserial_fails}`, {stringified_bigserial_fails_message}")
                )
            )
        }
    }
}
//

//
#[derive(Debug, serde::Deserialize)]
pub struct GetQueryParameters {
    pub limit: crate::server::postgres::rows_per_table::RowsPerTable,
    pub id: Option<BigserialIds>,
    pub name: Option<crate::server::routes::helpers::strings_deserialized_from_string_splitted_by_comma::StringsDeserializedFromStringSplittedByComma>,
    pub color: Option<std::string::String>,
    pub select: Option<GetSelect>,
}

//todo - make a macro for it?
//todo - maybe some serde serialization like this https://docs.rs/url_serde/latest/url_serde/
impl crate::common::url_encode::UrlEncode for GetQueryParameters {
    fn url_encode(&self) -> std::string::String {
        let mut stringified_query_parameters = String::from("?");
        let limit_query_parameter_handle =
            format!("limit={}", urlencoding::encode(&self.limit.to_string())); //todo -maybe write macro for it
        stringified_query_parameters.push_str(&format!("&{limit_query_parameter_handle}"));
        if let Some(id) = &self.id {
            let ids_stringified = {
                let mut ids_stringified =
                    id.0.iter().fold(String::from(""), |mut acc, bigserial| {
                        acc.push_str(&format!("{bigserial},"));
                        acc
                    });
                if let false = ids_stringified.is_empty() {
                    ids_stringified.pop();
                }
                ids_stringified
            };
            let query_parameter_handle = format!("id={}", urlencoding::encode(&ids_stringified));
            stringified_query_parameters.push_str(&format!("&{query_parameter_handle}"));
        }
        if let Some(names) = &self.name {
            stringified_query_parameters.push_str(&format!(
                "&name={}",
                crate::common::url_encode::UrlEncode::url_encode(names)
            ));
        }
        if let Some(color) = &self.color {
            let query_parameter_handle = format!("color={}", urlencoding::encode(color));
            stringified_query_parameters.push_str(&format!("&{query_parameter_handle}"));
        }
        if let Some(select) = &self.select {
            let query_parameter_handle = format!("select={}", select.url_encode()); //urlencoding::encode(select)
            stringified_query_parameters.push_str(&format!("&{query_parameter_handle}"));
        }
        stringified_query_parameters
    }
}

impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery for GetQueryParameters {
    fn bind_sqlx_query<'q, TableScheme: for<'a> serde::Deserialize<'a>>(
        self,
        mut query: sqlx::query::QueryAs<
            'q,
            sqlx::Postgres,
            TableScheme,
            sqlx::postgres::PgArguments,
        >,
    ) -> sqlx::query::QueryAs<'q, sqlx::Postgres, TableScheme, sqlx::postgres::PgArguments> {
        if let Some(id) = self.id {
            for id_handle in id.0 {
                query = query.bind(id_handle.into_inner());
            }
        }
        //
        if let Some(names) = self.name {
            for name_handle in names.0 {
                query = query.bind(name_handle);
            }
        }
        if let Some(color) = self.color {
            query = query.bind(color);
        }
        query = query.bind(self.limit);
        query
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
    }
}

impl crate::common::url_encode::UrlEncode for GetSelectField {
    fn url_encode(&self) -> std::string::String {
        urlencoding::encode(&self.to_string()).to_string()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
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
impl std::default::Default for GetSelect {
    fn default() -> Self {
        Self::IdNameColor
    }
}
impl std::convert::From<Option<Self>> for GetSelect {
    fn from(option_value: Option<Self>) -> Self {
        match option_value {
            Some(value) => value,
            None => Self::default(),
        }
    }
}
impl crate::common::url_encode::UrlEncode for GetSelect {
    fn url_encode(&self) -> std::string::String {
        urlencoding::encode(&self.to_string()).to_string()
    }
}
impl GetSelect {
    pub fn into_get_select_field_vec(&self) -> Vec<GetSelectField> {
        match self {
            GetSelect::Id => vec![GetSelectField::Id],
            GetSelect::Name => vec![GetSelectField::Name],
            GetSelect::Color => vec![GetSelectField::Color],
            GetSelect::IdName => vec![GetSelectField::Id, GetSelectField::Name],
            GetSelect::IdColor => vec![GetSelectField::Id, GetSelectField::Color],
            GetSelect::NameColor => vec![GetSelectField::Name, GetSelectField::Color],
            GetSelect::IdNameColor => vec![
                GetSelectField::Id,
                GetSelectField::Name,
                GetSelectField::Color,
            ],
        }
    }
}
