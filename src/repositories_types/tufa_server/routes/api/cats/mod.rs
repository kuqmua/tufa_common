pub mod create;
pub mod create_batch;
pub mod create_or_update;
pub mod create_or_update_by_id;
pub mod delete;
pub mod delete_with_body;
pub mod delete_by_id;
pub mod read;
pub mod read_by_id;
pub mod read_with_body;
pub mod update;
pub mod update_by_id;
//todo openapi
//todo test if create\update\delete empty array 
pub const CATS: &str = "cats";

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
    #[serde(deserialize_with = "deserialize_cat_order_by")]
    pub  crate::server::postgres::order_by::OrderBy<CatColumn>,
);

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for CatOrderByWrapper {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        let column = &self.0.column;
        let order = self.0.order.unwrap_or_default();
        format!("column={column},order={order}")
    }
}

fn deserialize_cat_order_by<'de, D>(
    deserializer: D,
) -> Result<crate::server::postgres::order_by::OrderBy<CatColumn>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let string_deserialized = {
        use serde::Deserialize;
        String::deserialize(deserializer)?
    };
    let split_inner_url_parameters_symbol = ',';
    let default_message = "Invalid CatOrderBy:";
    let column_equal_str = "column=";
    let order_equal_str = "order=";
    let column = match string_deserialized.find(column_equal_str) {
        Some(index) => match index.checked_add(column_equal_str.len()) {
            Some(offset) => match string_deserialized.get(offset..) {
                Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
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
                Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
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
    Ok(crate::server::postgres::order_by::OrderBy { column, order })
}

///////////////////////

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CreateBatchParameters {
    pub payload: Vec<CreateBatchPayloadElement>,
}
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CreateBatchPayloadElement {
    pub name: String,
    pub color: String,
}
impl crate::server::postgres::bind_query::BindQuery for CreateBatchPayloadElement {
    fn try_generate_bind_increments(
        &self,
        increment: &mut u64,
    ) -> Result<
        std::string::String,
        crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
    > {
        let mut increments = std::string::String::from("");
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                increments.push_str(&format!("${increment}, "));
            },
            None => {
                return Err(crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed::CheckedAdd { 
                    checked_add: std::string::String::from("checked_add is None"), 
                    code_occurence: crate::code_occurence_tufa_common!(), 
                });
            },
        }
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                increments.push_str(&format!("${increment}, "));
            },
            None => {
                return Err(crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed::CheckedAdd { 
                    checked_add: std::string::String::from("checked_add is None"), 
                    code_occurence: crate::code_occurence_tufa_common!(), 
                });
            },
        }
        increments.pop();
        increments.pop();
        Ok(increments)
    }
    fn bind_value_to_query(
        self,
        mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.name);
        query = query.bind(self.color);
        query
    }
}
impl CreateBatchParameters {
    pub async fn prepare_and_execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::create_batch::TryCreateBatchResponseVariants
    {
        let bind_increments = {
            let mut increment: u64 = 0;
            let mut bind_increments = std::string::String::from("");
            for element in &self.payload {
                let mut element_bind_increments = std::string::String::from("");
                match crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments(
                    element,
                    &mut increment
                ) {
                    Ok(bind_increments_handle) => {
                        element_bind_increments.push_str(&format!("{bind_increments_handle}, "));
                    },
                    Err(e) => {
                        return crate::repositories_types::tufa_server::routes::api::cats::create_batch::TryCreateBatchResponseVariants::BindQuery { 
                            checked_add: e.into_serialize_deserialize_version(), 
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                    },
                }
                element_bind_increments.pop();
                element_bind_increments.pop();
                bind_increments.push_str(&format!("({element_bind_increments}), "));
            }
            bind_increments.pop();
            bind_increments.pop();
            bind_increments
        };
        let query_string = format!(
            "{} {} {}(name, color) {} {bind_increments}",
            crate::server::postgres::constants::INSERT_NAME,
            crate::server::postgres::constants::INTO_NAME,
            crate::repositories_types::tufa_server::routes::api::cats::CATS,
            crate::server::postgres::constants::VALUES_NAME
        );
        println!("{query_string}");
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            for element in self.payload {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(element, query);
            }
            query
        };
        match binded_query
            .execute(app_info_state.get_postgres_pool())
            .await
        {
            Ok(_) => {
                //todo - is need to return rows affected?
                crate::repositories_types::tufa_server::routes::api::cats::create_batch::TryCreateBatchResponseVariants::Desirable(())
            }
            Err(e) => {
                let error =
                    crate::repositories_types::tufa_server::routes::api::cats::create_batch::TryCreateBatch::from(
                        e,
                    );
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                crate::repositories_types::tufa_server::routes::api::cats::create_batch::TryCreateBatchResponseVariants::from(error)
            }
        }
    }
}

impl CreateOrUpdateByIdParameters {
    pub async fn prepare_and_execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::create_or_update_by_id::TryCreateOrUpdateByIdResponseVariants
    {
        let query_string = format!(
            "{} {} {} (id, name, color) {} ($1, $2, $3) {} {} (id) {} {} {} name = EXCLUDED.name, color = EXCLUDED.color",
            crate::server::postgres::constants::INSERT_NAME,
            crate::server::postgres::constants::INTO_NAME,
            crate::repositories_types::tufa_server::routes::api::cats::CATS,
            crate::server::postgres::constants::VALUES_NAME,
            crate::server::postgres::constants::ON_NAME,
            crate::server::postgres::constants::CONFLICT_NAME,
            crate::server::postgres::constants::DO_NAME,
            crate::server::postgres::constants::UPDATE_NAME,
            crate::server::postgres::constants::SET_NAME,
        );
        println!("{query_string}");
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            query = query
                .bind(self.path.id.into_inner())
                .bind(self.payload.name)
                .bind(self.payload.color);
            query
        };
        match binded_query
            .execute(app_info_state.get_postgres_pool())
            .await
        {
            Ok(_) => {
                //todo - is need to return rows affected?
                crate::repositories_types::tufa_server::routes::api::cats::create_or_update_by_id::TryCreateOrUpdateByIdResponseVariants::Desirable(())
            }
            Err(e) => {
                let error =
                    crate::repositories_types::tufa_server::routes::api::cats::create_or_update_by_id::TryCreateOrUpdateById::from(
                        e,
                    );
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                crate::repositories_types::tufa_server::routes::api::cats::create_or_update_by_id::TryCreateOrUpdateByIdResponseVariants::from(error)
            }
        }
    }
}

#[derive(Debug, serde :: Deserialize)]
pub struct CreateOrUpdateParameters {
    pub payload: Vec<CreateOrUpdatePayloadElement>,
}
#[derive(Debug, serde_derive :: Serialize, serde_derive :: Deserialize)]
pub struct CreateOrUpdatePayloadElement {
    pub id: crate::server::postgres::bigserial::Bigserial,
    pub name: String,
    pub color: String,
}
impl CreateOrUpdateParameters {
    pub async fn prepare_and_execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::create_or_update::TryCreateOrUpdateResponseVariants
    {
        // let query_string = format!(
        //     // insert into cats (id, name, color) values ($1, $2, $3) on conflict (id) do update set name = EXCLUDED.name, color = EXCLUDED.color
        //     "{} {} {} (id, name, color) {} ($1, $2, $3) {} {} (id) {} {} {} name = EXCLUDED.name, color = EXCLUDED.color",
        //     crate::server::postgres::constants::INSERT_NAME,
        //     crate::server::postgres::constants::INTO_NAME,
        //     crate::repositories_types::tufa_server::routes::api::cats::CATS,
        //     crate::server::postgres::constants::VALUES_NAME,
        //     crate::server::postgres::constants::ON_NAME,
        //     crate::server::postgres::constants::CONFLICT_NAME,
        //     crate::server::postgres::constants::DO_NAME,
        //     crate::server::postgres::constants::UPDATE_NAME,
        //     crate::server::postgres::constants::SET_NAME,
        // );
        // println!("{query_string}");
        // let binded_query = {
        //     let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        //     query = query
        //         .bind(self.path.id.into_inner())
        //         .bind(self.payload.name)
        //         .bind(self.payload.color);
        //     query
        // };
        // match binded_query
        //     .execute(app_info_state.get_postgres_pool())
        //     .await
        // {
        //     Ok(_) => {
        //         //todo - is need to return rows affected?
        //         crate::repositories_types::tufa_server::routes::api::cats::create_or_update_by_id::TryCreateOrUpdateByIdResponseVariants::Desirable(())
        //     }
        //     Err(e) => {
        //         let error =
        //             crate::repositories_types::tufa_server::routes::api::cats::create_or_update_by_id::TryCreateOrUpdateById::from(
        //                 e,
        //             );
        //         crate::common::error_logs_logic::error_log::ErrorLog::error_log(
        //             &error,
        //             app_info_state.as_ref(),
        //         );
        //         crate::repositories_types::tufa_server::routes::api::cats::create_or_update_by_id::TryCreateOrUpdateByIdResponseVariants::from(error)
        //     }
        // }
        todo!()
    }
}

impl CreateParameters {
    pub async fn prepare_and_execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::create::TryCreateResponseVariants
    {
        let query_string = format!(
            "{} {} {}(name, color) {} ($1, $2)",
            crate::server::postgres::constants::INSERT_NAME,
            crate::server::postgres::constants::INTO_NAME,
            crate::repositories_types::tufa_server::routes::api::cats::CATS,
            crate::server::postgres::constants::VALUES_NAME
        );
        println!("{query_string}");
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            query = query.bind(self.payload.name).bind(self.payload.color);
            query
        };
        match binded_query
            .execute(app_info_state.get_postgres_pool())
            .await
        {
            Ok(_) => {
                //todo - is need to return rows affected?
                crate::repositories_types::tufa_server::routes::api::cats::create::TryCreateResponseVariants::Desirable(())
            }
            Err(e) => {
                let error =
                    crate::repositories_types::tufa_server::routes::api::cats::create::TryCreate::from(
                        e,
                    );
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                crate::repositories_types::tufa_server::routes::api::cats::create::TryCreateResponseVariants::from(error)
            }
        }
    }
}

impl DeleteByIdParameters {
    pub async fn prepare_and_execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants
    {
        let query_string = format!(
            "{} {} {} {} id = $1",
            crate::server::postgres::constants::DELETE_NAME,
            crate::server::postgres::constants::FROM_NAME,
            crate::repositories_types::tufa_server::routes::api::cats::CATS,
            crate::server::postgres::constants::WHERE_NAME
        );
        println!("{query_string}");
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            query = query.bind(self.path.id.to_inner());
            query
        };
        match binded_query
            .execute(app_info_state.get_postgres_pool())
            .await
        {
            Ok(_) => {
                //todo - is need to return rows affected?
                crate::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::Desirable(())
            }
            Err(e) => {
                let error =
                    crate::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteById::from(
                        e,
                    );
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                crate::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::from(error)
            }
        }
    }
}

impl DeleteParameters {
    pub async fn prepare_and_execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants
    {
        let query_string = {
            let mut query = format!(
                "{} {} {} {} ",
                crate::server::postgres::constants::DELETE_NAME,
                crate::server::postgres::constants::FROM_NAME,
                crate::repositories_types::tufa_server::routes::api::cats::CATS,
                crate::server::postgres::constants::WHERE_NAME
            );
            match (&self.query.name, &self.query.color) {
                (None, None) => {
                    return crate::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::NoParameters { 
                        no_parameters: std::string::String::from("no parameters"), 
                        code_occurence: crate::code_occurence_tufa_common!(),
                    };
                },
                (None, Some(_)) => {
                    query.push_str("color = $1");
                },
                (Some(_), None) => {
                    query.push_str("name = $1");
                },
                (Some(_), Some(_)) => {
                    query.push_str("name = $1 AND color = $2");
                },
            }
            query
        };
        println!("{query_string}");
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match (self.query.name, self.query.color) {
                (None, None) => {
                    return crate::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::NoParameters { 
                        no_parameters: std::string::String::from("no parameters"), 
                        code_occurence: crate::code_occurence_tufa_common!(),
                    };
                },
                (None, Some(color)) => {
                    query = query.bind(color);
                },
                (Some(name), None) => {
                    query = query.bind(name);
                },
                (Some(name), Some(color)) => {
                    query = query.bind(name);
                    query = query.bind(color);
                },
            }
            query
        };
        match binded_query
            .execute(app_info_state.get_postgres_pool())
            .await
        {
            Ok(_) => {
                //todo - is need to return rows affected?
                crate::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::Desirable(())
            }
            Err(e) => {
                let error =
                    crate::repositories_types::tufa_server::routes::api::cats::delete::TryDelete::from(
                        e,
                    );
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                crate::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::from(error)
            }
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct DeleteQueryForUrlEncoding {
    name: Option<std::string::String>,
    color: Option<std::string::String>,
}
impl DeleteQuery {
    fn into_url_encoding_version(self) -> DeleteQueryForUrlEncoding {
        let name = self.name.map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        let color = self.color.map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        DeleteQueryForUrlEncoding {
            name,
            color,
        }
    }
}

#[derive(Debug, serde :: Deserialize)]
pub struct DeleteWithBodyParameters {
    pub payload: DeleteWithBodyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DeleteWithBodyPayload {
    pub select: CatColumnSelect,
    pub ids: Option<Vec<crate::server::postgres::bigserial::Bigserial>>,
    pub name_regex: Option<Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub color_regex: Option<Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub order_by: crate::server::postgres::order_by::OrderBy<CatColumn>,
    pub limit: crate::server::postgres::postgres_number::PostgresNumber,
    pub offset: crate::server::postgres::postgres_number::PostgresNumber,
}
impl DeleteWithBodyParameters {
    pub async fn prepare_and_execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::delete_with_body::TryDeleteWithBodyResponseVariants
    {
        // let query_string = {
        //     let mut query = format!(
        //         "{} {} {} {} ",
        //         crate::server::postgres::constants::DELETE_NAME,
        //         crate::server::postgres::constants::FROM_NAME,
        //         crate::repositories_types::tufa_server::routes::api::cats::CATS,
        //         crate::server::postgres::constants::WHERE_NAME
        //     );
        //     match (&self.query.name, &self.query.color) {
        //         (None, None) => {
        //             return crate::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::NoParameters { 
        //                 no_parameters: std::string::String::from("no parameters"), 
        //                 code_occurence: crate::code_occurence_tufa_common!(),
        //             };
        //         },
        //         (None, Some(_)) => {
        //             query.push_str("color = $1");
        //         },
        //         (Some(_), None) => {
        //             query.push_str("name = $1");
        //         },
        //         (Some(_), Some(_)) => {
        //             query.push_str("name = $1 AND color = $2");
        //         },
        //     }
        //     query
        // };
        // println!("{query_string}");
        // let binded_query = {
        //     let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        //     match (self.query.name, self.query.color) {
        //         (None, None) => {
        //             return crate::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::NoParameters { 
        //                 no_parameters: std::string::String::from("no parameters"), 
        //                 code_occurence: crate::code_occurence_tufa_common!(),
        //             };
        //         },
        //         (None, Some(color)) => {
        //             query = query.bind(color);
        //         },
        //         (Some(name), None) => {
        //             query = query.bind(name);
        //         },
        //         (Some(name), Some(color)) => {
        //             query = query.bind(name);
        //             query = query.bind(color);
        //         },
        //     }
        //     query
        // };
        // match binded_query
        //     .execute(app_info_state.get_postgres_pool())
        //     .await
        // {
        //     Ok(_) => {
        //         //todo - is need to return rows affected?
        //         crate::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::Desirable(())
        //     }
        //     Err(e) => {
        //         let error =
        //             crate::repositories_types::tufa_server::routes::api::cats::delete::TryDelete::from(
        //                 e,
        //             );
        //         crate::common::error_logs_logic::error_log::ErrorLog::error_log(
        //             &error,
        //             app_info_state.as_ref(),
        //         );
        //         crate::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::from(error)
        //     }
        // }
        todo!()
    }
}

impl ReadByIdParameters {
    pub async fn prepare_and_execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::read_by_id::TryReadByIdResponseVariants
    {
        let select = self.query.select.unwrap_or_default();
        let query_string = format!(
            "{} {} {} {} {} id = $1",
            crate::server::postgres::constants::SELECT_NAME,
            crate::server::postgres::generate_query::GenerateQuery::generate_query(&select),
             crate::server::postgres::constants::FROM_NAME,
            crate::repositories_types::tufa_server::routes::api::cats::CATS,
            crate::server::postgres::constants::WHERE_NAME,
        );
        println!("{query_string}");
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            query = query.bind(self.path.id.into_inner());
            query
        };
        match binded_query.fetch_one(app_info_state.get_postgres_pool()).await {
            Ok(row) => match select.options_try_from_sqlx_row(&row) {
                Ok(value) => crate::repositories_types::tufa_server::routes::api::cats::read_by_id::TryReadByIdResponseVariants::Desirable(value),
                Err(e) => {
                    let error = crate::repositories_types::tufa_server::routes::api::cats::read_by_id::TryReadById::from(e);
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    crate::repositories_types::tufa_server::routes::api::cats::read_by_id::TryReadByIdResponseVariants::from(error)
                },
            },
            Err(e) => {
                let error = crate::repositories_types::tufa_server::routes::api::cats::read_by_id::TryReadById::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                crate::repositories_types::tufa_server::routes::api::cats::read_by_id::TryReadByIdResponseVariants::from(error)
            },
        }
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ReadByIdQueryForUrlEncoding {
    select: Option<std::string::String>,
}
impl ReadByIdQuery {
    fn into_url_encoding_version(self) -> ReadByIdQueryForUrlEncoding {
        let select = self.select.map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        ReadByIdQueryForUrlEncoding {
            select
        }
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ReadWithBodyParameters {
    pub payload: ReadWithBodyPayload,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ReadWithBodyPayload {
    pub select: CatColumnSelect,
    pub ids: Option<Vec<crate::server::postgres::bigserial::Bigserial>>,
    pub name_regex: Option<Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub color_regex: Option<Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub order_by: crate::server::postgres::order_by::OrderBy<CatColumn>,
    pub limit: crate::server::postgres::postgres_number::PostgresNumber,
    pub offset: crate::server::postgres::postgres_number::PostgresNumber,
}

impl ReadWithBodyParameters {
    pub async fn prepare_and_execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::read_with_body::TryReadWithBodyResponseVariants
    {
        let query_string = {
            let mut query = std::string::String::from("");
            {
                query.push_str(&format!(
                    "{} {}",
                    crate::server::postgres::constants::SELECT_NAME,
                    crate::server::postgres::generate_query::GenerateQuery::generate_query(
                        &self.payload.select
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
                if let Some(value) = &self.payload.ids {
                    let prefix = match additional_parameters.is_empty() {
                        true => crate::server::postgres::constants::WHERE_NAME.to_string(),
                        false => format!(" {}", crate::server::postgres::constants::AND_NAME),
                    };
                    let bind_increments = {
                        let mut bind_increments = std::string::String::from("");
                        for element in value {
                            match crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments(
                                element,
                                &mut increment
                            ) {
                                Ok(bind_increments_handle) => {
                                    bind_increments.push_str(&format!("{bind_increments_handle}, "));
                                },
                                Err(e) => {
                                    return crate::repositories_types::tufa_server::routes::api::cats::read_with_body::TryReadWithBodyResponseVariants::BindQuery { 
                                        checked_add: e.into_serialize_deserialize_version(), 
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    };
                                },
                            }
                        }
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
                if let Some(value) = &self.payload.name_regex {
                    let prefix = match additional_parameters.is_empty() {
                        true => crate::server::postgres::constants::WHERE_NAME.to_string(),
                        false => format!(" {}", crate::server::postgres::constants::AND_NAME),
                    };
                    let column_name = "name";
                    let bind_increments = {
                        let mut bind_increments = std::string::String::from("");
                        for (index, element) in value.iter().enumerate() {
                            let conjuctive_operator = &element.conjuctive_operator;
                            match crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments(
                                element,
                                &mut increment
                            ) {
                                Ok(bind_increments_handle) => match index == 0 {
                                    true => {
                                        bind_increments.push_str(&format!("{column_name} ~ {bind_increments_handle} "));
                                    },
                                    false => {
                                        bind_increments.push_str(&format!("{conjuctive_operator} {column_name} ~ {bind_increments_handle} "));
                                    },
                                },
                                Err(e) => {
                                    return crate::repositories_types::tufa_server::routes::api::cats::read_with_body::TryReadWithBodyResponseVariants::BindQuery { 
                                        checked_add: e.into_serialize_deserialize_version(), 
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    };
                                },
                            }
                        }
                        if let false = bind_increments.is_empty() {
                            bind_increments.pop();
                        }
                        bind_increments
                    };
                    additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
                }
                if let Some(value) = &self.payload.color_regex {
                    let prefix = match additional_parameters.is_empty() {
                        true => crate::server::postgres::constants::WHERE_NAME.to_string(),
                        false => format!(" {}", crate::server::postgres::constants::AND_NAME),
                    };
                    let column_name = "color";
                    let bind_increments = {
                        let mut bind_increments = std::string::String::from("");
                        for (index, element) in value.iter().enumerate() {
                            let conjuctive_operator = &element.conjuctive_operator;
                            match crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments(
                                element,
                                &mut increment
                            ) {
                                Ok(bind_increments_handle) => match index == 0 {
                                    true => {
                                        bind_increments.push_str(&format!("{column_name} ~ {bind_increments_handle} "));
                                    },
                                    false => {
                                        bind_increments.push_str(&format!("{conjuctive_operator} {column_name} ~ {bind_increments_handle} "));
                                    },
                                },
                                Err(e) => {
                                    return crate::repositories_types::tufa_server::routes::api::cats::read_with_body::TryReadWithBodyResponseVariants::BindQuery { 
                                        checked_add: e.into_serialize_deserialize_version(), 
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    };
                                },
                            }
                        }
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
                    let value = &self.payload.order_by;
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
                    let value = match crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments(
                        &self.payload.limit,
                        &mut increment
                    ) {
                        Ok(value) => value,
                        Err(e) => {
                            return crate::repositories_types::tufa_server::routes::api::cats::read_with_body::TryReadWithBodyResponseVariants::BindQuery { 
                                checked_add: e.into_serialize_deserialize_version(), 
                                code_occurence: crate::code_occurence_tufa_common!(),
                            };
                        },
                    };
                    additional_parameters.push_str(&format!(
                        "{prefix}{} {value}",
                        crate::server::postgres::constants::LIMIT_NAME,
                    ));
                }
                {
                    let prefix = match additional_parameters.is_empty() {
                        true => "",
                        false => " ",
                    };
                    let value = match crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments(
                        &self.payload.offset,
                        &mut increment
                    ) {
                        Ok(value) => value,
                        Err(e) => {
                            return crate::repositories_types::tufa_server::routes::api::cats::read_with_body::TryReadWithBodyResponseVariants::BindQuery { 
                                checked_add: e.into_serialize_deserialize_version(), 
                                code_occurence: crate::code_occurence_tufa_common!(),
                            };
                        },
                    };
                    additional_parameters.push_str(&format!(
                        "{prefix}{} {value}",
                        crate::server::postgres::constants::OFFSET_NAME,
                    ));
                }
                additional_parameters
            };
            query.push_str(&format!(" {additional_parameters}"));
            println!("{query}");
            query
        };
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            if let Some(values) = self.payload.ids {
                for value in values {
                    query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                        value, query,
                    );
                }
            }
            if let Some(values) = self.payload.name_regex {
                for value in values {
                    query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                        value, query,
                    );
                }
            }
            if let Some(values) = self.payload.color_regex {
                for value in values {
                    query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                        value, query,
                    );
                }
            }
            query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                self.payload.limit,
                query,
            );
            query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                self.payload.offset,
                query,
            );
            query
        };
        let vec_values = {
            let mut rows = binded_query.fetch(app_info_state.get_postgres_pool());
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
                        let error = crate::repositories_types::tufa_server::routes::api::cats::read_with_body::TryReadWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return crate::repositories_types::tufa_server::routes::api::cats::read_with_body::TryReadWithBodyResponseVariants::from(error);
                    }
                }
            } {
                match self.payload.select.options_try_from_sqlx_row(&row) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        let error = crate::repositories_types::tufa_server::routes::api::cats::read_with_body::TryReadWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return crate::repositories_types::tufa_server::routes::api::cats::read_with_body::TryReadWithBodyResponseVariants::from(error);
                    }
                }
            }
            vec_values
        };
        crate::repositories_types::tufa_server::routes::api::cats::read_with_body::TryReadWithBodyResponseVariants::Desirable(vec_values)
    }
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ReadQueryForUrlEncoding {
    select: Option<std::string::String>,
    id: Option<std::string::String>,
    name: Option<std::string::String>,
    color: Option<std::string::String>,
    order_by: Option<std::string::String>,
    limit: std::string::String,
    offset: Option<std::string::String>,
}
impl ReadQuery {
    fn into_url_encoding_version(self) -> ReadQueryForUrlEncoding {
        let select = self.select.map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        let id = self.id.map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        let name = self.name.map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        let color = self.color.map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        let order_by = self.order_by.map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        let limit =
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                self.limit,
            );
        let offset = self.offset.map(|value| {
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

impl ReadParameters {
    pub async fn prepare_and_execute_query(
        self, //impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuer + crate::server::postgres::generate_query::GenerateQuery
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::read::TryReadResponseVariants
    {
        let select = CatColumnSelect::from(self.query.select.clone());
        let query_string = {
            let mut query = std::string::String::from("");
            {
                query.push_str(&format!(
                    "{} {}",
                    crate::server::postgres::constants::SELECT_NAME,
                    crate::server::postgres::generate_query::GenerateQuery::generate_query(&select)
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
                if let Some(value) = &self.query.id {
                    let prefix = match additional_parameters.is_empty() {
                        true => crate::server::postgres::constants::WHERE_NAME.to_string(),
                        false => format!(" {}", crate::server::postgres::constants::AND_NAME),
                    };
                    let value = match crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments(
                        value,
                        &mut increment
                    ) {
                        Ok(value) => value,
                        Err(e) => {
                            return crate::repositories_types::tufa_server::routes::api::cats::read::TryReadResponseVariants::BindQuery { 
                                checked_add: e.into_serialize_deserialize_version(), 
                                code_occurence: crate::code_occurence_tufa_common!(),
                            };
                        },
                    };
                    additional_parameters.push_str(&format!(
                        "{prefix} id = {}({}[{value}])",
                        crate::server::postgres::constants::ANY_NAME,
                        crate::server::postgres::constants::ARRAY_NAME,
                    ));
                }
                if let Some(value) = &self.query.name {
                    let prefix = match additional_parameters.is_empty() {
                        true => crate::server::postgres::constants::WHERE_NAME.to_string(),
                        false => format!(" {}", crate::server::postgres::constants::AND_NAME),
                    };
                    let value = match crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments(
                        value,
                        &mut increment
                    ) {
                        Ok(value) => value,
                        Err(e) => {
                            return crate::repositories_types::tufa_server::routes::api::cats::read::TryReadResponseVariants::BindQuery { 
                                checked_add: e.into_serialize_deserialize_version(), 
                                code_occurence: crate::code_occurence_tufa_common!(),
                            };
                        },
                    };
                    additional_parameters.push_str(&format!(
                        "{prefix} name = {}({}[{value}])",
                        crate::server::postgres::constants::ANY_NAME,
                        crate::server::postgres::constants::ARRAY_NAME,
                    ));
                }
                if let Some(value) = &self.query.color {
                    let prefix = match additional_parameters.is_empty() {
                        true => crate::server::postgres::constants::WHERE_NAME.to_string(),
                        false => format!(" {}", crate::server::postgres::constants::AND_NAME),
                    };
                    let value = match crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments(
                        value,
                        &mut increment
                    ) {
                        Ok(value) => value,
                        Err(e) => {
                            return crate::repositories_types::tufa_server::routes::api::cats::read::TryReadResponseVariants::BindQuery { 
                                checked_add: e.into_serialize_deserialize_version(), 
                                code_occurence: crate::code_occurence_tufa_common!(),
                            };
                        },
                    };
                    additional_parameters.push_str(&format!(
                        "{prefix} color = {}({}[{value}])",
                        crate::server::postgres::constants::ANY_NAME,
                        crate::server::postgres::constants::ARRAY_NAME,
                    ));
                }
                if let Some(value) = &self.query.order_by {
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
                    let value = match crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments(
                        &self.query.limit,
                        &mut increment
                    ) {
                        Ok(value) => value,
                        Err(e) => {
                            return crate::repositories_types::tufa_server::routes::api::cats::read::TryReadResponseVariants::BindQuery { 
                                checked_add: e.into_serialize_deserialize_version(), 
                                code_occurence: crate::code_occurence_tufa_common!(),
                            };
                        },
                    };
                    additional_parameters.push_str(&format!(
                        "{prefix}{} {value}",
                        crate::server::postgres::constants::LIMIT_NAME,
                    ));
                }
                if let Some(value) = &self.query.offset {
                    let prefix = match additional_parameters.is_empty() {
                        true => "",
                        false => " ",
                    };
                    let value = match crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments(
                        value,
                        &mut increment
                    ) {
                        Ok(value) => value,
                        Err(e) => {
                            return crate::repositories_types::tufa_server::routes::api::cats::read::TryReadResponseVariants::BindQuery { 
                                checked_add: e.into_serialize_deserialize_version(), 
                                code_occurence: crate::code_occurence_tufa_common!(),
                            };
                        },
                    };
                    additional_parameters.push_str(&format!(
                        "{prefix}{} {value}",
                        crate::server::postgres::constants::OFFSET_NAME,
                    ));
                }
                additional_parameters
            };
            query.push_str(&format!(" {additional_parameters}"));
            println!("{query}");
            query
        };
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            if let Some(value) = self.query.id {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            if let Some(value) = self.query.name {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            if let Some(value) = self.query.color {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                self.query.limit,
                query,
            );
            if let Some(value) = self.query.offset {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            query
        };
        let vec_values = {
            let mut rows = binded_query.fetch(app_info_state.get_postgres_pool());
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

impl UpdateByIdParameters {
    pub async fn prepare_and_execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants
    {
        let query_string = {
            let mut query = format!(
                "{} {} {} ",
                crate::server::postgres::constants::UPDATE_NAME,
                crate::repositories_types::tufa_server::routes::api::cats::CATS,
                crate::server::postgres::constants::SET_NAME,
            );
            let mut increment: u64 = 0;
            match (&self.payload.name, &self.payload.color) {
                (None, None) => {
                    return crate::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants::NoPayloadFields { 
                        no_payload_fields: std::string::String::from("no payload fields"), 
                        code_occurence: crate::code_occurence_tufa_common!()
                    };
                },
                (None, Some(_)) => {
                    match increment.checked_add(1) {
                        Some(incr) => {
                            increment = incr;
                        },
                        None => {
                            return crate::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants::CheckedAdd { 
                                checked_add: std::string::String::from("checked_add is None"), 
                                code_occurence: crate::code_occurence_tufa_common!(), 
                            }
                        },
                    }
                    query.push_str(&format!("color = ${increment}"));
                },
                (Some(_), None) => {
                    match increment.checked_add(1) {
                        Some(incr) => {
                            increment = incr;
                        },
                        None => {
                            return crate::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants::CheckedAdd { 
                                checked_add: std::string::String::from("checked_add is None"), 
                                code_occurence: crate::code_occurence_tufa_common!(), 
                            }
                        },
                    }
                    query.push_str(&format!("name = ${increment}"));
                },
                (Some(_), Some(_)) => {
                    match increment.checked_add(1) {
                        Some(incr) => {
                            increment = incr;
                        },
                        None => {
                            return crate::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants::CheckedAdd { 
                                checked_add: std::string::String::from("checked_add is None"), 
                                code_occurence: crate::code_occurence_tufa_common!(), 
                            }
                        },
                    }
                    query.push_str(&format!("name = ${increment}, "));
                    match increment.checked_add(1) {
                        Some(incr) => {
                            increment = incr;
                        },
                        None => {
                            return crate::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants::CheckedAdd { 
                                checked_add: std::string::String::from("checked_add is None"), 
                                code_occurence: crate::code_occurence_tufa_common!(), 
                            }
                        },
                    }
                    query.push_str(&format!("color = ${increment}"));
                },
            }
            match increment.checked_add(1) {
                Some(incr) => {
                    increment = incr;
                },
                None => {
                    return crate::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants::CheckedAdd { 
                        checked_add: std::string::String::from("checked_add is None"), 
                        code_occurence: crate::code_occurence_tufa_common!(), 
                    }
                },
            }
            query.push_str(&format!(
                " {} id = ${increment}",
                crate::server::postgres::constants::WHERE_NAME,
            ));
            query
        };
        println!("{query_string}");
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match (self.payload.name, self.payload.color) {
                (None, None) => {
                    return crate::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants::NoPayloadFields { 
                        no_payload_fields: std::string::String::from("no payload fields"),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    };
                },
                (None, Some(color)) => {
                    query = query.bind(color);
                },
                (Some(name), None) => {
                    query = query.bind(name);
                },
                (Some(name), Some(color)) => {
                    query = query.bind(name);
                    query = query.bind(color);
                },
            }
            query = query.bind(self.path.id.into_inner());
            query
        };
        match binded_query
            .execute(app_info_state.get_postgres_pool())
            .await
        {
            Ok(_) => {
                //todo - is need to return rows affected?
                crate::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants::Desirable(())
            }
            Err(e) => {
                let error =
                    crate::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateById::from(
                        e,
                    );
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                crate::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants::from(error)
            }
        }
    }
}

#[derive(Debug, serde :: Deserialize)]
pub struct UpdateParameters {
    pub payload: Vec<UpdatePayloadElement>,
}
#[derive(Debug, serde_derive :: Serialize, serde_derive :: Deserialize)]
pub struct UpdatePayloadElement {
    pub id: crate::server::postgres::bigserial::Bigserial,
    pub name: std::string::String,
    pub color: std::string::String,
}
impl UpdateParameters {
    pub async fn prepare_and_execute_query(
        self,
        app_info_state: &crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
    ) -> crate::repositories_types::tufa_server::routes::api::cats::update::TryUpdateResponseVariants
    {
        let query_string = {
            let mut values = std::string::String::from("");
            let mut increment: u64 = 0;
            for _ in &self.payload {
                let mut element_value = std::string::String::default();
                match increment.checked_add(1) {
                    Some(incr) => {
                        increment = incr;
                        element_value.push_str(&format!("${increment}, "));
                    },
                    None => {
                        return crate::repositories_types::tufa_server::routes::api::cats::update::TryUpdateResponseVariants::CheckedAdd { 
                            checked_add: std::string::String::from("checked_add is None"), 
                            code_occurence: crate::code_occurence_tufa_common!(), 
                        }
                    },
                }
                match increment.checked_add(1) {
                    Some(incr) => {
                        increment = incr;
                        element_value.push_str(&format!("${increment}, "));
                    },
                    None => {
                        return crate::repositories_types::tufa_server::routes::api::cats::update::TryUpdateResponseVariants::CheckedAdd { 
                            checked_add: std::string::String::from("checked_add is None"), 
                            code_occurence: crate::code_occurence_tufa_common!(), 
                        }
                    },
                }
                match increment.checked_add(1) {
                    Some(incr) => {
                        increment = incr;
                        element_value.push_str(&format!("${increment}, "));
                    },
                    None => {
                        return crate::repositories_types::tufa_server::routes::api::cats::update::TryUpdateResponseVariants::CheckedAdd { 
                            checked_add: std::string::String::from("checked_add is None"), 
                            code_occurence: crate::code_occurence_tufa_common!(), 
                        }
                    },
                }
                element_value.pop();
                element_value.pop();
                values.push_str(&format!("({element_value}), "));
            }
            values.pop();
            values.pop();
            format!(
                "{} {} {} t {} name = data.name, color = data.color {} (values {values}) as data(id, name, color) where t.id = data.id",
                crate::server::postgres::constants::UPDATE_NAME,
                crate::repositories_types::tufa_server::routes::api::cats::CATS,
                crate::server::postgres::constants::AS_NAME,
                crate::server::postgres::constants::SET_NAME,
                crate::server::postgres::constants::FROM_NAME,
            )
        };
        println!("{query_string}");
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            for element in self.payload {
                query = query.bind(element.id.into_inner());
                query = query.bind(element.name);
                query = query.bind(element.color);
            }
            query
        };
        match binded_query
            .execute(app_info_state.get_postgres_pool())
            .await
        {
            Ok(_) => {
                //todo - is need to return rows affected?
                crate::repositories_types::tufa_server::routes::api::cats::update::TryUpdateResponseVariants::Desirable(())
            }
            Err(e) => {
                let error =
                    crate::repositories_types::tufa_server::routes::api::cats::update::TryUpdate::from(
                        e,
                    );
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                crate::repositories_types::tufa_server::routes::api::cats::update::TryUpdateResponseVariants::from(error)
            }
        }
    }
}
//