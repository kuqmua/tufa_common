#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoDropCollectionErrorNamed {
    MongoDB {
        #[eo_display]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

pub async fn mongo_drop_collection<'a>(
    mongo_client: &mongodb::Client,
    db_name: &'a str,
    db_collection_name: &'a str,
) -> Result<(), Box<crate::server::mongo::mongo_drop_collection::MongoDropCollectionErrorNamed>> {
    let collection: mongodb::Collection<mongodb::bson::Document> = mongo_client
        .database(db_name)
        .collection(db_collection_name);
    if let Err(e) = collection.drop(None).await {
        return Err(Box::new(
            crate::server::mongo::mongo_drop_collection::MongoDropCollectionErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            },
        ));
    }
    Ok(())
}
