//todo - maybe should exists types of config - config for parsing env file, config builder for checks some logic like its have u8 but its not 42(wrapper under u8)

#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    init_from_env_with_panic_if_failed::InitFromEnvWithPanicIfFailedWithPanicIfFailed,
)]
pub struct ConfigUnchecked {
    pub server_port: u16,
    pub hmac_secret: String,
    pub base_url: String,
    pub require_ssl: bool,

    pub github_name: String,
    pub github_token: String,

    pub timezone: i32,

    pub redis_ip: String,
    pub redis_port: u16,

    pub mongo_url: String,

    pub mongo_connection_timeout: u64,

    pub database_url: String,//postgres_url, naming required by sqlx::query::query!

    pub postgres_fourth_handle_url_part: String,
    pub postgres_fifth_handle_url_part: String,
    pub postgres_sixth_handle_url_part: String,

    pub postgres_login: String,
    pub postgres_password: String,
    pub postgres_ip: String, //todo: 4x u8
    pub postgres_port: u16,
    pub postgres_db: String,
    pub postgres_params: String,

    pub postgres_connection_timeout: u64,

    pub starting_check_link: String, //todo add browser url limit check

    pub tracing_type: crate::server::tracing_type::TracingType,
    pub source_place_type: crate::common::source_place_type::SourcePlaceType,
}

#[derive(
    Debug,
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFields,
)]
pub struct Config {
    server_port: crate::common::user_port::UserPort,
    hmac_secret: String,
    base_url: String,
    require_ssl: bool,

    github_name: String,
    github_token: String,

    timezone: i32,

    redis_ip: String,
    redis_port: crate::common::user_port::UserPort,

    mongo_url: String,

    mongo_connection_timeout: u64,

    database_url: String,//postgres_url, naming required by sqlx::query::query!

    postgres_fourth_handle_url_part: String,
    postgres_fifth_handle_url_part: String,
    postgres_sixth_handle_url_part: String,

    postgres_login: String,
    postgres_password: String,
    postgres_ip: String, //todo: 4x u8
    postgres_port: crate::common::user_port::UserPort,
    postgres_db: String,
    postgres_params: String,

    postgres_connection_timeout: u64,

    starting_check_link: String, //todo add browser url limit check

    tracing_type: crate::server::tracing_type::TracingType,
    source_place_type: crate::common::source_place_type::SourcePlaceType,
}

impl TryFrom<ConfigUnchecked> for Config {
    type Error = ConfigCheckError;
    fn try_from(config_struct: ConfigUnchecked) -> Result<Self, ConfigCheckError> {
        //its important to check timezone first coz it will be used later. it must be valid
        // if !(-86_400 < self.timezone && self.timezone < 86_400) {
        //     return Err(Box::new(ConfigCheckErrorNamed::Timezone {
        //         timezone: self.timezone,
        //         code_occurence: crate::code_occurence_tufa_common!()
        //     }));
        // }
        let server_port_handle = match crate::common::user_port::UserPort::try_from(config_struct.server_port) {
            Ok(user_port) => user_port,
            Err(e) => {
                return Err(ConfigCheckError::ServerPort(e));
            },
        };
        let hmac_secret_handle = match config_struct.hmac_secret.is_empty() {
            true => {
                return Err(ConfigCheckError::HmacSecret(config_struct.hmac_secret));
            },
            false => config_struct.hmac_secret,
        };
        let base_url_handle = match config_struct.base_url.is_empty() {
            true => {
                return Err(ConfigCheckError::BaseUrl(config_struct.base_url));
            },
            false => config_struct.base_url,
        };
        let require_ssl_handle = config_struct.require_ssl;

        let github_name_handle = match config_struct.github_name.is_empty() {
            true => {
                return Err(ConfigCheckError::GithubName(config_struct.github_name));
            },
            false => config_struct.github_name,
        };
        let github_token_handle = match config_struct.github_token.is_empty() {
            true => {
                return Err(ConfigCheckError::GithubToken(config_struct.github_token));
            },
            false => config_struct.github_token,
        };

        let timezone_handle = config_struct.timezone;

        let redis_ip_handle = match config_struct.redis_ip.is_empty() {
            true => {
                return Err(ConfigCheckError::RedisIp(config_struct.redis_ip));
            },
            false => config_struct.redis_ip,
        };
        let redis_port_handle = match crate::common::user_port::UserPort::try_from(config_struct.redis_port) {
            Ok(user_port) => user_port,
            Err(e) => {
                return Err(ConfigCheckError::RedisPort(e));
            },
        };

        let mongo_url_handle = match config_struct.mongo_url.is_empty() {
            true => {
                return Err(ConfigCheckError::MongoUrl(config_struct.mongo_url));
            },
            false => config_struct.mongo_url,
        };

        let mongo_connection_timeout_handle = config_struct.mongo_connection_timeout;

        let database_url_handle = match config_struct.database_url.is_empty() {
            true => {
                return Err(ConfigCheckError::DatabaseUrl(config_struct.database_url));
            },
            false => config_struct.database_url,
        };//postgres_url = config_struct.; naming required by sqlx::query::query!

        let postgres_fourth_handle_url_part_handle = match config_struct.postgres_fourth_handle_url_part.is_empty() {
            true => {
                return Err(ConfigCheckError::PostgresFourthHandleUrlPart(config_struct.postgres_fourth_handle_url_part));
            },
            false => config_struct.postgres_fourth_handle_url_part,
        };
        let postgres_fifth_handle_url_part_handle = match config_struct.postgres_fifth_handle_url_part.is_empty() {
            true => {
                return Err(ConfigCheckError::PostgresFifthHandleUrlpart(config_struct.postgres_fifth_handle_url_part));
            },
            false => config_struct.postgres_fifth_handle_url_part,
        };
        let postgres_sixth_handle_url_part_handle = config_struct.postgres_sixth_handle_url_part;

        let postgres_login_handle = match config_struct.postgres_login.is_empty() {
            true => {
                return Err(ConfigCheckError::PostgresLogin(config_struct.postgres_login));
            },
            false => config_struct.postgres_login,
        };
        let postgres_password_handle = match config_struct.postgres_password.is_empty() {
            true => {
                return Err(ConfigCheckError::PostgresPassword(config_struct.postgres_password));
            },
            false => config_struct.postgres_password,
        };
        let postgres_ip_handle = match config_struct.postgres_ip.is_empty() {
            true => {
                return Err(ConfigCheckError::PostgresIp(config_struct.postgres_ip));
            },
            false => config_struct.postgres_ip,
        }; //todo_handle: 4x u8
        let postgres_port_handle = match crate::common::user_port::UserPort::try_from(config_struct.postgres_port) {
            Ok(user_port) => user_port,
            Err(e) => {
                return Err(ConfigCheckError::PostgresPort(e));
            },
        };
        let postgres_db_handle = match config_struct.postgres_db.is_empty() {
            true => {
                return Err(ConfigCheckError::PostgresDb(config_struct.postgres_db));
            },
            false => config_struct.postgres_db,
        };
        let postgres_params_handle = config_struct.postgres_params;

        let postgres_connection_timeout_handle = config_struct.postgres_connection_timeout;

        let starting_check_link_handle = match config_struct.starting_check_link.is_empty() {
            true => {
                return Err(ConfigCheckError::StartingCheckLink(config_struct.starting_check_link));
            },
            false => config_struct.starting_check_link,
        }; //todo add browser url limit check

        let tracing_type_handle = config_struct.tracing_type;
        let source_place_type_handle = config_struct.source_place_type;
        Ok(Config {
            server_port: server_port_handle,
            hmac_secret: hmac_secret_handle,
            base_url: base_url_handle,
            require_ssl: require_ssl_handle,

            github_name: github_name_handle,
            github_token: github_token_handle,

            timezone: timezone_handle,

            redis_ip: redis_ip_handle,
            redis_port: redis_port_handle,

            mongo_url: mongo_url_handle,

            mongo_connection_timeout: mongo_connection_timeout_handle,

            database_url: database_url_handle,//postgres_url, naming required by sqlx::query::query!

            postgres_fourth_handle_url_part: postgres_fourth_handle_url_part_handle,
            postgres_fifth_handle_url_part: postgres_fifth_handle_url_part_handle,
            postgres_sixth_handle_url_part: postgres_sixth_handle_url_part_handle,

            postgres_login: postgres_login_handle,
            postgres_password: postgres_password_handle,
            postgres_ip: postgres_ip_handle, //todo: 4x u8
            postgres_port: postgres_port_handle,
            postgres_db: postgres_db_handle,
            postgres_params: postgres_params_handle,

            postgres_connection_timeout: postgres_connection_timeout_handle,

            starting_check_link: starting_check_link_handle, //todo add browser url limit check

            tracing_type: tracing_type_handle,
            source_place_type: source_place_type_handle,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigCheckError {
    ServerPort(crate::common::user_port::UserPortTryFromStringError),
    HmacSecret(std::string::String),
    BaseUrl(std::string::String),
    RequireSsl(bool),
    GithubName(std::string::String),
    GithubToken(std::string::String),
    Timezone(i32),
    RedisIp(std::string::String),
    RedisPort(crate::common::user_port::UserPortTryFromStringError),
    MongoUrl(std::string::String),
    MongoConnectionTimeout(u64),
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

impl std::fmt::Display for ConfigCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigCheckError::ServerPort(i) => write!(f, "{i}"),
            ConfigCheckError::HmacSecret(i) => write!(f, "{i}"),
            ConfigCheckError::BaseUrl(i) => write!(f, "{i}"),
            ConfigCheckError::RequireSsl(i) => write!(f, "{i}"),
            ConfigCheckError::GithubName(i) => write!(f, "{i}"),
            ConfigCheckError::GithubToken(i) => write!(f, "{i}"),
            ConfigCheckError::Timezone(i) => write!(f, "{i}"),
            ConfigCheckError::RedisIp(i) => write!(f, "{i}"),
            ConfigCheckError::RedisPort(i) => write!(f, "{i}"),
            ConfigCheckError::MongoUrl(i) => write!(f, "{i}"),
            ConfigCheckError::MongoConnectionTimeout(i) => write!(f, "{i}"),
            ConfigCheckError::DatabaseUrl(i) => write!(f, "{i}"),
            ConfigCheckError::PostgresFourthHandleUrlPart(i) => write!(f, "{i}"),
            ConfigCheckError::PostgresFifthHandleUrlpart(i) => write!(f, "{i}"),
            ConfigCheckError::PostgresSixthHandleUrlPart(i) => write!(f, "{i}"),
            ConfigCheckError::PostgresLogin(i) => write!(f, "{i}"),
            ConfigCheckError::PostgresPassword(i) => write!(f, "{i}"),
            ConfigCheckError::PostgresIp(i) => write!(f, "{i}"),
            ConfigCheckError::PostgresPort(i) => write!(f, "{i}"),
            ConfigCheckError::PostgresDb(i) => write!(f, "{i}"),
            ConfigCheckError::PostgresParams(i) => write!(f, "{i}"),
            ConfigCheckError::PostgresConnectionTimeout(i) => write!(f, "{i}"),
            ConfigCheckError::StartingCheckLink(i) => write!(f, "{i}"),
            ConfigCheckError::TracingType(i) => write!(f, "{i}"),
            ConfigCheckError::SourcePlaceType(i) => write!(f, "{i}"),
        }
    }
}