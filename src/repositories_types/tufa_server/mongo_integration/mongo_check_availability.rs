#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckAvailabilityErrorNamed<'a> {
    ClientWithOptions {
        #[eo_display]
        client_with_options: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ListCollectionNames {
        #[eo_display]
        list_collection_names: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_check_availability<'a>(
    client_options: mongodb::options::ClientOptions,
    db_name: &str,
) -> Result<(), Box<MongoCheckAvailabilityErrorNamed<'a>>> {
    match mongodb::Client::with_options(client_options) {
        Err(e) => Err(Box::new(MongoCheckAvailabilityErrorNamed::ClientWithOptions {
            client_with_options: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        })),
        Ok(client) => {
            if let Err(e) = client.database(db_name).list_collection_names(None).await {
                return Err(Box::new(MongoCheckAvailabilityErrorNamed::ListCollectionNames {
                    list_collection_names: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }));
            }
            Ok(())
        }
    }
}
