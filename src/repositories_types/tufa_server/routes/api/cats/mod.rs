pub mod create;
pub mod create_or_update_by_id;
pub mod delete;
pub mod delete_by_id;
pub mod read;
pub mod read_by_id;
pub mod read_post;
pub mod update_by_id;
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
    #[generate_postgresql_crud_id]
    pub id: i64, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    pub name: String,
    pub color: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CatOrderByWrapper(
    #[serde(deserialize_with = "deserialize_cat_order_by")] pub CatOrderBy,
);

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for CatOrderByWrapper {
    fn serde_urlencoded_parameter(&self) -> std::string::String {
        let column = &self.0.column;
        let order = self.0.order.clone().unwrap_or_default();
        format!("column={column},order={order}")
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

#[derive(Debug, serde::Deserialize)]
pub struct ReadByIdParameters {
    pub path: ReadByIdPath,
    pub query: ReadByIdQuery,
}

#[derive(Debug, serde::Deserialize)]
pub struct ReadByIdPath {
    pub id: crate::server::postgres::bigserial::Bigserial,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ReadByIdQuery {
    pub select: Option<CatColumnSelect>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ReadParameters {
    pub query: ReadQuery,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ReadQuery {
    pub select: Option<CatColumnSelect>,
    pub id: Option<crate::server::postgres::bigserial_ids::BigserialIds>,
    pub name: Option<crate::server::routes::helpers::strings_deserialized_from_string_splitted_by_comma::StringsDeserializedFromStringSplittedByComma>,
    pub color: Option<crate::server::routes::helpers::strings_deserialized_from_string_splitted_by_comma::StringsDeserializedFromStringSplittedByComma>,
    pub order_by: Option<CatOrderByWrapper>,
    pub limit: crate::server::postgres::postgres_number::PostgresNumber,
    pub offset: Option<crate::server::postgres::postgres_number::PostgresNumber>,
}

impl ReadQuery {
    pub fn into_url_encoding_version(self) -> ReadQueryForUrlEncoding {
        let select = self.select.as_ref().map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        let id = self.id.as_ref().map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        let name = self.name.as_ref().map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        let color = self.color.as_ref().map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        let order_by = self.order_by.as_ref().map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        let limit =
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                &self.limit,
            );
        let offset = self.offset.as_ref().map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        ReadQueryForUrlEncoding {
            select,
            id,
            name,
            color,
            order_by,
            limit,
            offset,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ReadQueryForUrlEncoding {
    pub select: Option<std::string::String>,
    pub id: Option<std::string::String>,
    pub name: Option<std::string::String>,
    pub color: Option<std::string::String>,
    pub order_by: Option<std::string::String>,
    pub limit: std::string::String,
    pub offset: Option<std::string::String>,
}

//todo - make a macro for it?
//todo - maybe some serde serialization like this https://docs.rs/url_serde/latest/url_serde/

impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery for ReadQuery {
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

impl crate::server::postgres::generate_get_query::GenerateGetQuery for ReadQuery {
    fn generate_get_query(&self) -> std::string::String {
        // SELECT id,name,color FROM cats WHERE id = ANY(ARRAY[$1, $2, $3, $4]) AND name = ANY(ARRAY[$5, $6]) AND color = ANY(ARRAY[$7]) LIMIT $8
        let mut query = std::string::String::from("");
        {
            query.push_str(&format!(
                "{} {}",
                crate::server::postgres::constants::SELECT_NAME,
                crate::server::postgres::generate_get_query::GenerateGetQuery::generate_get_query(
                    &CatColumnSelect::from(self.select.clone())
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
pub struct CreateParameters {
    pub payload: CreatePayload,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CreatePayload {
    pub name: String,
    pub color: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateOrUpdateByIdParameters {
    pub path: CreateOrUpdateByIdPath,
    pub payload: CreateOrUpdateByIdPayload,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateOrUpdateByIdPath {
    pub id: crate::server::postgres::bigserial::Bigserial,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CreateOrUpdateByIdPayload {
    pub name: String,
    pub color: String,
}
//
// todo
// The PUT method updates a resource on a server. A PUT request to the /users/20 can be used to update the profile of the user with an ID 20.
//
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PutPayload {
    pub id: crate::server::postgres::bigserial::Bigserial, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    pub name: String,
    pub color: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateByIdParameters {
    pub path: UpdateByIdPath,
    pub payload: UpdateByIdPayload,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateByIdPath {
    pub id: crate::server::postgres::bigserial::Bigserial,
}

// #[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
// pub struct UpdateByIdPayload {
//     pub name: Option<std::string::String>,
//     pub color: Option<std::string::String>,
// }

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum UpdateByIdPayload {
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

#[derive(Debug, serde::Deserialize)]
pub struct DeleteByIdParameters {
    pub path: DeleteByIdPath,
}

#[derive(Debug, serde::Deserialize)]
pub struct DeleteByIdPath {
    pub id: crate::server::postgres::bigserial::Bigserial,
}

#[derive(Debug, serde::Deserialize)]
pub struct DeleteParameters {
    pub query: DeleteQuery,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteQuery {
    pub name: Option<String>,
    pub color: Option<String>,
}

impl ReadQuery {
    pub async fn execute_query(
        self, //impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuer + crate::server::postgres::generate_get_query::GenerateGetQuery
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::read::TryReadResponseVariants
    {
        let vec_values = {
            let select = CatColumnSelect::from(self.select.clone());
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
                        let error = crate::repositories_types::tufa_server::routes::api::cats::read::TryRead::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return crate::repositories_types::tufa_server::routes::api::cats::read::TryReadResponseVariants::from(error);
                    }
                }
            } {
                match select.options_try_from_sqlx_row(&row) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        let error = crate::repositories_types::tufa_server::routes::api::cats::read::TryRead::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return crate::repositories_types::tufa_server::routes::api::cats::read::TryReadResponseVariants::from(error);
                    }
                }
            }
            vec_values
        };
        crate::repositories_types::tufa_server::routes::api::cats::read::TryReadResponseVariants::Desirable(vec_values)
    }
}

/////////////////////

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ReadPostParameters {
    pub payload: ReadPostPayload,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ReadPostPayload {
    pub select: CatColumnSelect,
    pub ids: Option<Vec<crate::server::postgres::bigserial::Bigserial>>,
    pub name_regex: Option<Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub color_regex: Option<Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub order_by: CatOrderBy,
    pub limit: crate::server::postgres::postgres_number::PostgresNumber,
    pub offset: crate::server::postgres::postgres_number::PostgresNumber,
}

impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery for ReadPostPayload {
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

impl ReadPostPayload {
    pub async fn execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::read_post::TryReadPostResponseVariants
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
                        let error = crate::repositories_types::tufa_server::routes::api::cats::read_post::TryReadPost::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return crate::repositories_types::tufa_server::routes::api::cats::read_post::TryReadPostResponseVariants::from(error);
                    }
                }
            } {
                match select.options_try_from_sqlx_row(&row) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        let error = crate::repositories_types::tufa_server::routes::api::cats::read_post::TryReadPost::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return crate::repositories_types::tufa_server::routes::api::cats::read_post::TryReadPostResponseVariants::from(error);
                    }
                }
            }
            vec_values
        };
        crate::repositories_types::tufa_server::routes::api::cats::read_post::TryReadPostResponseVariants::Desirable(vec_values)
    }
}

// pub async fn request_options(
//     query_string: &std::string::String,
//     rows: std::pin::Pin<
//         Box<dyn futures::Stream<Item = Result<sqlx::postgres::PgRow, sqlx::Error>> + Send>,
//     >,
//     app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
// ) {
//     // let mut rows = crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery::bind_sqlx_query(
//     //     self,
//     //     sqlx::query::<sqlx::Postgres>(query_string),
//     // )
//     // .fetch(app_info_state.get_postgres_pool());
//     let mut vec_values = Vec::new();
//     while let Some(row) = {
//         match {
//             use futures::TryStreamExt;
//             rows.try_next()
//         }
//         .await
//         {
//             Ok(option_pg_row) => option_pg_row,
//             Err(e) => {
//                 let error = crate::repositories_types::tufa_server::routes::api::cats::read_post::TryReadPost::from(e);
//                 crate::common::error_logs_logic::error_log::ErrorLog::error_log(
//                     &error,
//                     app_info_state.as_ref(),
//                 );
//                 return crate::repositories_types::tufa_server::routes::api::cats::read_post::TryReadPostResponseVariants::from(error);
//             }
//         }
//     } {
//         match select.options_try_from_sqlx_row(&row) {
//             Ok(value) => {
//                 vec_values.push(value);
//             }
//             Err(e) => {
//                 let error = crate::repositories_types::tufa_server::routes::api::cats::read_post::TryReadPost::from(e);
//                 crate::common::error_logs_logic::error_log::ErrorLog::error_log(
//                     &error,
//                     app_info_state.as_ref(),
//                 );
//                 return crate::repositories_types::tufa_server::routes::api::cats::read_post::TryReadPostResponseVariants::from(error);
//             }
//         }
//     }
//     vec_values
// }

impl crate::server::postgres::generate_get_query::GenerateGetQuery for ReadPostPayload {
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
