#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoDropEmptyCollectionErrorNamed<'a> {
    MongoDB {
        #[eo_display]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CollectionIsNotEmpty {
        #[eo_display_with_serialize_deserialize]
        collection_name: std::string::String,
        #[eo_display_with_serialize_deserialize]
        collection_len: u64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_drop_empty_collection<'a>(
    mongo_url: String,
    db_name: &'a str,
    db_collection_name: String,
) -> Result<(), Box<crate::repositories_types::tufa_server::mongo_integration::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed<'a>>> {
    match mongodb::options::ClientOptions::parse(&mongo_url).await {
        Err(e) => Err(Box::new(
            crate::repositories_types::tufa_server::mongo_integration::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        )),
        Ok(client_options) => {
            match mongodb::Client::with_options(client_options) {//todo - maybe move mongodb::Client value to global variables?
                Err(e) => Err(Box::new(
                    crate::repositories_types::tufa_server::mongo_integration::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::MongoDB {
                        mongodb: e,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    }
                )),
                Ok(client) => {
                    let collection: mongodb::Collection<mongodb::bson::Document> =
                        client.database(db_name).collection(&db_collection_name);
                    match collection.count_documents(None, None).await {
                    Err(e) => Err(Box::new(
                        crate::repositories_types::tufa_server::mongo_integration::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::MongoDB {
                            mongodb: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }
                    )),
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            Err(Box::new(
                                crate::repositories_types::tufa_server::mongo_integration::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::CollectionIsNotEmpty {
                                    collection_name: db_collection_name,
                                    collection_len: documents_number,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                }
                            ))
                        } else {
                            if let Err(e) = collection.drop(None).await {
                                return Err(Box::new(
                                    crate::repositories_types::tufa_server::mongo_integration::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::MongoDB {
                                        mongodb: e,
                                        code_occurence: crate::code_occurence_tufa_common!(),
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
    }
}
