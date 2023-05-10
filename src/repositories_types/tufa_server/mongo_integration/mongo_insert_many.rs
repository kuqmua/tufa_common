#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertManyErrorNamed<'a> {
    InsertMany {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        insert_many: std::collections::HashMap<std::string::String, MongoInsertManyErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertManyErrorUnnamed<'a> {
    InsertMany(MongoInsertManyHandleErrorNamed<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertManyHandleErrorNamed<'a> {
    InsertMany {
        #[eo_display_foreign_type]
        insert_many: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_insert_many<'a>(
    providers_json_local_data_hashmap: std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
    db: mongodb::Database,
    config: &'a impl crate::traits::fields::GetMongoProvidersLogsDbCollectionDocumentFieldNameHandle
) -> Result<(), Box<crate::repositories_types::tufa_server::mongo_integration::mongo_insert_many::MongoInsertManyErrorNamed<'a>>> {
    let error_vec_insert_many = futures::future::join_all(
        providers_json_local_data_hashmap.iter().map(
                |(pk, data_vec)|
                async {
                    (
                        pk.to_string(), 
                        db
                        .collection(&{
                            use crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
                            pk.get_db_tag()
                        })
                        .insert_many(
                            data_vec
                            .iter()
                            .map(|data|
                                mongodb::bson::doc! { config.get_mongo_providers_logs_db_collection_document_field_name_handle(): data }
                            )
                            .collect::<Vec<mongodb::bson::Document>>(), 
                            None
                        ).await
                    )
                }
            )
        ).await
        .into_iter()
        .filter_map(|(pk, result)| {
        if let Err(e) = result {
            return Some((
                pk.to_string(), 
                crate::repositories_types::tufa_server::mongo_integration::mongo_insert_many::MongoInsertManyErrorUnnamed::InsertMany(
                    crate::repositories_types::tufa_server::mongo_integration::mongo_insert_many::MongoInsertManyHandleErrorNamed::InsertMany { 
                        insert_many: e, 
                        code_occurence: crate::code_occurence_tufa_common!()  
                    }
                )
            ));
        }
        None
    })
    .collect::<std::collections::HashMap<String, crate::repositories_types::tufa_server::mongo_integration::mongo_insert_many::MongoInsertManyErrorUnnamed>>();
    match error_vec_insert_many.is_empty() {
        true => Ok(()),
        false => Err(Box::new(
            crate::repositories_types::tufa_server::mongo_integration::mongo_insert_many::MongoInsertManyErrorNamed::InsertMany { 
                insert_many: error_vec_insert_many, 
                code_occurence: crate::code_occurence_tufa_common!() 
            },
        ))
    }
}
