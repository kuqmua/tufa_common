#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckDbIsEmptyErrorNamed<'a> {
    MongoDB {
        #[eo_display_foreign_type]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ListCollectionNamesIsNotEmpty {
        #[eo_display_with_serialize_deserialize]
        list_collection_names_len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_check_db_is_empty<'a>(
    client_options: mongodb::options::ClientOptions,
    db_name: &str,
) -> Result<(), Box<MongoCheckDbIsEmptyErrorNamed<'a>>> {
    match mongodb::Client::with_options(client_options) {
        Err(e) => Err(Box::new(
            MongoCheckDbIsEmptyErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: crate::code_occurence_tufa_common!()
            }
        )),
        Ok(client) => match client.database(db_name).list_collection_names(None).await {
            Err(e) => Err(Box::new(
                MongoCheckDbIsEmptyErrorNamed::MongoDB {
                    mongodb: e,
                    code_occurence: crate::code_occurence_tufa_common!()
                }
            )),
            Ok(documents_number) => {
                if !documents_number.is_empty() {
                    return Err(Box::new(
                        MongoCheckDbIsEmptyErrorNamed::ListCollectionNamesIsNotEmpty {
                            list_collection_names_len: documents_number.len(),
                            code_occurence: crate::code_occurence_tufa_common!()
                        }
                    ));
                }
                Ok(())
            }
        },
    }
}
