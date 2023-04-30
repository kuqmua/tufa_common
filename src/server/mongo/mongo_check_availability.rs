#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckAvailabilityError<'a> {
    Mongo {
        #[eo_display_foreign_type]
        error: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_check_availability<'a>(
    client_options: mongodb::options::ClientOptions,
    db_name: &str,
) -> Result<(), Box<MongoCheckAvailabilityError<'a>>> {
    match mongodb::Client::with_options(client_options) {
        Err(e) => Err(Box::new(MongoCheckAvailabilityError::Mongo {
            error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        })),
        Ok(client) => {
            if let Err(e) = client.database(db_name).list_collection_names(None).await {
                return Err(Box::new(MongoCheckAvailabilityError::Mongo {
                    error: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }));
            }
            Ok(())
        }
    }
}
