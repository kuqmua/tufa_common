// impl crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods for crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind {
//     fn get_item_handle(&self) -> Option<&'static str> {
//         match self {
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Arxiv => Some("</item>"),
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Biorxiv => Some("</item>"),
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Github => Some("</entry>"),
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Habr => Some("</item>"),
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Medrxiv => Some("</item>"),
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Reddit => None,
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Twitter => Some("</item>"),
//         }
//     }
//     fn get_mongo_log_collection_name(
//         &self,
//         config: &'static (
// impl crate::common::config::config_fields::GetMongoProvidersLogsDbCollectionHandleSecondPart
// + std::marker::Send
// + std::marker::Sync
// )
//     ) -> String {
//         format!(
//             "{self}second_part",
//         )
//     }
//     fn get_init_local_data_file_path(
//         &self,
//     ) -> String {
//         format!(
//             "./providers_link_parts/{self}_link_parts.json",
//         )
//     }
//     fn generate_provider_links(
//         &self,
//         names_vector: Vec<String>,
//         config: &'static (
// impl crate::common::config::config_fields::GetGithubToken
// + std::marker::Send
// + std::marker::Sync
// )
//     ) -> Vec<String> {
//         match self {
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Arxiv => crate::repositories_types::tufa_server::providers::providers_info::links::generate_arxiv_links::generate_arxiv_links(names_vector),
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Biorxiv => crate::repositories_types::tufa_server::providers::providers_info::links::generate_biorxiv_links::generate_biorxiv_links(names_vector),
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Github => crate::repositories_types::tufa_server::providers::providers_info::links::generate_github_links::generate_github_links(names_vector, config),
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Habr => crate::repositories_types::tufa_server::providers::providers_info::links::generate_habr_links::generate_habr_links(names_vector),
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Medrxiv => crate::repositories_types::tufa_server::providers::providers_info::links::generate_medrxiv_links::generate_medrxiv_links(names_vector),
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Reddit => crate::repositories_types::tufa_server::providers::providers_info::links::generate_reddit_links::generate_reddit_links(names_vector),
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Twitter => crate::repositories_types::tufa_server::providers::providers_info::links::generate_twitter_links::generate_twitter_links(names_vector),
//         }
//     }
//     fn generate_hashmap_with_empty_string_vecs_for_enabled_providers() -> std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>> {
//         crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::get_enabled_providers_vec()
//             .iter()
//             .map(|pk| (*pk, Vec::<String>::new()))
//             .collect()
//     }
//     fn get_enabled_providers_vec() -> Vec<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind> {
//         {
//             use strum::IntoEnumIterator;
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::iter()
//         }.filter(|pk| {
//             // use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
//             // *pk.is_enabled()
//             false
//         }).collect()
//     }
//     fn get_enabled_string_name_vec() -> Vec<String> {
//         {
//             use strum::IntoEnumIterator;
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::iter()
//         }
//             .filter_map(|pk| {
//                 if {
//                     // use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
//                     // *pk.is_enabled()
//                     false
//                 } {
//                     return Some(format!("{pk}"));
//                 }
//                 None
//             })
//             .collect()
//     }
//     fn get_mongo_initialization_provider_kind_vec() -> Vec<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind> {
//         {
//             use strum::IntoEnumIterator;
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::iter()
//         }
//             .filter(|pk| {
//                 // use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
//                 // *pk.is_mongo_initialization_enabled()
//                 false
//             })
//             .collect()
//     }
//     fn get_mongo_initialization_string_name_vec() -> Vec<String> {
//         {
//             use strum::IntoEnumIterator;
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::iter()
//         }
//             .filter_map(|pk| {
//                 // if {
//                 //     use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
//                 //     *pk.is_mongo_initialization_enabled(config)
//                 // } {
//                 //     return Some(format!("{pk}"));
//                 // }
//                 None
//             })
//             .collect()
//     }
//     fn into_string_name_and_kind_hashmap() -> std::collections::HashMap<String, crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind> {
//         {
//             use strum::IntoEnumIterator;
//             crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::iter()
//         }
//             .map(|pk| (format!("{pk}"), pk))
//             .collect()
//     }
//     fn get_db_tag(&self) -> String {
//         format!("{self}")
//     }
//     fn get_postgres_table_name(&self) -> String {
//         format!("{}_link_parts", self.to_lower_snake_case())
//     }
// }
