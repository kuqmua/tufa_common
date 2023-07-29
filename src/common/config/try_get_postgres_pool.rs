#[derive(Debug)]
pub enum TryGetPostgresPoolError {
    Connect(sqlx::Error),
}

impl std::fmt::Display for TryGetPostgresPoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TryGetPostgresPoolError::Connect(e) => write!(f, "{e}"),
        }
    }
}

pub trait TryGetPostgresPool {
    async fn try_get_postgres_pool(
        &self,
    ) -> Result<sqlx::Pool<sqlx::Postgres>, TryGetPostgresPoolError>;
}

impl<SelfGeneric> TryGetPostgresPool for SelfGeneric
where
    Self: crate::common::config::config_fields::GetDatabaseUrl, //meaning postgres. sqlx::query! macro uses DATABASE_URL env var for compile time checks
{
    async fn try_get_postgres_pool(
        &self,
    ) -> Result<sqlx::Pool<sqlx::Postgres>, TryGetPostgresPoolError> {
        println!("trying to create postgres pool...");
        match sqlx::postgres::PgPoolOptions::new()
            .connect(secrecy::ExposeSecret::expose_secret(
                self.get_database_url(),
            ))
            .await
        {
            Err(e) => Err(TryGetPostgresPoolError::Connect(e)),
            Ok(pool) => Ok(pool),
        }
    }
}
