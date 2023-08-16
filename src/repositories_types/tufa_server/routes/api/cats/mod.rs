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

#[derive(
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    // generate_postgresql_crud::GeneratePostgresqlCrud,
)]
pub struct Cat {
    pub id: i64, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    pub name: String,
    pub color: String,
}

// #[derive(
//     Debug,
//     serde_derive::Serialize,
//     serde_derive::Deserialize,
//     // generate_postgresql_crud::GeneratePostgresqlCrud,
// )]
// pub struct CatTest {
//     pub one: i64, //1//todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
//     pub two: String, //3
//     pub three: String, //7
//                   // pub four: String, //15
//                   // pub five: String, //31
//                   // pub six: String, //63
//                   // pub seven: String, //127
// }

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

trait FromRow<'r, R: sqlx::Row>: Sized {
    fn sqlx_from_row(row: &'r R, select: &GetSelect) -> Result<Self, sqlx::Error>;
}
impl<'a, R: ::sqlx::Row> FromRow<'a, R> for CatOptions
where
    &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    Option<i64>: ::sqlx::decode::Decode<'a, R::Database>,
    Option<i64>: ::sqlx::types::Type<R::Database>,
    Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
    Option<String>: ::sqlx::types::Type<R::Database>,
    Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
    Option<String>: ::sqlx::types::Type<R::Database>,
{
    fn sqlx_from_row(row: &'a R, select: &GetSelect) -> ::sqlx::Result<Self> {
        let mut id: Option<i64> = None;
        let mut name: Option<String> = None;
        let mut color: Option<String> = None;
        match select {
            GetSelect::Id => {
                id = row.try_get("id")?;
            }
            GetSelect::Name => {
                name = row.try_get("name")?;
            }
            GetSelect::Color => {
                color = row.try_get("color")?;
            }
            GetSelect::IdName => {
                id = row.try_get("id")?;
                name = row.try_get("name")?;
            }
            GetSelect::IdColor => {
                id = row.try_get("id")?;
                color = row.try_get("color")?;
            }
            GetSelect::NameColor => {
                name = row.try_get("name")?;
                color = row.try_get("color")?;
            }
            GetSelect::IdNameColor => {
                id = row.try_get("id")?;
                name = row.try_get("name")?;
                color = row.try_get("color")?;
            }
        }
        Ok(CatOptions { id, name, color })
    }
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

#[derive(serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatId {
    pub id: i64,
}
#[derive(serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatName {
    pub name: String,
}
#[derive(serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatColor {
    pub color: String,
}
#[derive(serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatIdName {
    pub id: i64,
    pub name: String,
}
#[derive(serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatIdColor {
    pub id: i64,
    pub color: String,
}
#[derive(serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatNameColor {
    pub name: String,
    pub color: String,
}
#[derive(serde_derive::Serialize, serde_derive::Deserialize, sqlx::FromRow)]
pub struct CatIdNameColor {
    pub id: i64,
    pub name: String,
    pub color: String,
}

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

#[derive(Debug, serde::Deserialize)]
pub struct GetQueryParameters {
    pub select: Option<GetSelect>,
    pub limit: crate::server::postgres::rows_per_table::RowsPerTable,
    pub id: Option<crate::server::postgres::bigserial_ids::BigserialIds>,
    pub name: Option<crate::server::routes::helpers::strings_deserialized_from_string_splitted_by_comma::StringsDeserializedFromStringSplittedByComma>,
    pub color: Option<crate::server::routes::helpers::strings_deserialized_from_string_splitted_by_comma::StringsDeserializedFromStringSplittedByComma>,
}

//todo - make a macro for it?
//todo - maybe some serde serialization like this https://docs.rs/url_serde/latest/url_serde/
impl crate::common::url_encode::UrlEncode for GetQueryParameters {
    fn url_encode(&self) -> std::string::String {
        let mut stringified_query_parameters = String::from("?");
        if let Some(select) = &self.select {
            let query_parameter_handle = format!("select={}", select.url_encode()); //urlencoding::encode(select)
            stringified_query_parameters.push_str(&format!("&{query_parameter_handle}"));
        }
        let limit_query_parameter_handle =
            format!("limit={}", urlencoding::encode(&self.limit.to_string())); //todo -maybe write macro for it
        stringified_query_parameters.push_str(&format!("&{limit_query_parameter_handle}"));
        if let Some(value) = &self.id {
            stringified_query_parameters.push_str(&format!(
                "&id={}",
                crate::common::url_encode::UrlEncode::url_encode(value)
            ));
        }
        if let Some(value) = &self.name {
            stringified_query_parameters.push_str(&format!(
                "&name={}",
                crate::common::url_encode::UrlEncode::url_encode(value)
            ));
        }
        if let Some(value) = &self.color {
            stringified_query_parameters.push_str(&format!(
                "&color={}",
                crate::common::url_encode::UrlEncode::url_encode(value)
            ));
        }
        stringified_query_parameters
    }
}

impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery for GetQueryParameters {
    fn bind_sqlx_query(
        self,
        mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let Some(value) = self.id {
            query = value.bind_sqlx_query(query);
        }
        if let Some(value) = self.name {
            query = value.bind_sqlx_query(query);
        }
        if let Some(value) = self.color {
            query = value.bind_sqlx_query(query);
        }
        query = query.bind(self.limit);
        query
    }
}

impl crate::server::postgres::generate_where_get_parameters::GenerateWhereGetParametersBindPlaces
    for GetQueryParameters
{
    fn generate_where_get_parameters_bind_places(&self) -> std::string::String {
        // SELECT id,name,color FROM cats WHERE id = ANY(ARRAY[$1, $2, $3, $4]) AND name = ANY(ARRAY[$5, $6]) AND color = ANY(ARRAY[$7]) LIMIT $8
        // SELECT id,name,color FROM public.cats WHERE name LIKE 'test%' OR name LIKE '%patch%' ;
        let mut additional_parameters = std::string::String::from("");
        let mut increment: u64 = 0;
        if let Some(value) = &self.id {
            let prefix = match additional_parameters.is_empty() {
                true => format!("{}", crate::server::postgres::constants::WHERE_NAME),
                false => format!(" {}", crate::server::postgres::constants::AND_NAME),
            };
            additional_parameters.push_str(&format!(
            "{prefix} id = {}({}[{}])", 
            crate::server::postgres::constants::ANY_NAME,
            crate::server::postgres::constants::ARRAY_NAME,
            crate::server::postgres::generate_bind_increments::GenerateBindIncrements::generate_bind_increments(value, &mut increment)
        ));
        }
        if let Some(value) = &self.name {
            let prefix = match additional_parameters.is_empty() {
                true => format!("{}", crate::server::postgres::constants::WHERE_NAME),
                false => format!(" {}", crate::server::postgres::constants::AND_NAME),
            };
            additional_parameters.push_str(&format!(
            "{prefix} name = {}({}[{}])",
            crate::server::postgres::constants::ANY_NAME,
            crate::server::postgres::constants::ARRAY_NAME,
            crate::server::postgres::generate_bind_increments::GenerateBindIncrements::generate_bind_increments(value, &mut increment)
        ));
        }
        if let Some(value) = &self.color {
            let prefix = match additional_parameters.is_empty() {
                true => format!("{}", crate::server::postgres::constants::WHERE_NAME),
                false => format!(" {}", crate::server::postgres::constants::AND_NAME),
            };
            additional_parameters.push_str(&format!(
            "{prefix} color = {}({}[{}])",
            crate::server::postgres::constants::ANY_NAME,
            crate::server::postgres::constants::ARRAY_NAME,
            crate::server::postgres::generate_bind_increments::GenerateBindIncrements::generate_bind_increments(value, &mut increment)
        ));
        }
        {
            increment += 1;
            let limit_prefix = match additional_parameters.is_empty() {
                true => format!("{}", crate::server::postgres::constants::LIMIT_NAME),
                false => format!(" {}", crate::server::postgres::constants::LIMIT_NAME),
            };
            additional_parameters.push_str(&format!("{limit_prefix} ${increment}"));
        }
        additional_parameters
    }
}

impl crate::server::postgres::generate_get_query::GenerateGetQuery for GetQueryParameters {
    fn generate_get_query(&self) -> std::string::String {
        let additional_get_parameters_bind_places = crate::server::postgres::generate_where_get_parameters::GenerateWhereGetParametersBindPlaces::generate_where_get_parameters_bind_places(self);
        let select = crate::repositories_types::tufa_server::routes::api::cats::GetSelect::from(
            self.select.clone(),
        );
        let query_string = format!(
            "{} {select} {} {} {additional_get_parameters_bind_places}",
            crate::server::postgres::constants::SELECT_NAME,
            crate::server::postgres::constants::FROM_NAME,
            crate::repositories_types::tufa_server::routes::api::cats::CATS
        );
        println!("{query_string}");
        query_string
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
    pub async fn execute_query(
        &self,
        query_string: std::string::String,
        pool: &sqlx::PgPool,
        query_parameters: impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery,
        app_info_state: &axum::extract::State<crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants
    {
        let vec_values = {
            let mut rows =
                crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery::bind_sqlx_query(
                    query_parameters,
                    sqlx::query::<sqlx::Postgres>(&query_string),
                )
                .fetch(pool);
            let mut vec_values = Vec::new();
            while let Some(row) = {
                match {
                    use futures::TryStreamExt;
                    rows.try_next()
                }
                .await
                {
                    Ok(option_pg_row) => option_pg_row,
                    Err(e) => {
                        let error = crate::repositories_types::tufa_server::routes::api::cats::get::TryGet::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return crate::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::from(error);
                    }
                }
            } {
                match CatOptions::sqlx_from_row(&row, self) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        let error = crate::repositories_types::tufa_server::routes::api::cats::get::TryGet::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return crate::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::from(error);
                    }
                }
            }
            vec_values
        };
        crate::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::Desirable(vec_values)
    }
}
