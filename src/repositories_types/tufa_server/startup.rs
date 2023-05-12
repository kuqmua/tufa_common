pub fn get_connection_pool(configuration: &crate::repositories_types::tufa_server::configuration::DatabaseSettings) -> sqlx::PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub struct ApplicationBaseUrl(pub String);

#[derive(Debug)]
pub enum ApplicationRunErrorEnum {
    NewRedisSessionStore { source: std::string::String },
    HttpServerListen { source: std::io::Error },
}

#[derive(Clone)]
pub struct HmacSecret(pub secrecy::Secret<String>);
