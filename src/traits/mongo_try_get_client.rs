#[derive(Debug)]
pub enum MongoTryGetClientError {
    Tokio(std::io::Error),
    MongoDB(mongodb::error::Error),
}

pub trait MongoTryGetClient {
    fn mongo_try_get_client(&self) -> mongodb::Client;
}

impl<SelfGeneric> MongoTryGetClient for SelfGeneric
where
    Self: crate::traits::config_fields::GetMongoUrl,
{
    fn mongo_try_get_client(&self) -> Result<mongodb::Client, MongoTryGetClientError> {
        match tokio::runtime::Builder::new_multi_thread()
            .worker_threads(num_cpus::get())
            .enable_all()
            .build()
        {
            Err(e) => Err(MongoTryGetClientError::Tokio(e)),
            Ok(runtime) => runtime.block_on(async move {
                match mongodb::options::ClientOptions::parse(mongo_url).await
                {
                    Ok(mongo_client_options) => match mongodb::Client::with_options(mongo_client_options) {
                        Ok(client) => Ok(client),
                        Err(e) => Err(MongoTryGetClientError::MongoDB(e)),
                    },
                    Err(e) => Err(MongoTryGetClientError::MongoDB(e)),
                }
            })
        }
    }
}