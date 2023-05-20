#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckAvailabilityErrorNamed<'a> {
    ListCollectionNames {
        #[eo_display]
        list_collection_names: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_check_availability<'a>(
    mongo_client: &mongodb::Client,
    db_name: &str,
) -> Result<(), Box<MongoCheckAvailabilityErrorNamed<'a>>> {
    if let Err(e) = mongo_client.database(db_name).list_collection_names(None).await {
        return Err(Box::new(MongoCheckAvailabilityErrorNamed::ListCollectionNames {
            list_collection_names: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }));
    }
    Ok(())
}
