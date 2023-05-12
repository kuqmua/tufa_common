#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoDropCollectionErrorNamed<'a> {
    MongoDB {
        #[eo_display]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_drop_collection<'a>(
    mongo_url: &'a str,
    db_name: &'a str,
    db_collection_name: &'a str,
) -> Result<(), Box<crate::repositories_types::tufa_server::mongo_integration::mongo_drop_collection::MongoDropCollectionErrorNamed<'a>>> {
    match mongodb::options::ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(
            crate::repositories_types::tufa_server::mongo_integration::mongo_drop_collection::MongoDropCollectionErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        )),
        Ok(client_options) => match mongodb::Client::with_options(client_options) {
            Err(e) => Err(Box::new(
                crate::repositories_types::tufa_server::mongo_integration::mongo_drop_collection::MongoDropCollectionErrorNamed::MongoDB {
                    mongodb: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }
            )),
            Ok(client) => {
                let collection: mongodb::Collection<mongodb::bson::Document> =
                    client.database(db_name).collection(db_collection_name);
                if let Err(e) = collection.drop(None).await {
                    return Err(Box::new(
                        crate::repositories_types::tufa_server::mongo_integration::mongo_drop_collection::MongoDropCollectionErrorNamed::MongoDB {
                            mongodb: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }
                    ));
                }
                Ok(())
            }
        },
    }
}
