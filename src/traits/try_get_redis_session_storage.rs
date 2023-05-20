#[derive(Debug)]
pub enum TryGetRedisSessionStorageError {
    Redis(std::string::String),//anyhow::Error
}

pub trait TryGetRedisSessionStorage {
    async fn mongo_try_get_client(&self) -> Result<actix_session::storage::RedisSessionStore, TryGetRedisSessionStorageError>;
}

impl<SelfGeneric> TryGetRedisSessionStorage for SelfGeneric
where
    Self: crate::traits::config_fields::GetRedisUrl,
{
    async fn mongo_try_get_client(&self) -> Result<actix_session::storage::RedisSessionStore, TryGetRedisSessionStorageError> {
        match actix_session::storage::RedisSessionStore::new(self.get_redis_url()).await {
            Ok(redis_session_store) => Ok(redis_session_store),
            Err(e) => {
                return Err(TryGetRedisSessionStorageError::Redis(format!("{e}")));
            }
        }
    }
}