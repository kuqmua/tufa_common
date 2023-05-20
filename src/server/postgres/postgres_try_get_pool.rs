#[derive(Debug)]
pub enum PostgresTryGetPoolError {
    Tokio(std::io::Error),
    Connect(sqlx::Error),
}
//todo move it to config logic mod
pub fn postgres_try_get_pool(url: &std::string::String) -> Result<sqlx::Pool<sqlx::Postgres>, PostgresTryGetPoolError> {
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime.block_on(async move {
            //
        // let require_ssl_handle = match config_unchecked.require_ssl {
        //         true => sqlx::postgres::PgSslMode::Require,
        //         false => sqlx::postgres::PgSslMode::Prefer,
        // };
            //
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
            match sqlx::postgres::PgPoolOptions::new()
                // .max_connections(10)//todo
                // .connect_timeout(std::time::Duration::from_millis(*connection_timeout))// //todo add timeout constant or env var
                .connect(url)
                .await
            {
                Err(e) => Err(PostgresTryGetPoolError::Connect(e)),
                Ok(pool) => Ok(pool),
            }
        }),
        Err(e) => Err(PostgresTryGetPoolError::Tokio(e)),
    }
}