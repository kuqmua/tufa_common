#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum WrapConfigChecksErrorNamed<'a> {
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
    RedditUserAgent {
        #[eo_display_with_serialize_deserialize] 
        reddit_user_agent: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    RedditClientId {
        #[eo_display_with_serialize_deserialize] 
        reddit_client_id: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    RedditClientSecret {
        #[eo_display_with_serialize_deserialize] 
        reddit_client_secret: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    RedditUsername {
        #[eo_display_with_serialize_deserialize] 
        reddit_username: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    RedditPassword {
        #[eo_display_with_serialize_deserialize] 
        reddit_password: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    MongoUrl {
        #[eo_display_with_serialize_deserialize] 
        mongo_url: std::string::String, 
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>, 
    },
    PathToProviderLinkPartsFolder {
        #[eo_display_with_serialize_deserialize] 
        path_to_provider_link_parts_folder: std::string::String, 
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
        if self.reddit_user_agent.is_empty() {
            return Err(Box::new(WrapConfigChecksErrorNamed::RedditUserAgent {
                reddit_user_agent: self.reddit_user_agent,
                code_occurence: crate::code_occurence_tufa_common!()
            }));
        }
        if self.reddit_client_id.is_empty() {
            return Err(Box::new(WrapConfigChecksErrorNamed::RedditClientId {
                reddit_client_id: self.reddit_client_id,
                code_occurence: crate::code_occurence_tufa_common!()
            }));
        }
        if self.reddit_client_secret.is_empty() {
            return Err(Box::new(WrapConfigChecksErrorNamed::RedditClientSecret {
                reddit_client_secret: self.reddit_client_secret,
                code_occurence: crate::code_occurence_tufa_common!()
            }));
        }
        if self.reddit_username.is_empty() {
            return Err(Box::new(WrapConfigChecksErrorNamed::RedditUsername {
                reddit_username: self.reddit_username,
                code_occurence: crate::code_occurence_tufa_common!()
            }));
        }
        if self.reddit_password.is_empty() {
            return Err(Box::new(WrapConfigChecksErrorNamed::RedditPassword {
                reddit_password: self.reddit_password,
                code_occurence: crate::code_occurence_tufa_common!()
            }));
        }
        if self.mongo_url.is_empty() {
            return Err(Box::new(WrapConfigChecksErrorNamed::MongoUrl {
                mongo_url: self.mongo_url,
                code_occurence: crate::code_occurence_tufa_common!()
            }));
        }
        if self.path_to_provider_link_parts_folder.is_empty() {
            return Err(Box::new(WrapConfigChecksErrorNamed::PathToProviderLinkPartsFolder {
                path_to_provider_link_parts_folder: self.path_to_provider_link_parts_folder,
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