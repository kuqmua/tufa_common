pub trait GetServerPort {
    fn get_server_port(&self) -> &u16;
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
pub trait GetRedditUserAgent {
    fn get_reddit_user_agent(&self) -> &String;
}
pub trait GetRedditClientId {
    fn get_reddit_client_id(&self) -> &String;
}
pub trait GetRedditClientSecret {
    fn get_reddit_client_secret(&self) -> &String;
}
pub trait GetRedditUsername {
    fn get_reddit_username(&self) -> &String;
}
pub trait GetRedditPassword {
    fn get_reddit_password(&self) -> &String;
}
pub trait GetProvidersLinkPartsSource {
    fn get_providers_link_parts_source(&self) -> &crate::server::resource::Resource;
}
pub trait GetTimezone {
    fn get_timezone(&self) -> &i32;
}
pub trait GetRedisIp {
    fn get_redis_ip(&self) -> &String;
}
pub trait GetRedisPort {
    fn get_redis_port(&self) -> &u16;
}
pub trait GetMongoUrl {
    fn get_mongo_url(&self) -> &String;
}
pub trait GetMongoConnectionTimeout {
    fn get_mongo_connection_timeout(&self) -> &u64;
}
pub trait GetMongoProvidersLogsDbName {
    fn get_mongo_providers_logs_db_name(&self) -> &String;
}
pub trait GetMongoProvidersLogsDbCollectionDocumentFieldNameHandle {
    fn get_mongo_providers_logs_db_collection_document_field_name_handle(&self) -> &String;
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
pub trait GetPostgresSixthHandleUrlPart {
    fn get_postgres_sixth_handle_url_part(&self) -> &String;
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
    fn get_postgres_port(&self) -> &u16;
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
