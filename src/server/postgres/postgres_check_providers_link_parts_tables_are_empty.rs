// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum PostgresCheckProvidersLinkPartsTablesEmptyErrorNamed<'a> {
//     SelectCountOrigin {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         hashmap_provider_kind_sqlx_error: std::collections::HashMap<std::string::String, sqlx::Error>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     NotEmptyOrigin {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         hashmap_provider_kind_len: std::collections::HashMap<std::string::String, i64>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     }
// }

// pub async fn postgres_check_providers_link_parts_tables_are_empty<'a>(
//     providers_json_local_data_hashmap: &std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
//     db: &sqlx::Pool<sqlx::Postgres>,
// ) -> Result<(), Box<crate::server::postgres::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyErrorNamed<'a>>> {
//     let count_provider_links_tables_tasks_vec =
//         providers_json_local_data_hashmap.keys().map(|pk| async {
//             let query_string = format!(
//                 "SELECT count(*) AS exact_count FROM {};",
//                 {
//                     use crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
//                     pk.get_postgres_table_name()
//                 }
//             );
//             (*pk, sqlx::query_as(&query_string).fetch_one(db).await)
//         });
//     let count_provider_links_tables_error_vec: Vec<(crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Result<(i64,), sqlx::Error>)> =
//         futures::future::join_all(count_provider_links_tables_tasks_vec).await;
//     let mut count_provider_links_tables_error_hashmap: std::collections::HashMap<std::string::String, sqlx::Error> =
//         std::collections::HashMap::with_capacity(count_provider_links_tables_error_vec.len());
//     let mut provider_links_tables_not_empty_error_hashmap: std::collections::HashMap<std::string::String, i64> =
//         std::collections::HashMap::with_capacity(count_provider_links_tables_error_vec.len());
//     for (pk, result) in count_provider_links_tables_error_vec {
//         match result {
//             Err(e) => {
//                 count_provider_links_tables_error_hashmap.insert(pk.to_string(), e);
//             }
//             Ok((count,)) => {
//                 if count > 0 {
//                     provider_links_tables_not_empty_error_hashmap.insert(pk.to_string(), count);
//                 }
//             }
//         }
//     }
//     if !count_provider_links_tables_error_hashmap.is_empty() {
//         return Err(Box::new(
//             crate::server::postgres::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyErrorNamed::SelectCountOrigin {
//                 hashmap_provider_kind_sqlx_error: count_provider_links_tables_error_hashmap,
//                 code_occurence: crate::code_occurence_tufa_common!()
//             }
//         ));
//     }
//     if !provider_links_tables_not_empty_error_hashmap.is_empty() {
//         return Err(Box::new(
//             crate::server::postgres::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyErrorNamed::NotEmptyOrigin {
//                 hashmap_provider_kind_len: provider_links_tables_not_empty_error_hashmap,
//                 code_occurence: crate::code_occurence_tufa_common!()
//             }
//         ));
//     }
//     Ok(())
// }
