#[derive(Debug)]
pub enum PostgresGetPoolError {
    Tokio(std::io::Error),
    Connect(sqlx::Error),
}
//todo move it to config logic mod
pub fn postgres_get_pool(url: &std::string::String) -> Result<sqlx::Pool<sqlx::Postgres>, PostgresGetPoolError> {
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime.block_on(async move {
            match sqlx::postgres::PgPoolOptions::new()
                // .max_connections(10)//
                // .connect_timeout(10000)//std::time::Duration::from_millis(*config.get_postgres_connection_timeout()) //todo add timeout constant or env var
                .connect(url)
                .await
            {
                Err(e) => Err(PostgresGetPoolError::Connect(e)),
                Ok(pool) => Ok(pool),
            }
        }),
        Err(e) => Err(PostgresGetPoolError::Tokio(e)),
    }
}