// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum PostgresInitErrorNamed<'a> {
//     CreateTableQueries{
//         #[eo_error_occurence]
//         create_table_queries: crate::server::postgres::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsErrorNamed<'a>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     CheckProviderLinksTablesAreEmpty{
//         #[eo_error_occurence]
//         check_provider_links_tables_are_empty: crate::server::postgres::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyErrorNamed<'a>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     DeleteAllFromProvidersTables{
//         #[eo_error_occurence]
//         delete_all_from_providers_tables: crate::server::postgres::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesErrorNamed<'a>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength{
//         #[eo_error_occurence]
//         check_providers_links_tables_length_rows_equal_initialization_data_length: crate::server::postgres::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorNamed<'a>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     InsertLinkPartsIntoProvidersTables{
//         #[eo_error_occurence]
//         insert_link_parts_into_providers_tables: crate::server::postgres::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorNamed<'a>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }

// pub async fn init_postgres<'a, SelfGeneric>(
//     providers_json_local_data_hashmap: std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
//     config: &'static (
//         impl crate::common::config::config_fields::GetPostgresPool
//         + std::marker::Send
//         + std::marker::Sync
//     )
// ) -> Result<(), Box<crate::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed<'a>>> {
//     if let Err(e) = crate::server::postgres::postgres_create_providers_tables_if_not_exists::postgres_create_providers_tables_if_not_exists(
//         &providers_json_local_data_hashmap,
//         config,
//     )
//     .await
//     {
//         return Err(Box::new(
//             crate::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed::CreateTableQueries {
//                 create_table_queries: *e,
//                 code_occurence: crate::code_occurence_tufa_common!()
//             }
//         ));
//     }
//     if let Err(e) = crate::server::postgres::postgres_check_providers_link_parts_tables_are_empty::postgres_check_providers_link_parts_tables_are_empty(
//         &providers_json_local_data_hashmap,
//         config,
//     )
//     .await
//     {
//         return Err(Box::new(
//             crate::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed::CheckProviderLinksTablesAreEmpty {
//                 check_provider_links_tables_are_empty: *e,
//                 code_occurence: crate::code_occurence_tufa_common!()
//             }
//         ));
//     }
//     if let Err(e) = crate::server::postgres::postgres_delete_all_from_providers_link_parts_tables::postgres_delete_all_from_providers_link_parts_tables(
//         &providers_json_local_data_hashmap,
//         config
//     )
//     .await
//     {
//         return Err(Box::new(
//             crate::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed::DeleteAllFromProvidersTables {
//                 delete_all_from_providers_tables: *e,
//                 code_occurence: crate::code_occurence_tufa_common!()
//             }
//         ));
//     }
//     if let Err(e) = crate::server::postgres::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length(
//         &providers_json_local_data_hashmap,
//         config,
//     )
//     .await {
//         return Err(Box::new(
//             crate::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength {
//                 check_providers_links_tables_length_rows_equal_initialization_data_length: *e,
//                 code_occurence: crate::code_occurence_tufa_common!()
//             }
//     ));
//     }
//     if let Err(e) = crate::server::postgres::postgres_insert_link_parts_into_providers_tables::postgres_insert_link_parts_into_providers_tables(
//         &providers_json_local_data_hashmap,
//         config,
//     )
//     .await
//     {
//         return Err(Box::new(
//             crate::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed::InsertLinkPartsIntoProvidersTables {
//                 insert_link_parts_into_providers_tables: *e,
//                 code_occurence: crate::code_occurence_tufa_common!()
//             }
//         ));
//     }
//     Ok(())
// }
