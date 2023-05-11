#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertDocsInEmptyCollectionErrorNamed<'a> {
    MongoDB {
        #[eo_display_foreign_type]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CollectionIsNotEmpty {
        #[eo_display_with_serialize_deserialize]
        collection_is_not_empty: u64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_insert_docs_in_empty_collection<'a>(
    client_options: mongodb::options::ClientOptions,
    db_name_handle: &str,
    db_collection_handle: String,
    collection_field_name: &'a String,
    vec_of_values: Vec<String>,
) -> Result<(), Box<MongoInsertDocsInEmptyCollectionErrorNamed<'a>>> {
    match mongodb::Client::with_options(client_options) {
        Err(e) => Err(Box::new(
            MongoInsertDocsInEmptyCollectionErrorNamed::MongoDB{
                mongodb: e,
                code_occurence: crate::code_occurence_tufa_common!()
            }
        )),
        Ok(client) => {
            let collection = client
                .database(db_name_handle)
                .collection(&db_collection_handle);
            match collection.count_documents(None, None).await {
                Err(e) => Err(Box::new(
                    MongoInsertDocsInEmptyCollectionErrorNamed::MongoDB{
                        mongodb: e,
                        code_occurence: crate::code_occurence_tufa_common!()
                    }
                )),
                Ok(documents_number) => {
                    if documents_number > 0 {
                        Err(Box::new(
                            MongoInsertDocsInEmptyCollectionErrorNamed::CollectionIsNotEmpty{
                                collection_is_not_empty: documents_number,
                                code_occurence: crate::code_occurence_tufa_common!()
                            }
                        ))
                    } else {
                        if let Err(e) = collection
                            .insert_many(
                                vec_of_values
                                    .iter()
                                    .map(|value| mongodb::bson::doc! { collection_field_name: value })
                                    .collect::<Vec<mongodb::bson::Document>>(),
                                None,
                            )
                            .await
                        {
                            return Err(Box::new(
                                    MongoInsertDocsInEmptyCollectionErrorNamed::MongoDB{
                                        mongodb: e,
                                        code_occurence: crate::code_occurence_tufa_common!()
                                    }
                                ));
                        }
                        Ok(())
                    }
                }
            }
        }
    }
}