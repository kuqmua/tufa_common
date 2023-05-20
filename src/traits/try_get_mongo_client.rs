#[derive(Debug)]
pub enum TryGetMongoClientError {
    MongoDB(mongodb::error::Error),
}

pub trait TryGetMongoClient {
    async fn try_get_mongo_client(&self) -> Result<mongodb::Client, TryGetMongoClientError>;
}

impl<SelfGeneric> TryGetMongoClient for SelfGeneric
where
    Self: crate::traits::config_fields::GetMongoUrl,
{
    async fn try_get_mongo_client(&self) -> Result<mongodb::Client, TryGetMongoClientError> {
        match mongodb::options::ClientOptions::parse(self.get_mongo_url()).await
        {
            Ok(mongo_client_options) => match mongodb::Client::with_options(mongo_client_options) {
                Ok(client) => Ok(client),
                Err(e) => Err(TryGetMongoClientError::MongoDB(e)),
            },
            Err(e) => Err(TryGetMongoClientError::MongoDB(e)),
        }
    }
}