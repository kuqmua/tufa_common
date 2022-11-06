use crate::common::where_was::WhereWas;
use crate::config_mods::config_struct::ConfigStruct;
use crate::lazy_static::config::CONFIG;

#[derive(Debug)]
pub struct WrapConfigChecksError {
    pub source: Box<WrapConfigChecksErrorEnum>,
}

#[derive(Debug)]
pub enum WrapConfigChecksErrorEnum {
    Timezone { source: i32 }, //no where_was for this coz inside using timezone
    ServerIp { source: String, where_was: WhereWas },
    GithubName { source: String, where_was: WhereWas },
    GithubToken { source: String, where_was: WhereWas },
    RedditUserAgent { source: String, where_was: WhereWas },
    RedditClientId { source: String, where_was: WhereWas },
    RedditClientSecret { source: String, where_was: WhereWas },
    RedditUsername { source: String, where_was: WhereWas },
    RedditPassword { source: String, where_was: WhereWas },
    MongoLogin { source: String, where_was: WhereWas },
    MongoPassword { source: String, where_was: WhereWas },
    MongoIp { source: String, where_was: WhereWas },
    LogFileExtension { source: String, where_was: WhereWas },
    PathToProviderLinkPartsFolder { source: String, where_was: WhereWas },
    ProvidersDbCollectionDocumentFieldName { source: String, where_was: WhereWas },
    WarningLogsDirectoryName { source: String, where_was: WhereWas },
    LinksLimitProviderse { source: usize, where_was: WhereWas },
}

impl ConfigStruct {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn wrap_config_checks(self) -> Result<Self, WrapConfigChecksError> {
        //its important to check timezone first coz it will be used later. it must be valid
        if !(-86_400 < self.timezone && self.timezone < 86_400) {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::Timezone {
                    source: self.timezone,
                }),
            });
        }
        //todo: add server_ip server_port checks. and postgres and mongo for validation
        if self.server_ip.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::ServerIp {
                    source: self.server_ip,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        //todo: check ip pattern. check port pattern
        if self.github_name.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::GithubName {
                    source: self.github_name,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        if self.github_token.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::GithubToken {
                    source: self.github_token,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        if self.reddit_user_agent.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::RedditUserAgent {
                    source: self.reddit_user_agent,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        if self.reddit_client_id.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::RedditClientId {
                    source: self.reddit_client_id,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        if self.reddit_client_secret.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::RedditClientSecret {
                    source: self.reddit_client_secret,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        if self.reddit_username.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::RedditUsername {
                    source: self.reddit_username,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        if self.reddit_password.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::RedditPassword {
                    source: self.reddit_password,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        if self.mongo_login.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::MongoLogin {
                    source: self.mongo_login,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        if self.mongo_password.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::MongoPassword {
                    source: self.mongo_password,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        if self.mongo_ip.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::MongoIp {
                    source: self.mongo_ip,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        if self.log_file_extension.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::LogFileExtension {
                    source: self.log_file_extension,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        if self.path_to_provider_link_parts_folder.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::PathToProviderLinkPartsFolder {
                    source: self.path_to_provider_link_parts_folder,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        if self.warning_logs_directory_name.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::WarningLogsDirectoryName {
                    source: self.warning_logs_directory_name,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                }),
            });
        }
        //useless coz usize must be not negative
        // if self.links_limit_providers <= 0 {
        //     println!("fak");
        //     return Err(WrapConfigChecksError {
        //         source: Box::new(WrapConfigChecksErrorEnum::LinksLimitProviderse {
        //             source: self.links_limit_providers,
        //             where_was: WhereWas {
        //                 time: std::time::SystemTime::now()
        //                    .duration_since(std::time::UNIX_EPOCH)
        //                    .expect("cannot convert time to unix_epoch"),
        //                 file: file!(),
        //                 line: line!(),
        //                 column: column!(),
        //             },
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
        // if !ConfigStruct::check_valid_i64_providers_links_limits_for_mongo(&self) {
        //     return Err(WrapConfigChecksError {
        //         source: Box::new(WrapConfigChecksErrorEnum::GithubName {
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
