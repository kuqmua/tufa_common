#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    init_from_env_with_panic_if_failed::InitFromEnvWithPanicIfFailedWithPanicIfFailed,
)]
pub struct ConfigUnchecked {
    server_port: u16,
    hmac_secret: String,
    base_url: String,
    access_control_max_age: usize,
    access_control_allow_origin: String,

    github_name: String,
    github_token: String,

    timezone: i32,//for some reason chrono::FixedOffset::east_opt uses i32 but i16 is enough 

    redis_ip: String,
    redis_port: u16,

    mongo_url: String,

    mongo_connection_timeout: u64,

    database_url: String,//postgres_url, naming required by sqlx::query::query!

    postgres_connection_timeout: u64,

    starting_check_link: String, //todo add browser url limit check

    tracing_type: crate::server::tracing_type::TracingType,
    source_place_type: crate::common::source_place_type::SourcePlaceType,
}

#[derive(
    Debug,
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFields,//todo - add 2 attributes - for reference\borrow(&) and for value(move)
)]
pub struct Config {
    server_port: crate::common::user_port::UserPort,
    hmac_secret: secrecy::Secret<std::string::String>,
    base_url: String,
    access_control_max_age: usize,
    access_control_allow_origin: String,

    github_name: String,
    github_token: String,

    timezone: chrono::FixedOffset,

    redis_ip: String,
    redis_port: crate::common::user_port::UserPort,

    mongo_url: String,
    mongo_client: mongodb::Client,

    mongo_connection_timeout: u64,

    postgres_pool: sqlx::Pool<sqlx::Postgres>,
    database_url: String,//postgres_url, naming required by sqlx::query::query!

    postgres_connection_timeout: u64,

    starting_check_link: String, //todo add browser url limit check

    tracing_type: crate::server::tracing_type::TracingType,
    source_place_type: crate::common::source_place_type::SourcePlaceType,
}

impl TryFrom<ConfigUnchecked> for Config {
    type Error = ConfigCheckError;
    fn try_from(config_unchecked: ConfigUnchecked) -> Result<Self, ConfigCheckError> {
        let server_port_handle = match crate::common::user_port::UserPort::try_from(config_unchecked.server_port) {
            Ok(user_port) => user_port,
            Err(e) => {
                return Err(ConfigCheckError::ServerPort(e));
            },
        };
        let hmac_secret_handle = match config_unchecked.hmac_secret.is_empty() {
            true => {
                return Err(ConfigCheckError::HmacSecret(config_unchecked.hmac_secret));
            },
            false => secrecy::Secret::new(config_unchecked.hmac_secret),
        };
        let base_url_handle = match config_unchecked.base_url.is_empty() {
            true => {
                return Err(ConfigCheckError::BaseUrl(config_unchecked.base_url));
            },
            false => config_unchecked.base_url,
        };
        let access_control_max_age_handle = config_unchecked.access_control_max_age;
        let access_control_allow_origin_handle = match config_unchecked.access_control_allow_origin.is_empty() {
            true => {
                return Err(ConfigCheckError::AccessControlAllowOrigin(config_unchecked.access_control_allow_origin));//todo - maybe add check if its uri\url
            },
            false => config_unchecked.access_control_allow_origin,
        };

        let github_name_handle = match config_unchecked.github_name.is_empty() {
            true => {
                return Err(ConfigCheckError::GithubName(config_unchecked.github_name));
            },
            false => config_unchecked.github_name,
        };
        let github_token_handle = match config_unchecked.github_token.is_empty() {
            true => {
                return Err(ConfigCheckError::GithubToken(config_unchecked.github_token));
            },
            false => config_unchecked.github_token,
        };

        let timezone_handle = match chrono::FixedOffset::east_opt(config_unchecked.timezone) {
            Some(fixed_offset) => fixed_offset,
            None => {
                return Err(ConfigCheckError::Timezone(config_unchecked.timezone));
            },
        };

        let redis_ip_handle = match config_unchecked.redis_ip.is_empty() {
            true => {
                return Err(ConfigCheckError::RedisIp(config_unchecked.redis_ip));
            },
            false => config_unchecked.redis_ip,
        };
        let redis_port_handle = match crate::common::user_port::UserPort::try_from(config_unchecked.redis_port) {
            Ok(user_port) => user_port,
            Err(e) => {
                return Err(ConfigCheckError::RedisPort(e));
            },
        };

        let mongo_url_handle = match config_unchecked.mongo_url.is_empty() {
            true => {
                return Err(ConfigCheckError::MongoUrl(config_unchecked.mongo_url));
            },
            false => config_unchecked.mongo_url,
        };
        let mongo_client_handle = match crate::server::mongo::mongo_try_get_client::mongo_try_get_client(&mongo_url_handle) {
            Ok(mongo_client) => mongo_client,
            Err(e) => {
                return Err(ConfigCheckError::MongoClient(e));
            },
        };

        let mongo_connection_timeout_handle = config_unchecked.mongo_connection_timeout;

        let postgres_connection_timeout_handle = config_unchecked.postgres_connection_timeout;
        let postgres_pool_handle = match crate::server::postgres::postgres_get_pool::postgres_get_pool(
            &config_unchecked.postgres_connection_timeout,
            &config_unchecked.database_url
        ){
            Ok(pool) => pool,
            Err(e) => {
                return Err(ConfigCheckError::PostgresPool(e));
            },
        };
        let database_url_handle = match config_unchecked.database_url.is_empty() {
            true => {
                return Err(ConfigCheckError::DatabaseUrl(config_unchecked.database_url));
            },
            false => config_unchecked.database_url,
        };//postgres_url = config_unchecked.; naming required by sqlx::query::query!

        let starting_check_link_handle = match config_unchecked.starting_check_link.is_empty() {
            true => {
                return Err(ConfigCheckError::StartingCheckLink(config_unchecked.starting_check_link));
            },
            false => config_unchecked.starting_check_link,
        }; //todo add browser url limit check

        let tracing_type_handle = config_unchecked.tracing_type;
        let source_place_type_handle = config_unchecked.source_place_type;
        Ok(Self {
            server_port: server_port_handle,
            hmac_secret: hmac_secret_handle,
            base_url: base_url_handle,
            access_control_max_age: access_control_max_age_handle,
            access_control_allow_origin: access_control_allow_origin_handle,

            github_name: github_name_handle,
            github_token: github_token_handle,

            timezone: timezone_handle,

            redis_ip: redis_ip_handle,
            redis_port: redis_port_handle,

            mongo_url: mongo_url_handle,
            mongo_client: mongo_client_handle,

            mongo_connection_timeout: mongo_connection_timeout_handle,

            postgres_pool: postgres_pool_handle,
            database_url: database_url_handle,//postgres_url, naming required by sqlx::query::query!

            postgres_connection_timeout: postgres_connection_timeout_handle,

            starting_check_link: starting_check_link_handle, //todo add browser url limit check

            tracing_type: tracing_type_handle,
            source_place_type: source_place_type_handle,
        })
    }
}

#[derive(Debug, thiserror::Error, strum_macros::Display)]
pub enum ConfigCheckError {
    //TODO todo for empty string cases - why need to store empty string? remove it
    ServerPort(crate::common::user_port::UserPortTryFromStringError),
    HmacSecret(std::string::String),
    BaseUrl(std::string::String),
    RequireSsl(bool),
    AccessControlAllowOrigin(std::string::String),
    GithubName(std::string::String),
    GithubToken(std::string::String),
    Timezone(i32),
    RedisIp(std::string::String),
    RedisPort(crate::common::user_port::UserPortTryFromStringError),
    MongoUrl(std::string::String),
    MongoClient(crate::server::mongo::mongo_try_get_client::MongoTryGetClientError),
    MongoConnectionTimeout(u64),
    PostgresPool(crate::server::postgres::postgres_get_pool::PostgresGetPoolError),
    DatabaseUrl(std::string::String),
    PostgresFourthHandleUrlPart(std::string::String),
    PostgresFifthHandleUrlpart(std::string::String),
    PostgresSixthHandleUrlPart(std::string::String),
    PostgresLogin(std::string::String),
    PostgresPassword(std::string::String),
    PostgresIp(std::string::String),
    PostgresPort(crate::common::user_port::UserPortTryFromStringError),
    PostgresDb(std::string::String),
    PostgresParams(std::string::String),
    PostgresConnectionTimeout(u64),
    StartingCheckLink(std::string::String),
    TracingType(crate::server::tracing_type::TracingType),
    SourcePlaceType(crate::common::source_place_type::SourcePlaceType)
}