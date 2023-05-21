// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum MongoInsertDataErrorNamed<'a> {
//     Errors {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         errors_hashmap: std::collections::HashMap<std::string::String, MongoInsertDataErrorUnnamed<'a>>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum MongoInsertDataErrorUnnamed<'a> {
//     MongoInsertDocsInEmptyCollection(crate::server::mongo::mongo_insert_docs_in_empty_collection::MongoInsertDocsInEmptyCollectionErrorNamed<'a>)
// }

// pub async fn mongo_insert_data<'a>(
//     db_name_handle: &'a str,
//     vec_of_link_parts_hashmap: std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,//todo impl Display instead of ProviderKind
//     mongodb_options_client_options: mongodb::options::ClientOptions,
//     config: &'static (
//         impl crate::common::config::config_fields::GetMongoProvidersLogsDbCollectionHandleSecondPart
//         + crate::common::config::config_fields::GetMongoProvidersLogsDbCollectionDocumentFieldNameHandle
        // + std::marker::Send 
        // + std::marker::Sync
//     )
// ) -> Result<(), Box<crate::server::mongo::mongo_insert_data::MongoInsertDataErrorNamed<'a>>> {
//     let error_hashmap = futures::future::join_all(vec_of_link_parts_hashmap.into_iter().map(
//         |(pk, vec_of_link_parts)| {
//             let mongodb_options_client_options_handle = mongodb_options_client_options.clone();
//             async move {
//                 (
//                     pk,
//                     crate::server::mongo::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection(
//                         mongodb_options_client_options_handle,
//                         db_name_handle,
//                         format!(
//                             "{pk}second_part",
//                         ),
//                         "data",
//                         vec_of_link_parts
//                     )
//                     .await,
//                 )
//             }
//         },
//     ))
//     .await
//     .into_iter()
//     .filter_map(|(pk, result)| {
//         if let Err(e) = result {
//             return Some((
//                 pk.to_string(), 
//                 crate::server::mongo::mongo_insert_data::MongoInsertDataErrorUnnamed::MongoInsertDocsInEmptyCollection(*e)
//             ));
//         }
//         None
//     })
//     .collect::<std::collections::HashMap<std::string::String, crate::server::mongo::mongo_insert_data::MongoInsertDataErrorUnnamed>>();
//     if !error_hashmap.is_empty() {
//         return Err(Box::new(
//             crate::server::mongo::mongo_insert_data::MongoInsertDataErrorNamed::Errors {
//                 errors_hashmap: error_hashmap,
//                 code_occurence: crate::code_occurence_tufa_common!()
//             }
//         ));
//     }
//     Ok(())
// }