pub trait GetServerPort {
    fn get_server_port(&self) -> &crate::common::user_port::UserPort;
}
pub trait GetHmacSecret {
    fn get_hmac_secret(&self) -> &String;
}
pub trait GetBaseUrl {
    fn get_base_url(&self) -> &String;
}
pub trait GetRequireSsl {
    fn get_require_ssl(&self) -> &bool;
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
pub trait GetMongoConnectionTimeout {
    fn get_mongo_connection_timeout(&self) -> &u64;
}
pub trait GetDatabaseUrl {
    fn get_database_url(&self) -> &String;//postgres database url. required to exists in env
}
pub trait GetPostgresFourthHandleUrlPart {
    fn get_postgres_fourth_handle_url_part(&self) -> &String;
}
pub trait GetPostgresFifthHandleUrlPart {
    fn get_postgres_fifth_handle_url_part(&self) -> &String;
}
pub trait GetPostgresLogin {
    fn get_postgres_login(&self) -> &String;
}
pub trait GetPostgresPassword {
    fn get_postgres_password(&self) -> &String;
}
pub trait GetPostgresIp {
    fn get_postgres_ip(&self) -> &String;
}
pub trait GetPostgresPort {
    fn get_postgres_port(&self) -> &crate::common::user_port::UserPort;
}
pub trait GetPostgresDb {
    fn get_postgres_db(&self) -> &String;
}
pub trait GetPostgresParams {
    fn get_postgres_params(&self) -> &String;
}
pub trait GetPostgresConnectionTimeout {
    fn get_postgres_connection_timeout(&self) -> &u64;
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
