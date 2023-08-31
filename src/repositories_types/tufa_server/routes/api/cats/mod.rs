pub mod delete;
pub mod delete_by_id;
pub mod get;
pub mod get_by_id;
pub mod patch_by_id;
pub mod post;
pub mod post_search;
pub mod put_by_id;
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

const SPLIT_INNER_URL_PARAMETERS_SYMBOL: char = ',';

fn deserialize_cat_order_by<'de, D>(deserializer: D) -> Result<CatOrderBy, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let string_deserialized = {
        use serde::Deserialize;
        String::deserialize(deserializer)?
    };
    let default_message = "Invalid CatOrderBy:";
    let column_equal_str = "column=";
    let order_equal_str = "order=";
    let column = match string_deserialized.find(column_equal_str) {
        Some(index) => match index.checked_add(column_equal_str.len()) {
            Some(offset) => match string_deserialized.get(offset..) {
                Some(offset_slice) => match offset_slice.find(SPLIT_INNER_URL_PARAMETERS_SYMBOL) {
                    Some(offset_slice_next_comma_index) => {
                        match offset_slice.get(0..offset_slice_next_comma_index) {
                            Some(possible_column) => match {
                                use std::str::FromStr;
                                CatColumn::from_str(possible_column)
                            } {
                                Ok(column) => column,
                                Err(e) => {
                                    return Err(serde::de::Error::custom(&format!(
                                        "{default_message} {column_equal_str} {e}"
                                    )));
                                }
                            },
                            None => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {column_equal_str} failed to offset_slice.get(0..offset_slice_next_comma_index)"
                                )));
                            }
                        }
                    }
                    None => match offset_slice.get(0..) {
                        Some(possible_column) => match {
                            use std::str::FromStr;
                            CatColumn::from_str(possible_column)
                        } {
                            Ok(column) => column,
                            Err(e) => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {column_equal_str} {e}"
                                )));
                            }
                        },
                        None => {
                            return Err(serde::de::Error::custom(&format!(
                                "{default_message} {column_equal_str} failed to offset_slice.get(0..)"
                            )));
                        }
                    },
                },
                None => {
                    return Err(serde::de::Error::custom(&format!(
                        "{default_message} {column_equal_str} failed to string_deserialized.get(offset..)"
                    )));
                }
            },
            None => {
                return Err(serde::de::Error::custom(&format!(
                    "{default_message} {column_equal_str} index overflow"
                )));
            }
        },
        None => {
            return Err(serde::de::Error::custom(&format!(
                "{default_message} {column_equal_str} not found"
            )));
        }
    };
    let order = match string_deserialized.find(order_equal_str) {
        Some(index) => match index.checked_add(order_equal_str.len()) {
            Some(offset) => match string_deserialized.get(offset..) {
                Some(offset_slice) => match offset_slice.find(SPLIT_INNER_URL_PARAMETERS_SYMBOL) {
                    Some(offset_slice_next_comma_index) => {
                        match offset_slice.get(0..offset_slice_next_comma_index) {
                            Some(possible_order) => match {
                                use std::str::FromStr;
                                crate::server::postgres::order::Order::from_str(possible_order)
                            } {
                                Ok(order) => Some(order),
                                Err(e) => {
                                    return Err(serde::de::Error::custom(&format!(
                                        "{default_message} {order_equal_str} {e}"
                                    )));
                                }
                            },
                            None => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {order_equal_str} failed to offset_slice.get(0..offset_slice_next_comma_index)"
                                )));
                            }
                        }
                    }
                    None => match offset_slice.get(0..) {
                        Some(possible_order) => match {
                            use std::str::FromStr;
                            crate::server::postgres::order::Order::from_str(possible_order)
                        } {
                            Ok(order) => Some(order),
                            Err(e) => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {order_equal_str} {e}"
                                )));
                            }
                        },
                        None => {
                            return Err(serde::de::Error::custom(&format!(
                                "{default_message} {order_equal_str} failed to offset_slice.get(0..)"
                            )));
                        }
                    },
                },
                None => {
                    return Err(serde::de::Error::custom(&format!(
                        "{default_message} {order_equal_str} failed to string_deserialized.get(offset..)"
                    )));
                }
            },
            None => {
                return Err(serde::de::Error::custom(&format!(
                    "{default_message} {order_equal_str} index overflow"
                )));
            }
        },
        None => None,
    };
    Ok(CatOrderBy { column, order })
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CatOrderBy {
    pub column: CatColumn,
    pub order: Option<crate::server::postgres::order::Order>,
}

impl crate::common::url_encode::UrlEncode for CatOrderBy {
    fn url_encode(&self) -> std::string::String {
        let order_stringified = match &self.order {
            Some(order) => order.to_string(),
            None => crate::server::postgres::order::Order::default().to_string(),
        };
        urlencoding::encode(&format!(
            "column={}{}order={}",
            crate::common::url_encode::UrlEncode::url_encode(&self.column),
            SPLIT_INNER_URL_PARAMETERS_SYMBOL,
            urlencoding::encode(&order_stringified)
        ))
        .to_string()
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct GetByIdPathParameters {
    pub id: crate::server::postgres::bigserial::Bigserial,
}

#[derive(Debug, serde::Deserialize)]
pub struct GetByIdQueryParameters {
    pub select: Option<CatSelectUrl>,
}

#[derive(Debug, serde::Deserialize)]
pub struct GetQueryParameters {
    pub select: Option<CatSelectUrl>,
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
                None => CatSelectUrl::default().to_string(),
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

#[derive(Debug, serde::Deserialize)]
pub struct PutByIdPathParameters {
    pub id: crate::server::postgres::bigserial::Bigserial,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatToPutById {
    pub name: String,
    pub color: String,
}
//
// todo
// The PUT method updates a resource on a server. A PUT request to the /users/20 can be used to update the profile of the user with an ID 20.
//
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

// #[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
// pub struct CatToPatchById {
//     pub name: Option<std::string::String>,
//     pub color: Option<std::string::String>,
// }

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
            let select = CatSelectUrl::from(self.select.clone());
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

/////////////////////

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatToPostSearch {
    pub select: CatColumnSelectJson,
    pub ids: Option<Vec<crate::server::postgres::bigserial::Bigserial>>,
    pub name_regex: Option<Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub color_regex: Option<Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub order_by: CatOrderBy,
    pub limit: crate::server::postgres::postgres_number::PostgresNumber,
    pub offset: crate::server::postgres::postgres_number::PostgresNumber,
}

impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery for CatToPostSearch {
    fn bind_sqlx_query(
        self,
        mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        use crate::server::postgres::bind_query::BindQuery;
        if let Some(values) = self.ids {
            for value in values {
                query = value.bind_value_to_query(query);
            }
        }
        if let Some(values) = self.name_regex {
            for value in values {
                query = value.bind_value_to_query(query);
            }
        }
        if let Some(values) = self.color_regex {
            for value in values {
                query = value.bind_value_to_query(query);
            }
        }
        query = self.limit.bind_value_to_query(query);
        query = self.offset.bind_value_to_query(query);
        query
    }
}

impl CatToPostSearch {
    pub async fn execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::post_search::TryPostSearchResponseVariants
    {
        let vec_values = {
            let select = self.select.clone();
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
                        let error = crate::repositories_types::tufa_server::routes::api::cats::post_search::TryPostSearch::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return crate::repositories_types::tufa_server::routes::api::cats::post_search::TryPostSearchResponseVariants::from(error);
                    }
                }
            } {
                match select.options_try_from_sqlx_row(&row) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        let error = crate::repositories_types::tufa_server::routes::api::cats::post_search::TryPostSearch::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return crate::repositories_types::tufa_server::routes::api::cats::post_search::TryPostSearchResponseVariants::from(error);
                    }
                }
            }
            vec_values
        };
        crate::repositories_types::tufa_server::routes::api::cats::post_search::TryPostSearchResponseVariants::Desirable(vec_values)
    }
}

impl crate::server::postgres::generate_get_query::GenerateGetQuery for CatToPostSearch {
    fn generate_get_query(&self) -> std::string::String {
        let mut query = std::string::String::from("");
        {
            query.push_str(&format!(
                "{} {}",
                crate::server::postgres::constants::SELECT_NAME,
                crate::server::postgres::generate_get_query::GenerateGetQuery::generate_get_query(
                    &self.select
                )
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
            if let Some(value) = &self.ids {
                let prefix = match additional_parameters.is_empty() {
                    true => crate::server::postgres::constants::WHERE_NAME.to_string(),
                    false => format!(" {}", crate::server::postgres::constants::AND_NAME),
                };
                let bind_increments = {
                    let mut bind_increments = value.iter().fold(std::string::String::from(""), |mut acc, element| {
                        let bind_increments = crate::server::postgres::bind_query::BindQuery::generate_bind_increments(
                            element,
                            &mut increment
                        );
                       acc.push_str(&format!("{bind_increments}, "));
                        acc
                    });
                    if let false = bind_increments.is_empty() {
                        bind_increments.pop();
                        bind_increments.pop();
                    }
                    bind_increments
                };
                additional_parameters.push_str(&format!(
                    "{prefix} id = {}({}[{}])",
                    crate::server::postgres::constants::ANY_NAME,
                    crate::server::postgres::constants::ARRAY_NAME,
                    bind_increments
                ));
            }
            if let Some(value) = &self.name_regex {
                let prefix = match additional_parameters.is_empty() {
                    true => crate::server::postgres::constants::WHERE_NAME.to_string(),
                    false => format!(" {}", crate::server::postgres::constants::AND_NAME),
                };
                let column_name = "name";
                let bind_increments = {
                    let mut bind_increments = value.iter().enumerate().fold(std::string::String::from(""), |mut acc, (index, element)| {
                        let conjuctive_operator = &element.conjuctive_operator;
                        let bind_increments = crate::server::postgres::bind_query::BindQuery::generate_bind_increments(
                            element,
                            &mut increment
                        );
                         match index == 0 {
                            true => {
                                acc.push_str(&format!("{column_name} ~ {bind_increments} "));
                            },
                            false => {
                                acc.push_str(&format!("{conjuctive_operator} {column_name} ~ {bind_increments} "));
                            },
                        }
                        acc
                    });
                    if let false = bind_increments.is_empty() {
                        bind_increments.pop();
                    }
                    bind_increments
                };
                additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
            }
            if let Some(value) = &self.color_regex {
                let prefix = match additional_parameters.is_empty() {
                    true => crate::server::postgres::constants::WHERE_NAME.to_string(),
                    false => format!(" {}", crate::server::postgres::constants::AND_NAME),
                };
                let column_name = "color";
                let bind_increments = {
                    let mut bind_increments = value.iter().enumerate().fold(std::string::String::from(""), |mut acc, (index, element)| {
                        let conjuctive_operator = &element.conjuctive_operator;
                        let bind_increments = crate::server::postgres::bind_query::BindQuery::generate_bind_increments(
                            element,
                            &mut increment
                        );
                        match index == 0 {
                            true => {
                                acc.push_str(&format!("{column_name} ~ {bind_increments} "));
                            },
                            false => {
                                acc.push_str(&format!("{conjuctive_operator} {column_name} ~ {bind_increments} "));
                            },
                        }
                        acc
                    });
                    if let false = bind_increments.is_empty() {
                        bind_increments.pop();
                    }
                    bind_increments
                };
                additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
            }
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "",
                    false => " ",
                };
                let value = &self.order_by;
                let order_stringified = match &value.order {
                    Some(order) => order.to_string(),
                    None => crate::server::postgres::order::Order::default().to_string(),
                };
                additional_parameters.push_str(&format!(
                    "{prefix}{} {} {order_stringified}",
                    crate::server::postgres::constants::ORDER_BY_NAME,
                    value.column
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
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "",
                    false => " ",
                };
                additional_parameters.push_str(&format!(
                    "{prefix}{} {}",
                    crate::server::postgres::constants::OFFSET_NAME,
                    crate::server::postgres::bind_query::BindQuery::generate_bind_increments(
                        &self.offset,
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

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    enum_extension::EnumExtension,
    strum_macros::EnumIter,
    PartialEq,
    Eq,
    strum_macros::Display,
)]
pub enum CatColumnSelectJson {
    Id,
    Name,
    Color,
    IdName,
    IdColor,
    NameColor,
    IdNameColor,
}

impl crate::server::postgres::generate_get_query::GenerateGetQuery for CatColumnSelectJson {
    fn generate_get_query(&self) -> std::string::String {
        match self {
            Self::Id => std::string::String::from("id"),
            Self::Name => std::string::String::from("name"),
            Self::Color => std::string::String::from("color"),
            Self::IdName => std::string::String::from("id,name"),
            Self::IdColor => std::string::String::from("id,color"),
            Self::NameColor => std::string::String::from("name,color"),
            Self::IdNameColor => std::string::String::from("id,name,color"),
        }
    }
}

impl std::default::Default for CatColumnSelectJson {
    fn default() -> Self {
        Self::IdNameColor
    }
}
impl std::convert::From<Option<Self>> for CatColumnSelectJson {
    fn from(option_value: Option<Self>) -> Self {
        match option_value {
            Some(value) => value,
            None => Self::default(),
        }
    }
}

impl CatColumnSelectJson {
    fn options_try_from_sqlx_row<'a, R: ::sqlx::Row>(
        &self,
        row: &'a R,
    ) -> ::sqlx::Result<CatOptions>
    where
        &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
        Option<i64>: ::sqlx::decode::Decode<'a, R::Database>,
        Option<i64>: ::sqlx::types::Type<R::Database>,
        Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
        Option<String>: ::sqlx::types::Type<R::Database>,
        Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
        Option<String>: ::sqlx::types::Type<R::Database>,
    {
        let mut id: Option<i64> = None;
        let mut name: Option<String> = None;
        let mut color: Option<String> = None;
        match self {
            Self::Id => {
                id = row.try_get("id")?;
            }
            Self::Name => {
                name = row.try_get("name")?;
            }
            Self::Color => {
                color = row.try_get("color")?;
            }
            Self::IdName => {
                id = row.try_get("id")?;
                name = row.try_get("name")?;
            }
            Self::IdColor => {
                id = row.try_get("id")?;
                color = row.try_get("color")?;
            }
            Self::NameColor => {
                name = row.try_get("name")?;
                color = row.try_get("color")?;
            }
            Self::IdNameColor => {
                id = row.try_get("id")?;
                name = row.try_get("name")?;
                color = row.try_get("color")?;
            }
        }
        Ok(CatOptions { id, name, color })
    }
}
