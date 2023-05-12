#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoDropEmptyDbErrorNamed<'a> {
    MongoDB {
        #[eo_display]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CollectionNamesListIsNotEmpty {
        #[eo_display_with_serialize_deserialize]
        database: &'a str,
        #[eo_display_with_serialize_deserialize]
        list_collection_names_len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_drop_empty_db<'a>(
    mongo_url: &'a str,
    db_name: &'a str,
) -> Result<(), Box<crate::repositories_types::tufa_server::mongo_integration::mongo_drop_empty_db::MongoDropEmptyDbErrorNamed<'a>>> {
    match mongodb::options::ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(
            crate::repositories_types::tufa_server::mongo_integration::mongo_drop_empty_db::MongoDropEmptyDbErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        )),
        Ok(client_options) => match mongodb::Client::with_options(client_options) {
            Err(e) => Err(Box::new(
                crate::repositories_types::tufa_server::mongo_integration::mongo_drop_empty_db::MongoDropEmptyDbErrorNamed::MongoDB {
                    mongodb: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }
            )),
            Ok(client) => {
                let db = client.database(db_name);
                match db.list_collection_names(None).await {
                    Err(e) => Err(Box::new(
                        crate::repositories_types::tufa_server::mongo_integration::mongo_drop_empty_db::MongoDropEmptyDbErrorNamed::MongoDB {
                            mongodb: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }
                    )),
                    Ok(collections_names_list) => {
                        if !collections_names_list.is_empty() {
                            return Err(Box::new(
                                crate::repositories_types::tufa_server::mongo_integration::mongo_drop_empty_db::MongoDropEmptyDbErrorNamed::CollectionNamesListIsNotEmpty {
                                    database: db_name,
                                    list_collection_names_len: collections_names_list.len(),
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                }
                            ));
                        }
                        if let Err(e) = db.drop(None).await {
                            return Err(Box::new(
                                crate::repositories_types::tufa_server::mongo_integration::mongo_drop_empty_db::MongoDropEmptyDbErrorNamed::MongoDB {
                                    mongodb: e,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                }
                            ));
                        }
                        Ok(())
                    }
                }
            }
        },
    }
}
