pub fn get_connection_pool(configuration: &crate::repositories_types::tufa_server::configuration::DatabaseSettings) -> sqlx::PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub struct ApplicationBaseUrl(pub String);

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ApplicationRunErrorNamed<'a> {
    NewRedisSessionStore {
        #[eo_display_with_serialize_deserialize]
        new_redis_session_store: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    HttpServerListen {
        #[eo_display]
        http_server_listen: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Clone)]
pub struct HmacSecret(pub secrecy::Secret<String>);
