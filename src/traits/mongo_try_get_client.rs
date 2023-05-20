#[derive(Debug)]
pub enum MongoTryGetClientError {
    MongoDB(mongodb::error::Error),
}

pub trait MongoTryGetClient {
    async fn mongo_try_get_client(&self) -> Result<mongodb::Client, MongoTryGetClientError>;
}

impl<SelfGeneric> MongoTryGetClient for SelfGeneric
where
    Self: crate::traits::config_fields::GetMongoUrl,
{
    async fn mongo_try_get_client(&self) -> Result<mongodb::Client, MongoTryGetClientError> {
        match mongodb::options::ClientOptions::parse(self.get_mongo_url()).await
        {
            Ok(mongo_client_options) => match mongodb::Client::with_options(mongo_client_options) {
                Ok(client) => Ok(client),
                Err(e) => Err(MongoTryGetClientError::MongoDB(e)),
            },
            Err(e) => Err(MongoTryGetClientError::MongoDB(e)),
        }
    }
}