#[derive(Debug)]
pub enum PostgresTryGetPoolError {
    Connect(sqlx::Error),
}

pub trait PostgresTryGetPool {
    async fn postgres_try_get_pool(&self) -> Result<sqlx::Pool<sqlx::Postgres>, PostgresTryGetPoolError>;
}

impl<SelfGeneric> PostgresTryGetPool for SelfGeneric
where
    Self: crate::traits::config_fields::GetDatabaseUrl,//meaning postgres. sqlx::query! macro uses DATABASE_URL env var for compile time checks 
{
    async fn postgres_try_get_pool(&self) -> Result<sqlx::Pool<sqlx::Postgres>, PostgresTryGetPoolError> {
        match 
            sqlx::postgres::PgPoolOptions::new()
            .connect(self.get_database_url())
            .await
        {
            Err(e) => Err(PostgresTryGetPoolError::Connect(e)),
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
//     //         .password({
//     //             use secrecy::ExposeSecret;
//     //             config.get_postgres_password().expose_secret()
//     //         })
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