#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresEstablishConnectionErrorNamed<'a> {
    Connect {
        #[eo_display]
        sqlx_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn postgres_establish_connection<'a, SelfGeneric>(
    max_connections: u32,
    config: &'static (
        impl crate::traits::config_fields::GetPostgresConnectionTimeout
        + crate::traits::get_postgres_url::GetPostgresUrl<SelfGeneric>
        + std::marker::Send 
        + std::marker::Sync
    )
) -> Result<sqlx::Pool<sqlx::Postgres>, Box<crate::server::postgres::postgres_establish_connection::PostgresEstablishConnectionErrorNamed<'a>>> {
    match sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_timeout(std::time::Duration::from_millis(*config.get_postgres_connection_timeout())) //todo add timeout constant or env var
        .connect(&config.get_postgres_url())
        .await
    {
        Err(e) => Err(Box::new(
            crate::server::postgres::postgres_establish_connection::PostgresEstablishConnectionErrorNamed::Connect { 
                sqlx_error: e, 
                code_occurence: crate::code_occurence_tufa_common!(), 
            }
        )),
        Ok(pool) => Ok(pool),
    }
}
