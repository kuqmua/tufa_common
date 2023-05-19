#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoDropCollectionErrorNamed<'a> {
    MongoDB {
        #[eo_display]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_drop_collection<'a>(
    config: &'static impl crate::traits::config_fields::GetMongoClient,
    db_name: &'a str,
    db_collection_name: &'a str,
) -> Result<(), Box<crate::server::mongo::mongo_drop_collection::MongoDropCollectionErrorNamed<'a>>> {
    let collection: mongodb::Collection<mongodb::bson::Document> = config.get_mongo_client().database(db_name).collection(db_collection_name);
    if let Err(e) = collection.drop(None).await {
        return Err(Box::new(
            crate::server::mongo::mongo_drop_collection::MongoDropCollectionErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        ));
    }
    Ok(())
}
