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
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    //  {
    //     #[eo_display_with_serialize_deserialize] 
    //     : std::string::String, 
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    // },
    // //
    // server_port: u16,
    // hmac_secret: String,
    // base_url: String,
    // require_ssl: bool,

    // github_name: String,
    // github_token: String,

    // timezone: i32,

    // redis_ip: String,
    // redis_port: u16,

    // mongo_url: String,

    // mongo_connection_timeout: u64,

    // database_url: String,//postgres_url, naming required by sqlx::query::query!

    // postgres_fourth_handle_url_part: String,
    // postgres_fifth_handle_url_part: String,
    // postgres_sixth_handle_url_part: String,

    // postgres_login: String,
    // postgres_password: String,
    // postgres_ip: String, //todo: 4x u8
    // postgres_port: u16,
    // postgres_db: String,
    // postgres_params: String,

    // postgres_connection_timeout: u64,

    // starting_check_link: String, //todo add browser url limit check

    // tracing_type: crate::server::tracing_type::TracingType,
    // source_place_type: crate::common::source_place_type::SourcePlaceType,
    // //
    Timezone {
        #[eo_display_with_serialize_deserialize] 
        timezone: i32, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    }, //incorrect code_occurence maybe
    ServerIp {
        #[eo_display_with_serialize_deserialize] 
        server_ip: std::string::String, 
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
    MongoUrl {
        #[eo_display_with_serialize_deserialize] 
        mongo_url: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    ProvidersDbCollectionDocumentFieldName {
        #[eo_display_with_serialize_deserialize] 
        providers_db_collection_document_field_name: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    LinksLimitProviders {
        #[eo_display_with_serialize_deserialize]
        links_limit_providers: usize, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
}

impl crate::repositories_types::tufa_server::config::config_struct::ConfigStruct {
    pub fn wrap_config_checks<'a>(self) -> Result<Self, Box<WrapConfigChecksErrorNamed<'a>>> {
        //its important to check timezone first coz it will be used later. it must be valid
        if !(-86_400 < self.timezone && self.timezone < 86_400) {
            return Err(Box::new(WrapConfigChecksErrorNamed::Timezone {
                timezone: self.timezone,
                code_occurence: crate::code_occurence_tufa_common!()
            }));
        }
        //todo: check ip pattern. check port pattern
        if self.github_name.is_empty() {
            return Err(Box::new(WrapConfigChecksErrorNamed::GithubName {
                github_name: self.github_name,
                code_occurence: crate::code_occurence_tufa_common!()
            }));
        }
        if self.github_token.is_empty() {
            return Err(Box::new(WrapConfigChecksErrorNamed::GithubToken {
                github_token: self.github_token,
                code_occurence: crate::code_occurence_tufa_common!()
            }));
        }
        if self.mongo_url.is_empty() {
            return Err(Box::new(WrapConfigChecksErrorNamed::MongoUrl {
                mongo_url: self.mongo_url,
                code_occurence: crate::code_occurence_tufa_common!()
            }));
        }
        //useless coz usize must be not negative
        // if self.links_limit_providers <= 0 {
        //     println!("fak");
        //     return Err(WrapConfigChecksError {
        //         source: Box::new(WrapConfigChecksErrorNamed::LinksLimitProviderse {
        //             source: self.links_limit_providers,
        //             code_occurence: crate::code_occurence_tufa_common!()
        //         }),
        //     });
        // }
        /////////////
        //     pub fn east_opt(secs: i32) -> Option<FixedOffset> {
        //     if -86_400 < secs && secs < 86_400 {
        //         Some(FixedOffset { local_minus_utc: secs })
        //     } else {
        //         None
        //     }
        // }
        // pub fn west_opt(secs: i32) -> Option<FixedOffset> {
        //     if -86_400 < secs && secs < 86_400 {
        //         Some(FixedOffset { local_minus_utc: -secs })
        //     } else {
        //         None
        //     }
        // }
        ///////////////
        // if !crate::config_mods::config_struct::ConfigStruct::check_valid_i64_providers_links_limits_for_mongo(&self) {
        //     return Err(WrapConfigChecksError {
        //         source: Box::new(WrapConfigChecksErrorNamed::GithubName {
        //             source: self.github_name,
        //             file: file!(),
        //             line: line!(),
        //             column: column!(),
        //         })
        //     });
        // }
        Ok(self)
    }
}