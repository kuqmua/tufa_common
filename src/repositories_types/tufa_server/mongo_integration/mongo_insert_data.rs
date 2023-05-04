#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertDataErrorNamed<'a> {
    Errors {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        errors_hashmap: std::collections::HashMap<std::string::String, MongoInsertDataErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertDataErrorUnnamed<'a> {
    MongoInsertDocsInEmptyCollection(crate::server::mongo::mongo_insert_docs_in_empty_collection::MongoInsertDocsInEmptyCollectionErrorNamed<'a>)
}

pub async fn mongo_insert_data<'a>(
    db_name_handle: &'a str,
    vec_of_link_parts_hashmap: std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,//todo impl Display instead of ProviderKind
) -> Result<(), Box<crate::repositories_types::tufa_server::mongo_integration::mongo_insert_data::MongoInsertDataErrorNamed<'a>>> {
    let error_hashmap = futures::future::join_all(vec_of_link_parts_hashmap.into_iter().map(
        |(pk, vec_of_link_parts)| async move {
            (
                pk,
                crate::server::mongo::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection(
                    {
                        use std::ops::Deref;
                        crate::global_variables::runtime::mongo_client_options::MONGO_CLIENT_OPTIONS.deref().to_owned()
                    },
                    db_name_handle,
                    format!(
                        "{pk}{}",
                        crate::global_variables::runtime::config::CONFIG.mongo_providers_logs_db_collection_handle_second_part
                    ),
                    crate::global_variables::runtime::config::CONFIG
                        .mongo_providers_logs_db_collection_document_field_name_handle
                        .clone(),
                    vec_of_link_parts
                )
                .await,
            )
        },
    ))
    .await
    .into_iter()
    .filter_map(|(pk, result)| {
        if let Err(e) = result {
            return Some((
                pk.to_string(), 
                crate::repositories_types::tufa_server::mongo_integration::mongo_insert_data::MongoInsertDataErrorUnnamed::MongoInsertDocsInEmptyCollection(*e)
            ));
        }
        None
    })
    .collect::<std::collections::HashMap<std::string::String, crate::repositories_types::tufa_server::mongo_integration::mongo_insert_data::MongoInsertDataErrorUnnamed>>();
    if !error_hashmap.is_empty() {
        return Err(Box::new(
            crate::repositories_types::tufa_server::mongo_integration::mongo_insert_data::MongoInsertDataErrorNamed::Errors {
                errors_hashmap: error_hashmap,
                code_occurence: crate::code_occurence_tufa_common!()
            }
        ));
    }
    Ok(())
}