pub mod create;
pub mod create_batch;
pub mod delete;
pub mod delete_by_id;
pub mod delete_with_body;
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
// `DO` blocks cannot use bound parameters.  If you need to pass in values then you can create a temporary function and call that instead, though it's a bit more of a hassle.

impl UpdateByIdParameters {
    pub async fn
    prepare_and_execute_query(self, app_info_state : & crate ::
    repositories_types :: tufa_server :: routes :: api :: cats ::
    DynArcGetConfigGetPostgresPoolSendSync,) -> crate :: repositories_types ::
    tufa_server :: routes :: api :: cats :: update_by_id ::
    TryUpdateByIdResponseVariants
    {
        if let (None, None) = (&self.payload.name, &self.payload.color) {
            return crate :: repositories_types :: tufa_server :: routes :: api
            :: cats :: update_by_id :: TryUpdateByIdResponseVariants ::
            NoPayloadFields
            {
                no_payload_fields : std :: string :: String ::
                from("no payload fields"), code_occurence : crate ::
                code_occurence_tufa_common! ()
            } ;
        }
        let mut pool_connection = app_info_state.get_postgres_pool().acquire().await.unwrap();
        let pg_connection = sqlx::Acquire::acquire(&mut pool_connection).await.unwrap();
        let function_creation_query = sqlx::query::<sqlx::Postgres>(
            r#"
    create or replace function pg_temp.cat_update_by_id(cat_id bigint, cat_name varchar, cat_color varchar) returns void language plpgsql as $$ begin
    update cats set name = cat_name, color = cat_color where id = cat_id;if not found then raise exception 'cat id % not found', cat_id;end if;
    end $$;
            "#,
        );
        function_creation_query
            .execute(pg_connection.as_mut())
            .await
            .unwrap();
        println!("123");
        //     let mut query = sqlx::query::<sqlx::Postgres>(
        //         r#"
        // SELECT pg_temp.cats_update_by_id(cats_id => $1, cats_name => $2, cats_color => $3);
        //         "#,
        //     );
        //     query = query.bind(17);
        //     query = query.bind("name4");
        //     query = query.bind("color4");
        //     query.execute(pg_connection.as_mut()).await.unwrap();
        //     println!("456");
        //     crate :: repositories_types :: tufa_server :: routes :: api ::
        //             cats :: update_by_id :: TryUpdateByIdResponseVariants ::
        //             Desirable(())

        // let function_creation_query_stringified = {
        //     let create_or_replace_function_name = {
        //         let mut value = format!("cat_update_by_id");
        //         if self.payload.name.is_some() {
        //             value.push_str(&format!("_name"));
        //         }
        //         if self.payload.color.is_some() {
        //             value.push_str(&format!("_color"));
        //         }
        //         value
        //     };
        //     let create_or_replace_function_parameters = {
        //         let mut value = std::string::String::from("cat_id bigint, ");
        //         if self.payload.name.is_some() {
        //             value.push_str(&format!("cat_name varchar, "));
        //         }
        //         if self.payload.color.is_some() {
        //             value.push_str(&format!("cat_color varchar"));
        //         }
        //         value
        //     };
        //     let create_or_replace_function_first_line_query = {
        //         let mut value = format!(
        //             "{} {} {} ",
        //             crate::server::postgres::constants::UPDATE_NAME,
        //             crate::repositories_types::tufa_server::routes::api::cats::CATS,
        //             crate::server::postgres::constants::SET_NAME,
        //         );
        //         if self.payload.name.is_some() {
        //             value.push_str(&format!("name = id_name, "));
        //         }
        //         if self.payload.color.is_some() {
        //             value.push_str(&format!("color = id_color "));
        //         }
        //         value.push_str(&format!(
        //             "{} id = cat_id",
        //             crate::server::postgres::constants::WHERE_NAME
        //         ));
        //         value
        //     };
        //     let create_or_replace_function_second_line_query = std::string::String::from(
        //         "if not found then raise exception 'cats id % not found', cats_id",
        //     );
        //     let create_or_replace_function_third_line_query = std::string::String::from("end if");
        //     format!
        //     ("create or replace function pg_temp.{create_or_replace_function_name}({create_or_replace_function_parameters}) returns void language plpgsql as $$ begin {create_or_replace_function_first_line_query};{create_or_replace_function_second_line_query};{create_or_replace_function_third_line_query};end $$")
        // };
        // println!("function_creation_query_stringified {function_creation_query_stringified}");
        // let function_creation_query =
        //     sqlx::query::<sqlx::Postgres>(&function_creation_query_stringified);
        // function_creation_query
        //     .execute(pg_connection.as_mut())
        //     .await
        //     .unwrap();
        let query_string = {
            let mut increment: u64 = 0;
            let query_parameters = {
                let mut query = std::string::String::from("");
                match crate::server::postgres::bind_query::BindQuery::try_increment(
                    &self.path.id,
                    &mut increment,
                ) {
                    Ok(_) => {
                        query.push_str(&format!("cat_id => ${increment}, "));
                    }
                    Err(e) => {
                        return crate :: repositories_types :: tufa_server :: routes
                        :: api :: cats :: update_by_id ::
                        TryUpdateByIdResponseVariants :: BindQuery
                        {
                            checked_add : e.into_serialize_deserialize_version(),
                            code_occurence : crate :: code_occurence_tufa_common! (),
                        } ;
                    }
                }
                if let Some(value) = &self.payload.name {
                    match crate::server::postgres::bind_query::BindQuery::try_increment(
                        value,
                        &mut increment,
                    ) {
                        Ok(_) => {
                            query.push_str(&format!("cat_name => ${increment}, "));
                        }
                        Err(e) => {
                            return crate :: repositories_types :: tufa_server :: routes
                            :: api :: cats :: update_by_id ::
                            TryUpdateByIdResponseVariants :: BindQuery
                            {
                                checked_add : e.into_serialize_deserialize_version(),
                                code_occurence : crate :: code_occurence_tufa_common! ()
                            } ;
                        }
                    }
                }
                if let Some(value) = &self.payload.color {
                    match crate::server::postgres::bind_query::BindQuery::try_increment(
                        value,
                        &mut increment,
                    ) {
                        Ok(_) => {
                            query.push_str(&format!("cat_color => ${increment}"));
                        }
                        Err(e) => {
                            return crate :: repositories_types :: tufa_server :: routes
                            :: api :: cats :: update_by_id ::
                            TryUpdateByIdResponseVariants :: BindQuery
                            {
                                checked_add : e.into_serialize_deserialize_version(),
                                code_occurence : crate :: code_occurence_tufa_common! ()
                            } ;
                        }
                    }
                }
                query
            };
            format!(
                "{} pg_temp.cat_update_by_id({})",
                crate::server::postgres::constants::SELECT_NAME,
                query_parameters
            )
        };
        println!("query_string {query_string}");
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                self.path.id,
                query,
            );
            if let Some(value) = self.payload.name {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            if let Some(value) = self.payload.color {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            query
        };
        match binded_query.execute(pg_connection.as_mut()).await
        {
            Ok(_) =>
            {
                crate :: repositories_types :: tufa_server :: routes :: api ::
                cats :: update_by_id :: TryUpdateByIdResponseVariants ::
                Desirable(())
            } Err(e) =>
            {
                let error = crate :: repositories_types :: tufa_server ::
                routes :: api :: cats :: update_by_id :: TryUpdateById ::
                from(e) ; crate :: common :: error_logs_logic :: error_log ::
                ErrorLog :: error_log(& error, app_info_state.as_ref(),) ;
                crate :: repositories_types :: tufa_server :: routes :: api ::
                cats :: update_by_id :: TryUpdateByIdResponseVariants ::
                from(error)
            }
        }
    }
}

// let pool = sqlx::postgres::PgPoolOptions::new()
//     .connect("postgres://postgreslocalusername:postgreslocalpassword@127.0.0.1:5432/newsletter?connect_timeout=10")
//     .await.unwrap();
// //acquire or try_acquire
// let mut pool_connection = pool.acquire().await.unwrap();
// let pg_connection = sqlx::Acquire::acquire(&mut pool_connection).await.unwrap();
// let function_creation_query = sqlx::query::<sqlx::Postgres>(
//     r#"
// create or replace function pg_temp.cats_update_by_id(cats_id bigint, cats_name varchar, cats_color varchar) returns void language plpgsql as $$ begin
// update cats set name = cats_name, color = cats_color where id = cats_id;if not found then raise exception 'cats id % not found', cats_id;end if;
// end $$;
//         "#,
// );
// function_creation_query
//     .execute(pg_connection.as_mut())
//     .await
//     .unwrap();
// println!("123");
// let mut query = sqlx::query::<sqlx::Postgres>(
//     r#"
// SELECT pg_temp.cats_update_by_id(cats_id => $1, cats_name => $2, cats_color => $3);
//         "#,
// );
// query = query.bind(17);
// query = query.bind("name4");
// query = query.bind("color4");
// query.execute(pg_connection).await.unwrap();
