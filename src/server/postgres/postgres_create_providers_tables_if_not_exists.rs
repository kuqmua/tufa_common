// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum PostgresCreateProvidersDbsErrorNamed<'a> {
//     Postgres {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         sqlx_error_hashmap: std::collections::HashMap<std::string::String, sqlx::Error>,//todo key ProviderKind instead of std::string::String
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }

// pub async fn postgres_create_providers_tables_if_not_exists<'a>(
//     providers_json_local_data_hashmap: &std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
//     postgres_pool: &sqlx::Pool<sqlx::Postgres>,
// ) -> Result<(), Box<crate::server::postgres::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsErrorNamed<'a>>> {
//     let table_creation_error_hashmap = futures::future::join_all(
//         providers_json_local_data_hashmap.keys().map(|pk| async {
//             let query_string = format!(
//                 "CREATE TABLE IF NOT EXISTS {} (id integer GENERATED ALWAYS AS IDENTITY NOT NULL, link_part text, PRIMARY KEY (id));",
//                 {
//                     use crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
//                     pk.get_postgres_table_name()
//                 }
//             );
//             (*pk, sqlx::query(&query_string).execute(postgres_pool).await)
//         })).await
//         .into_iter()
//         .filter_map(|(pk, result)| {
//             if let Err(e) = result {
//                 return Some((pk.to_string(), e));
//             }
//             None
//         })
//         .collect::<std::collections::HashMap<std::string::String, sqlx::Error>>();
//     if !table_creation_error_hashmap.is_empty() {
//         return Err(Box::new(
//             crate::server::postgres::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsErrorNamed::Postgres {
//                 sqlx_error_hashmap: table_creation_error_hashmap,
//                 code_occurence: crate::code_occurence_tufa_common!()
//             }
//         ));
//     }
//     Ok(())
// }
