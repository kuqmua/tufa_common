#[derive(Debug)]
pub enum RedisTryGetSessionStorageError {
    Tokio(std::io::Error),
    Redis(std::string::String),//anyhow::Error
}
//this function must be use only in config logic. maybe create some config mod and move it there later
pub fn redis_try_get_session_storage(redis_url: &std::string::String) -> Result<actix_session::storage::RedisSessionStore, RedisTryGetSessionStorageError> {
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Err(e) => Err(RedisTryGetSessionStorageError::Tokio(e)),
        Ok(runtime) => runtime.block_on(async move {
            match actix_session::storage::RedisSessionStore::new(redis_url).await {
                Ok(redis_session_store) => Ok(redis_session_store),
                Err(e) => {
                    return Err(RedisTryGetSessionStorageError::Redis(format!("{e}")));
                }
            }
        })
    }
}