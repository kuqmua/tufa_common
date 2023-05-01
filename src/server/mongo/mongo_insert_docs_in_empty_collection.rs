#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertDocsInEmptyCollectionWrapperErrorNamed<'a> {
    ClientWithOptions {
        #[eo_display_foreign_type]
        client_with_options: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CountDocuments {
        #[eo_display_foreign_type]
        count_documents: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NotEmpty {
        #[eo_display_with_serialize_deserialize]
        not_empty: u64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CollectionInsertMany {
        #[eo_display_foreign_type]
        collection_insert_many: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_insert_docs_in_empty_collection(
    client_options: mongodb::options::ClientOptions,
    db_name_handle: &str,
    db_collection_handle: String,
    collection_field_name: String,
    vec_of_values: Vec<String>,
) -> Result<(), Box<MongoInsertDocsInEmptyCollectionWrapperErrorNamed>> {
    match mongodb::Client::with_options(client_options) {
        Err(e) => Err(Box::new(
            MongoInsertDocsInEmptyCollectionWrapperErrorNamed::ClientWithOptions{
                client_with_options: e,
                code_occurence: crate::code_occurence_tufa_common!()
            }
        )),
        Ok(client) => {
            let collection = client
                .database(db_name_handle)
                .collection(&db_collection_handle);
            match collection.count_documents(None, None).await {
                Err(e) => Err(Box::new(
                    MongoInsertDocsInEmptyCollectionWrapperErrorNamed::CountDocuments{
                        count_documents: e,
                        code_occurence: crate::code_occurence_tufa_common!()
                    }
                )),
                Ok(documents_number) => {
                    if documents_number > 0 {
                        Err(Box::new(
                            MongoInsertDocsInEmptyCollectionWrapperErrorNamed::NotEmpty{
                                not_empty: documents_number,
                                code_occurence: crate::code_occurence_tufa_common!()
                            }
                        ))
                    } else {
                        if let Err(e) = collection
                            .insert_many(
                                vec_of_values
                                    .iter()
                                    .map(|value| mongodb::bson::doc! { &collection_field_name: value })
                                    .collect::<Vec<mongodb::bson::Document>>(),
                                None,
                            )
                            .await
                        {
                            return Err(Box::new(
                                    MongoInsertDocsInEmptyCollectionWrapperErrorNamed::CollectionInsertMany{
                                        collection_insert_many: e,
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
