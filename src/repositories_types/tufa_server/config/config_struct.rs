//todo - maybe should exists types of config - config for parsing env file, config builder for checks some logic like its have u8 but its not 42(wrapper under u8)

#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    init_from_env_with_panic_if_failed::InitFromEnvWithPanicIfFailedWithPanicIfFailed,
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFields,
)]
pub struct ConfigStruct {
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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum WrapConfigChecksErrorNamed<'a> {
    ServerPort {
        #[eo_display_with_serialize_deserialize] 
        server_port: u16, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    HmacSecret {
        #[eo_display_with_serialize_deserialize] 
        hmac_secret: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    BaseUrl {
        #[eo_display_with_serialize_deserialize] 
        base_url: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    RequireSsl {
        #[eo_display_with_serialize_deserialize] 
        require_ssl: bool, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    GithubName {
        #[eo_display_with_serialize_deserialize] 
        github_name: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    GithubToken {
        #[eo_display_with_serialize_deserialize] 
        github_token: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    Timezone {
        #[eo_display_with_serialize_deserialize] 
        timezone: i32, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    RedisIp {
        #[eo_display_with_serialize_deserialize] 
        redis_ip: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    RedisPort {
        #[eo_display_with_serialize_deserialize] 
        redis_port: u16, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    MongoUrl {
        #[eo_display_with_serialize_deserialize] 
        mongo_url: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    MongoConnectionTimeout {
        #[eo_display_with_serialize_deserialize] 
        mongo_connection_timeout: u64, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    DatabaseUrl {
        #[eo_display_with_serialize_deserialize] 
        database_url: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    PostgresFourthHandleUrlPart {
        #[eo_display_with_serialize_deserialize] 
        postgres_fourth_handle_url_part: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    PostgresFifthHandleUrlpart {
        #[eo_display_with_serialize_deserialize] 
        postgres_fifth_handle_url_part: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    PostgresSixthHandleUrlPart {
        #[eo_display_with_serialize_deserialize] 
        postgres_sixth_handle_url_part: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    PostgresLogin {
        #[eo_display_with_serialize_deserialize] 
        PostgresLogin: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    PostgresPassword {
        #[eo_display_with_serialize_deserialize] 
        postgres_password: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    PostgresIp {
        #[eo_display_with_serialize_deserialize] 
        postgres_ip: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    PostgresPort {
        #[eo_display_with_serialize_deserialize] 
        postgres_port: u16, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    PostgresDb {
        #[eo_display_with_serialize_deserialize] 
        postgres_db: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    PostgresParams {
        #[eo_display_with_serialize_deserialize] 
        postgres_params: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    PostgresConnectionTimeout {
        #[eo_display_with_serialize_deserialize] 
        postgres_connection_timeout: u64, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    StartingCheckLink {
        #[eo_display_with_serialize_deserialize] 
        starting_check_link: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    TracingType {
        #[eo_display_with_serialize_deserialize] 
        tracing_type: crate::server::tracing_type::TracingType, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    SourcePlaceType {
        #[eo_display_with_serialize_deserialize] 
        source_place_type: crate::common::source_place_type::SourcePlaceType, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    }
}

impl crate::repositories_types::tufa_server::config::config_struct::ConfigStruct {
    pub fn wrap_config_checks<'a>(self) -> Result<Self, Box<WrapConfigChecksErrorNamed<'a>>> {
        //its important to check timezone first coz it will be used later. it must be valid
        // if !(-86_400 < self.timezone && self.timezone < 86_400) {
        //     return Err(Box::new(WrapConfigChecksErrorNamed::Timezone {
        //         timezone: self.timezone,
        //         code_occurence: crate::code_occurence_tufa_common!()
        //     }));
        // }
        // //todo: check ip pattern. check port pattern
        // if self.github_name.is_empty() {
        //     return Err(Box::new(WrapConfigChecksErrorNamed::GithubName {
        //         github_name: self.github_name,
        //         code_occurence: crate::code_occurence_tufa_common!()
        //     }));
        // }
        // if self.github_token.is_empty() {
        //     return Err(Box::new(WrapConfigChecksErrorNamed::GithubToken {
        //         github_token: self.github_token,
        //         code_occurence: crate::code_occurence_tufa_common!()
        //     }));
        // }
        // if self.mongo_url.is_empty() {
        //     return Err(Box::new(WrapConfigChecksErrorNamed::MongoUrl {
        //         mongo_url: self.mongo_url,
        //         code_occurence: crate::code_occurence_tufa_common!()
        //     }));
        // }
        Ok(self)
    }
}