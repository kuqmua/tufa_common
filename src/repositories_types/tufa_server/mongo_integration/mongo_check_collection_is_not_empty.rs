#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckCollectionIsNotEmptyErrorNamed<'a> {
    Mongo {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        inner_errors:
            std::collections::HashMap<std::string::String, MongoCheckCollectionIsNotEmptyErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckCollectionIsNotEmptyErrorUnnamed<'a> {
    CountDocumentsOrigin(MongoCheckCollectionIsNotEmptyErrorCountDocumentsErrorNamed<'a>),
    IsNotEmptyOrigin(MongoCheckCollectionIsNotEmptyErrorIsNotEmptyOriginErrorNamed<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckCollectionIsNotEmptyErrorCountDocumentsErrorNamed<'a> {
    CountDocuments {
        #[eo_display_foreign_type]
        error: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckCollectionIsNotEmptyErrorIsNotEmptyOriginErrorNamed<'a> {
    IsNotEmptyOrigin {
        #[eo_display_with_serialize_deserialize]
        error: u64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_check_collections_is_not_empty<'a>(
    providers_json_local_data_hashmap: std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
    db: &mongodb::Database,
) -> Result<(), Box<crate::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorNamed<'a>>>{
    let error_vec_count_documents =
        futures::future::join_all(providers_json_local_data_hashmap.keys().map(|pk| async move {
            (
                *pk,
                db.collection::<mongodb::bson::Document>(&format!("{pk}"))
                    .count_documents(None, None)
                    .await,
            )
        }))
        .await
        .into_iter()
        .filter_map(|(pk, result)| match result {
            Err(e) => Some((
                pk.to_string(),
                crate::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorUnnamed::CountDocumentsOrigin(
                    crate::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorCountDocumentsErrorNamed::CountDocuments { 
                        error: e, 
                        code_occurence: crate::code_occurence_tufa_common!(), 
                    }
                ),
            )),
            Ok(documents_number) => {
                if documents_number > 0 {
                    return Some((
                        pk.to_string(),
                        crate::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorUnnamed::IsNotEmptyOrigin(
                            crate::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorIsNotEmptyOriginErrorNamed::IsNotEmptyOrigin { 
                                error: documents_number, 
                                code_occurence: crate::code_occurence_tufa_common!()
                            }
                        )
                    ));
                }
                None
            }
        })
        .collect::<std::collections::HashMap<String, crate::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorUnnamed>>();
    if !error_vec_count_documents.is_empty() {
        return Err(Box::new(
            crate::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorNamed::Mongo {        
                inner_errors: error_vec_count_documents, 
                code_occurence: crate::code_occurence_tufa_common!()
            }
        ));
    }
    Ok(())
}
