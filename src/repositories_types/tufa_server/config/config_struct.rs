#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    init_from_env::InitFromEnv,
)]
pub struct ConfigUnchecked {
    server_port: u16,
    hmac_secret: std::string::String,
    base_url: std::string::String,
    access_control_max_age: usize,
    access_control_allow_origin: std::string::String,

    github_name: std::string::String,
    github_token: std::string::String,

    timezone: i32,//for some reason chrono::FixedOffset::east_opt uses i32 but i16 is enough 

    redis_url: std::string::String,

    mongo_url: std::string::String,

    database_url: std::string::String,//postgres_url, naming required by sqlx::query::query!

    starting_check_link: std::string::String, //todo add browser url limit check

    tracing_type: crate::server::tracing_type::TracingType,
    source_place_type: crate::common::source_place_type::SourcePlaceType,
}

#[derive(
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFields,//todo - add 2 attributes - for reference\borrow(&) and for value(move)
)]
pub struct Config {
    server_port: crate::common::user_port::UserPort,
    hmac_secret: secrecy::Secret<std::string::String>,
    base_url: std::string::String,
    access_control_max_age: usize,
    access_control_allow_origin: std::string::String,

    github_name: std::string::String,
    github_token: std::string::String,

    timezone: chrono::FixedOffset,

    redis_url: secrecy::Secret<std::string::String>,

    mongo_url: secrecy::Secret<std::string::String>,

    database_url: secrecy::Secret<std::string::String>,//postgres_url, naming required by sqlx::query::query!

    starting_check_link: std::string::String, //todo add browser url limit check

    tracing_type: crate::server::tracing_type::TracingType,
    source_place_type: crate::common::source_place_type::SourcePlaceType,
}

impl TryFrom<ConfigUnchecked> for Config {
    type Error = ConfigCheckError;
    fn try_from(config_unchecked: ConfigUnchecked) -> Result<Self, ConfigCheckError> {
        let server_port = match crate::common::user_port::UserPort::try_from(config_unchecked.server_port) {
            Ok(user_port) => user_port,
            Err(e) => {
                return Err(ConfigCheckError::ServerPort(e));
            },
        };
        let hmac_secret = match config_unchecked.hmac_secret.is_empty() {
            true => {
                return Err(ConfigCheckError::HmacSecret(config_unchecked.hmac_secret));
            },
            false => secrecy::Secret::new(config_unchecked.hmac_secret),
        };
        let base_url = match config_unchecked.base_url.is_empty() {
            true => {
                return Err(ConfigCheckError::BaseUrl(config_unchecked.base_url));
            },
            false => config_unchecked.base_url,
        };
        let access_control_max_age = config_unchecked.access_control_max_age;
        let access_control_allow_origin = match config_unchecked.access_control_allow_origin.is_empty() {
            true => {
                return Err(ConfigCheckError::AccessControlAllowOrigin(config_unchecked.access_control_allow_origin));//todo - maybe add check if its uri\url
            },
            false => config_unchecked.access_control_allow_origin,
        };

        let github_name = match config_unchecked.github_name.is_empty() {
            true => {
                return Err(ConfigCheckError::GithubName(config_unchecked.github_name));
            },
            false => config_unchecked.github_name,
        };
        let github_token = match config_unchecked.github_token.is_empty() {
            true => {
                return Err(ConfigCheckError::GithubToken(config_unchecked.github_token));
            },
            false => config_unchecked.github_token,
        };

        let timezone = match chrono::FixedOffset::east_opt(config_unchecked.timezone) {
            Some(fixed_offset) => fixed_offset,
            None => {
                return Err(ConfigCheckError::Timezone(config_unchecked.timezone));
            },
        };

        let redis_url = match config_unchecked.redis_url.is_empty() {
            true => {
                return Err(ConfigCheckError::RedisUrl(config_unchecked.redis_url));
            },
            false => secrecy::Secret::new(config_unchecked.redis_url),
        };

        let mongo_url = match config_unchecked.mongo_url.is_empty() {
            true => {
                return Err(ConfigCheckError::MongoUrl(config_unchecked.mongo_url));
            },
            false => secrecy::Secret::new(config_unchecked.mongo_url),
        };

        let database_url = match config_unchecked.database_url.is_empty() {
            true => {
                return Err(ConfigCheckError::DatabaseUrl(config_unchecked.database_url));
            },
            false => secrecy::Secret::new(config_unchecked.database_url),
        };//postgres_url = config_unchecked.; naming required by sqlx::query::query!

        let starting_check_link = match config_unchecked.starting_check_link.is_empty() {
            true => {
                return Err(ConfigCheckError::StartingCheckLink(config_unchecked.starting_check_link));
            },
            false => config_unchecked.starting_check_link,
        }; //todo add browser url limit check

        let tracing_type = config_unchecked.tracing_type;
        let source_place_type = config_unchecked.source_place_type;
        Ok(Self {
            server_port,
            hmac_secret,
            base_url,
            access_control_max_age,
            access_control_allow_origin,

            github_name,
            github_token,

            timezone,

            redis_url,

            mongo_url,

            database_url,//postgres_url, naming required by sqlx::query::query!

            starting_check_link, //todo add browser url limit check

            tracing_type,
            source_place_type,
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
    RedisUrl(std::string::String),
    MongoUrl(std::string::String),
    DatabaseUrl(std::string::String),
    StartingCheckLink(std::string::String),
    TracingType(crate::server::tracing_type::TracingType),
    SourcePlaceType(crate::common::source_place_type::SourcePlaceType)
}