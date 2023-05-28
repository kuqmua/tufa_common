#[derive(Debug, strum_macros::Display)]
pub enum TryGetRedisSessionStorageError {
    Redis(std::string::String), //anyhow::Error
}

pub trait TryGetRedisSessionStorage {
    async fn try_get_redis_session_storage(
        &self,
    ) -> Result<actix_session::storage::RedisSessionStore, TryGetRedisSessionStorageError>;
}

impl<SelfGeneric> TryGetRedisSessionStorage for SelfGeneric
where
    Self: crate::common::config::config_fields::GetRedisUrl,
{
    async fn try_get_redis_session_storage(
        &self,
    ) -> Result<actix_session::storage::RedisSessionStore, TryGetRedisSessionStorageError> {
        match actix_session::storage::RedisSessionStore::new({
            use secrecy::ExposeSecret;
            self.get_redis_url().expose_secret()
        })
        .await
        {
            Ok(redis_session_store) => Ok(redis_session_store),
            Err(e) => {
                return Err(TryGetRedisSessionStorageError::Redis(format!("{e}")));
            }
        }
    }
}
