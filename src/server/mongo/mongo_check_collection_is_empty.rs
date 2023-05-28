#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckCollectionIsEmptyErrorNamed<'a> {
    MongoDB {
        #[eo_display]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CollectionIsNotEmpty {
        #[eo_display_with_serialize_deserialize]
        collection_documents: u64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_check_collection_is_empty<'a>(
    mongo_client: &mongodb::Client,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), Box<MongoCheckCollectionIsEmptyErrorNamed<'a>>> {
    match mongo_client
        .database(db_name)
        .collection::<mongodb::bson::Document>(db_collection_name)
        .count_documents(None, None)
        .await
    {
        Err(e) => Err(Box::new(MongoCheckCollectionIsEmptyErrorNamed::MongoDB {
            mongodb: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        })),
        Ok(documents_number) => {
            if documents_number > 0 {
                return Err(Box::new(
                    MongoCheckCollectionIsEmptyErrorNamed::CollectionIsNotEmpty {
                        collection_documents: documents_number,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    },
                ));
            }
            Ok(())
        }
    }
}
