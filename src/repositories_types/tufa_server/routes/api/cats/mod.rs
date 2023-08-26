pub mod delete;
pub mod delete_by_id;
pub mod get;
pub mod get_by_id;
pub mod patch_by_id;
pub mod post;
pub mod put;
//todo openapi
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
    generate_postgresql_crud::GeneratePostgresqlCrud,
)]
pub struct Cat {
    pub id: i64, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    pub name: String,
    pub color: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CatOrderByWrapper(
    #[serde(deserialize_with = "deserialize_cat_order_by")] pub CatOrderBy,
);

impl crate::common::url_encode::UrlEncode for CatOrderByWrapper {
    fn url_encode(&self) -> std::string::String {
        crate::common::url_encode::UrlEncode::url_encode(&self.0)
    }
}

fn deserialize_cat_order_by<'de, D>(deserializer: D) -> Result<CatOrderBy, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let string_deserialized = {
        use serde::Deserialize;
        String::deserialize(deserializer)?
    };
    let splitted_string: Vec<&str> = string_deserialized.split('.').collect();
    let default_message = "Invalid CatOrderBy:";
    let mut splitted_string_into_iter = splitted_string.into_iter();
    let possible_column = match splitted_string_into_iter.next() {
        Some(possible_column) => possible_column,
        None => {
            return Err(serde::de::Error::custom(&format!(
                "{default_message} no first element after split"
            )));
        }
    };
    match splitted_string_into_iter.next() {
        Some(possible_order) => {
            let column = match {
                use std::str::FromStr;
                CatOrderByField::from_str(possible_column)
            } {
                Ok(column) => column,
                Err(e) => {
                    return Err(serde::de::Error::custom(&format!("{default_message} {e}")));
                }
            };
            let order = match {
                use std::str::FromStr;
                crate::server::postgres::order::Order::from_str(possible_order)
            } {
                Ok(order) => order,
                Err(e) => {
                    return Err(serde::de::Error::custom(&format!("{default_message} {e}")));
                }
            };
            Ok(CatOrderBy {
                column,
                order: Some(order),
            })
        }
        None => {
            let column = match {
                use std::str::FromStr;
                CatOrderByField::from_str(possible_column)
            } {
                Ok(column) => column,
                Err(e) => {
                    return Err(serde::de::Error::custom(&format!("{default_message} {e}")));
                }
            };
            Ok(CatOrderBy {
                column,
                order: None,
            })
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CatOrderBy {
    pub column: CatOrderByField,
    pub order: Option<crate::server::postgres::order::Order>,
}

impl crate::common::url_encode::UrlEncode for CatOrderBy {
    fn url_encode(&self) -> std::string::String {
        let order_stringified = match &self.order {
            Some(order) => order.to_string(),
            None => crate::server::postgres::order::Order::default().to_string(),
        };
        urlencoding::encode(&format!(
            "{}.{}",
            crate::common::url_encode::UrlEncode::url_encode(&self.column),
            urlencoding::encode(&order_stringified)
        ))
        .to_string()
    }
}

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    enum_extension::EnumExtension,
    strum_macros::EnumIter,
    PartialEq,
    Eq,
)]
pub enum CatOrderByField {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id,
    #[serde(rename(serialize = "name", deserialize = "name"))]
    Name,
    #[serde(rename(serialize = "color", deserialize = "color"))]
    Color,
}

impl std::str::FromStr for CatOrderByField {
    type Err = std::string::String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "id" => Ok(Self::Id),
            "name" => Ok(Self::Name),
            "color" => Ok(Self::Color),
            _ => Err(format!("Invalid CatOrderByField, expected one of \'id\', \'name\', \'color\', found {value}")),
        }
    }
}

impl std::fmt::Display for CatOrderByField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatOrderByField::Id => write!(f, "id"),
            CatOrderByField::Name => write!(f, "name"),
            CatOrderByField::Color => write!(f, "color"),
        }
    }
}

impl crate::common::url_encode::UrlEncode for CatOrderByField {
    fn url_encode(&self) -> std::string::String {
        urlencoding::encode(&self.to_string()).to_string()
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct GetByIdPathParameters {
    pub id: crate::server::postgres::bigserial::Bigserial,
}

#[derive(Debug, serde::Deserialize)]
pub struct GetByIdQueryParameters {
    pub select: Option<CatSelect>,
}

#[derive(Debug, serde::Deserialize)]
pub struct GetQueryParameters {
    pub select: Option<CatSelect>,
    pub id: Option<crate::server::postgres::bigserial_ids::BigserialIds>,
    pub name: Option<crate::server::routes::helpers::strings_deserialized_from_string_splitted_by_comma::StringsDeserializedFromStringSplittedByComma>,
    pub color: Option<crate::server::routes::helpers::strings_deserialized_from_string_splitted_by_comma::StringsDeserializedFromStringSplittedByComma>,
    pub order_by: Option<CatOrderByWrapper>,
    pub limit: crate::server::postgres::postgres_number::PostgresNumber,
    pub offset: Option<crate::server::postgres::postgres_number::PostgresNumber>,
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
        if let Some(value) = &self.order_by {
            stringified_query_parameters.push_str(&format!(
                "&order_by={}",
                crate::common::url_encode::UrlEncode::url_encode(value)
            ));
        }
        stringified_query_parameters.push_str(&format!(
            "&limit={}",
            urlencoding::encode(&self.limit.to_string())
        ));
        if let Some(value) = &self.offset {
            let query_parameter_handle =
                format!("offset={}", urlencoding::encode(&value.to_string()));
            stringified_query_parameters.push_str(&format!("&{query_parameter_handle}"));
        }
        stringified_query_parameters
    }
}

impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery for GetQueryParameters {
    fn bind_sqlx_query(
        self,
        mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        use crate::server::postgres::bind_query::BindQuery;
        if let Some(value) = self.id {
            query = value.bind_value_to_query(query);
        }
        if let Some(value) = self.name {
            query = value.bind_value_to_query(query);
        }
        if let Some(value) = self.color {
            query = value.bind_value_to_query(query);
        }
        query = self.limit.bind_value_to_query(query);
        if let Some(value) = self.offset {
            query = value.bind_value_to_query(query);
        }
        query
    }
}

impl crate::server::postgres::generate_get_query::GenerateGetQuery for GetQueryParameters {
    fn generate_get_query(&self) -> std::string::String {
        // SELECT id,name,color FROM cats WHERE id = ANY(ARRAY[$1, $2, $3, $4]) AND name = ANY(ARRAY[$5, $6]) AND color = ANY(ARRAY[$7]) LIMIT $8
        // SELECT id,name,color FROM public.cats WHERE name LIKE 'test%' OR name LIKE '%patch%' ;
        let mut query = std::string::String::from("");
        {
            let select_stringified = match &self.select {
                Some(select) => select.to_string(),
                None => CatSelect::default().to_string(),
            };
            query.push_str(&format!(
                "{} {select_stringified}",
                crate::server::postgres::constants::SELECT_NAME,
            ));
        }
        query.push_str(&format!(
            " {} {}",
            crate::server::postgres::constants::FROM_NAME,
            crate::repositories_types::tufa_server::routes::api::cats::CATS
        ));
        let additional_parameters = {
            let mut additional_parameters = std::string::String::from("");
            let mut increment: u64 = 0;
            if let Some(value) = &self.id {
                let prefix = match additional_parameters.is_empty() {
                    true => crate::server::postgres::constants::WHERE_NAME.to_string(),
                    false => format!(" {}", crate::server::postgres::constants::AND_NAME),
                };
                additional_parameters.push_str(&format!(
                    "{prefix} id = {}({}[{}])",
                    crate::server::postgres::constants::ANY_NAME,
                    crate::server::postgres::constants::ARRAY_NAME,
                    crate::server::postgres::bind_query::BindQuery::generate_bind_increments(
                        value,
                        &mut increment
                    )
                ));
            }
            if let Some(value) = &self.name {
                let prefix = match additional_parameters.is_empty() {
                    true => crate::server::postgres::constants::WHERE_NAME.to_string(),
                    false => format!(" {}", crate::server::postgres::constants::AND_NAME),
                };
                additional_parameters.push_str(&format!(
                    "{prefix} name = {}({}[{}])",
                    crate::server::postgres::constants::ANY_NAME,
                    crate::server::postgres::constants::ARRAY_NAME,
                    crate::server::postgres::bind_query::BindQuery::generate_bind_increments(
                        value,
                        &mut increment
                    )
                ));
            }
            if let Some(value) = &self.color {
                let prefix = match additional_parameters.is_empty() {
                    true => crate::server::postgres::constants::WHERE_NAME.to_string(),
                    false => format!(" {}", crate::server::postgres::constants::AND_NAME),
                };
                additional_parameters.push_str(&format!(
                    "{prefix} color = {}({}[{}])",
                    crate::server::postgres::constants::ANY_NAME,
                    crate::server::postgres::constants::ARRAY_NAME,
                    crate::server::postgres::bind_query::BindQuery::generate_bind_increments(
                        value,
                        &mut increment
                    )
                ));
            }
            if let Some(value) = &self.order_by {
                let prefix = match additional_parameters.is_empty() {
                    true => "",
                    false => " ",
                };
                let order_stringified = match &value.0.order {
                    Some(order) => order.to_string(),
                    None => crate::server::postgres::order::Order::default().to_string(),
                };
                additional_parameters.push_str(&format!(
                    "{prefix}{} {} {order_stringified}",
                    crate::server::postgres::constants::ORDER_BY_NAME,
                    value.0.column
                ));
            }
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "",
                    false => " ",
                };
                additional_parameters.push_str(&format!(
                    "{prefix}{} {}",
                    crate::server::postgres::constants::LIMIT_NAME,
                    crate::server::postgres::bind_query::BindQuery::generate_bind_increments(
                        &self.limit,
                        &mut increment
                    )
                ));
            }
            if let Some(value) = &self.offset {
                let prefix = match additional_parameters.is_empty() {
                    true => "",
                    false => " ",
                };
                additional_parameters.push_str(&format!(
                    "{prefix}{} {}",
                    crate::server::postgres::constants::OFFSET_NAME,
                    crate::server::postgres::bind_query::BindQuery::generate_bind_increments(
                        value,
                        &mut increment
                    )
                ));
            }
            additional_parameters
        };
        query.push_str(&format!(" {additional_parameters}"));
        println!("{query}");
        query
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
pub struct PatchByIdPathParameters {
    pub id: crate::server::postgres::bigserial::Bigserial,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum CatToPatchById {
    Name {
        name: std::string::String,
    },
    Color {
        color: std::string::String,
    },
    NameColor {
        name: std::string::String,
        color: std::string::String,
    },
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

impl GetQueryParameters {
    pub async fn execute_query(
        self, //impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuer + crate::server::postgres::generate_get_query::GenerateGetQuery
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants
    {
        let vec_values = {
            let select = CatSelect::from(self.select.clone());
            let query_string =
                crate::server::postgres::generate_get_query::GenerateGetQuery::generate_get_query(
                    &self,
                );
            let mut rows =
                crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery::bind_sqlx_query(
                    self,
                    sqlx::query::<sqlx::Postgres>(&query_string),
                )
                .fetch(app_info_state.get_postgres_pool());
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
                match select.options_try_from_sqlx_row(&row) {
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
