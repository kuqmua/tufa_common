#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresEstablishConnectionErrorNamed<'a> {
    Connect {
        #[eo_display]
        sqlx_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn postgres_establish_connection<'a>(
    max_connections: u32,
) -> Result<sqlx::Pool<sqlx::Postgres>, Box<crate::repositories_types::tufa_server::postgres_integration::postgres_establish_connection::PostgresEstablishConnectionErrorNamed<'a>>> {
    match sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_timeout(std::time::Duration::from_millis(crate::global_variables::runtime::config::CONFIG.postgres_connection_timeout)) //todo add timeout constant or env var
        .connect(&{
            use crate::traits::get_postgres_url::GetPostgresUrl;
            crate::global_variables::runtime::config::CONFIG.get_postgres_url()
        })
        .await
    {
        Err(e) => Err(Box::new(
            crate::repositories_types::tufa_server::postgres_integration::postgres_establish_connection::PostgresEstablishConnectionErrorNamed::Connect { 
                sqlx_error: e, 
                code_occurence: crate::code_occurence_tufa_common!(), 
            }
        )),
        Ok(pool) => Ok(pool),
    }
}
