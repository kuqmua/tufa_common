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

// let require_ssl_handle = match config_unchecked.require_ssl {
//         true => sqlx::postgres::PgSslMode::Require,
//         false => sqlx::postgres::PgSslMode::Prefer,
// };
// let pool = sqlx::postgres::PgPoolOptions::new()
//     .max_connections(10)//todo
//     // .min_connections(min)
//     // .max_lifetime(lifetime)
//     .connect_timeout(std::time::Duration::from_secs(10))
//     //
//     //
//     // .connect_lazy_with({
//     //     //
//     //     let mut options = {
//     //         sqlx::postgres::PgConnectOptions::new()
//     //         .host(&config.get_postgres_ip())
//     //         .username(&config.get_postgres_login())
//     //         .password(secrecy::ExposeSecret::expose_secret(config.get_postgres_password()))
//     //         .port(*config.get_postgres_port().port())
//     //         .ssl_mode(*config.get_require_ssl())
//     //     }.database(&config.get_postgres_db());
//     //     {
//     //         use sqlx::ConnectOptions;
//     //         options.log_statements(tracing::log::LevelFilter::Trace)
//     //     };
//     //     options
//     //     //
//     // })
//     .connect(&config.get_database_url())
//     .await
// .expect("error");
