#[derive(Debug)]
pub enum PostgresGetPoolError {
    Connect(sqlx::Error),
}

pub async fn postgres_get_pool(url: &std::string::String) -> Result<sqlx::Pool<sqlx::Postgres>, Box<PostgresGetPoolError>> {
    match sqlx::postgres::PgPoolOptions::new()
        // .max_connections(10)//
        // .connect_timeout(10000)//std::time::Duration::from_millis(*config.get_postgres_connection_timeout()) //todo add timeout constant or env var
        .connect(url)
        .await
    {
        Err(e) => Err(Box::new(PostgresGetPoolError::Connect(e))),
        Ok(pool) => Ok(pool),
    }
}