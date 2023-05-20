pub trait GetServerPort {
    fn get_server_port(&self) -> &crate::common::user_port::UserPort;
}
pub trait GetHmacSecret {
    fn get_hmac_secret(&self) -> &secrecy::Secret<std::string::String>;
}
pub trait GetBaseUrl {
    fn get_base_url(&self) -> &String;
}
pub trait GetAccessControlMaxAge {
    fn get_access_control_max_age(&self) -> &usize;
}
pub trait GetAccessControlAllowOrigin {
    fn get_access_control_allow_origin(&self) -> &String;
}
pub trait GetGithubName {
    fn get_github_name(&self) -> &String;
}
pub trait GetGithubToken {
    fn get_github_token(&self) -> &String;
}
pub trait GetTimezone {
    fn get_timezone(&self) -> &chrono::FixedOffset;
}
pub trait GetRedisIp {
    fn get_redis_ip(&self) -> &String;
}
pub trait GetRedisPort {
    fn get_redis_port(&self) -> &crate::common::user_port::UserPort;
}
pub trait GetMongoUrl {
    fn get_mongo_url(&self) -> &String;
}
pub trait GetMongoClient {
    fn get_mongo_client(&self) -> &mongodb::Client;
}
pub trait GetMongoConnectionTimeout {
    fn get_mongo_connection_timeout(&self) -> &u64;
}
pub trait GetPostgresConnectionTimeout {
    fn get_postgres_connection_timeout(&self) -> &u64;
}
pub trait GetPostgresPool {
    fn get_postgres_pool(&self) -> &sqlx::Pool<sqlx::Postgres>;
}
pub trait GetDatabaseUrl {
    fn get_database_url(&self) -> &String;//postgres database url. required to exists in env
}
pub trait GetStartingCheckLink {
    fn get_starting_check_link(&self) -> &String;
}
pub trait GetTracingType {
    fn get_tracing_type(&self) -> &crate::server::tracing_type::TracingType;
}
pub trait GetSourcePlaceType {
    fn get_source_place_type(&self) -> &crate::common::source_place_type::SourcePlaceType;
}
