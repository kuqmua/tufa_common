// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorNamed<'a> {
//     SelectCountOrigin {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         inner_errors: std::collections::HashMap<std::string::String, PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxUnnamed<'a>>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     ProviderLinksTablesRowsLengthNotEqualOrigin {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         inner_errors: std::collections::HashMap<std::string::String, PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorUnnamed<'a>>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxUnnamed<
//     'a,
// > {
//     Postgres(PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxNamed<'a>),
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxNamed<
//     'a,
// > {
//     Postgres {
//         #[eo_display]
//         error: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorUnnamed<
//     'a,
// > {
//     Postgres(PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorNamed<'a>),
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorNamed<
//     'a,
// > {
//     Postgres {
//         #[eo_display_with_serialize_deserialize]
//         table_rows_length: i64,
//         #[eo_display_with_serialize_deserialize]
//         initialization_data_length: usize,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }

// pub async fn postgres_check_providers_links_tables_length_rows_equal_initialization_data_length<'a>(
//     providers_json_local_data_hashmap: &std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
//     postgres_pool: &sqlx::Pool<sqlx::Postgres>,
// ) -> Result<
//     (),
//     Box<PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorNamed<'a>>,
// >{
//     let count_provider_links_tables_tasks_vec =
//         providers_json_local_data_hashmap
//             .iter()
//             .map(|(pk, string_vec)| async move {
//                 let query_string = format!(
//                     "SELECT count(*) AS exact_count FROM {};",
//                     {
//                         use crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
//                         pk.get_postgres_table_name()
//                     }
//                 );
//                 (
//                     pk,
//                     string_vec,
//                     sqlx::query_as(&query_string).fetch_one(postgres_pool).await,
//                 )
//             });
//     let count_provider_links_tables_error_vec: Vec<(
//         &crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind,
//         &Vec<String>,
//         Result<(i64,), sqlx::Error>,
//     )> = futures::future::join_all(count_provider_links_tables_tasks_vec).await;
//     let mut count_provider_links_tables_error_hashmap: std::collections::HashMap<std::string::String, PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxUnnamed> =
//         std::collections::HashMap::new();
//     let mut provider_links_tables_rows_length_not_equal_error_hashmap: std::collections::HashMap<
//         std::string::String,
//         PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorUnnamed,
//     > = std::collections::HashMap::new();
//     for (pk, string_vec, result) in count_provider_links_tables_error_vec {
//         match result {
//             Err(e) => {
//                 count_provider_links_tables_error_hashmap.insert(
//                     pk.to_string(),
//                  PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxUnnamed::Postgres(
//                         PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxNamed::Postgres {
//                             error: e,
//                             code_occurence: crate::code_occurence_tufa_common!()
//                         }
//                     )
//                 );
//             }
//             Ok((count,)) => {
//                 if count != string_vec.len() as i64 {
//                     provider_links_tables_rows_length_not_equal_error_hashmap.insert(
//                         pk.to_string(),
//                         PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorUnnamed::Postgres(
//                             PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorNamed::Postgres {
//                                 table_rows_length: count,
//                                 initialization_data_length: string_vec.len(),
//                                 code_occurence: crate::code_occurence_tufa_common!()
//                             }
//                         )
//                     );
//                 }
//             }
//         }
//     }
//     if !count_provider_links_tables_error_hashmap.is_empty() {
//         return Err(Box::new(
//             PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorNamed::SelectCountOrigin {
//                 inner_errors: count_provider_links_tables_error_hashmap,
//                 code_occurence: crate::code_occurence_tufa_common!()
//             }
//         ));
//     }
//     if !provider_links_tables_rows_length_not_equal_error_hashmap.is_empty() {
//         return Err(Box::new(
//             PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorNamed::ProviderLinksTablesRowsLengthNotEqualOrigin {
//                 inner_errors: provider_links_tables_rows_length_not_equal_error_hashmap,
//                 code_occurence: crate::code_occurence_tufa_common!()
//             }
//         ));
//     }
//     Ok(())
// }
