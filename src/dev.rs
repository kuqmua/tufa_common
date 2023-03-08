use crate::traits::display_foreign_type;

pub fn dev() {
    if let Err(e) = six() {
        //todo - this actually must be a config struct function, not an error - config.error_log(e)
        println!("{}", *e);
        use crate::traits::error_logs_logic::error_log::ErrorLog;
        e.error_log(once_cell::sync::Lazy::force(
            &crate::global_variables::runtime::config::CONFIG,
        ));
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)] //, error_occurence::ImplErrorOccurence
pub enum SixError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::vec::Vec<SixErrorEnum<'a>>,
        // TODO inner_errors: std::vec::Vec<String>,
        // TODO inner_errors: std::collections::HashMap<String, String>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize)] //, error_occurence::ImplErrorOccurence
pub enum SixErrorEnum<'a> {
    Seven(SevenError<'a>),
    Eight(EightError<'a>),
    // //todo #[simple_display] and #[display_foreign_type] support
    // // #[origin]
    Another(String),
    //todo #[simple_display] and #[display_foreign_type] support
    // #[origin]
    AnotherVec(Vec<String>),
    //todo #[simple_display] and #[display_foreign_type] support
    // #[origin]
    AnotherHashmap(std::collections::HashMap<String, String>),
}

#[derive(Debug, thiserror::Error, serde::Serialize)] //, error_occurence::ImplErrorOccurence
pub enum SevenError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize)] //, error_occurence::ImplErrorOccurence
pub enum EightError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub fn six<'a>() -> Result<(), Box<SixError<'a>>> {
    let thread_join_handle = std::thread::spawn(move || eight());
    let res = thread_join_handle.join().unwrap();
    match (seven(), res) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let ss = String::from("kekw");
            let yy = vec![String::from("vec1")];
            let hh =
                std::collections::HashMap::from([(String::from("key"), String::from("value"))]);
            let f = SixError::Something {
                inner_errors: vec![
                    SixErrorEnum::Seven(*seven_error),
                    SixErrorEnum::Eight(*eight_error),
                    // SixErrorEnum::Another(ss),
                    // SixErrorEnum::AnotherVec(yy),
                    // SixErrorEnum::AnotherHashmap(hh),
                ],
                code_occurence: crate::code_occurence_tufa_common!(),
            };
            return Err(Box::new(f));
        }
    }
}

pub fn seven<'a>() -> Result<(), Box<SevenError<'a>>> {
    return Err(Box::new(SevenError::Something {
        error: String::from("error_eight"),
        code_occurence: crate::code_occurence_tufa_common!(),
    }));
}

pub fn eight<'a>() -> Result<(), Box<EightError<'a>>> {
    let f = EightError::Something {
        error: String::from("error_eight"),
        code_occurence: crate::code_occurence_tufa_common!(),
    };
    // use crate::traits::error_logs_logic::error_log::ErrorLog;
    // f.error_log(once_cell::sync::Lazy::force(
    //     &crate::global_variables::runtime::config::CONFIG,
    // ));
    return Err(Box::new(f));
}

impl<'a> std::fmt::Display for SixError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
        write!(f, "{}", self.to_string_without_config())
    }
}
impl<'a> std::fmt::Display for SixErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
        write!(f, "{}", self.to_string_without_config_with_deserialize())
    }
}
impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
        'a,
        ConfigGeneric,
    > for SixError<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        match self {
            SixError::Something {
                inner_errors,
                code_occurence: _unused_second_argument,
            } => {
                use crate::traits::error_logs_logic::few_to_string_with_config::FewToStringWithConfig;
                inner_errors.few_to_string_with_config(config)
            }
        }
    }
}
impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
        'a,
    > for SixError<'a>
{
    fn source_to_string_without_config(&self) -> String {
        match self {
            SixError::Something {
                inner_errors,
                code_occurence: _unused_second_argument,
            } => {
                use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
                inner_errors.few_to_string_without_config()
            }
        }
    }
}
impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
    for SixError<'a>
{
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
        match self {
            SixError::Something {
                inner_errors: _unused_first_argument,
                code_occurence,
            } => code_occurence,
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum SixErrorWithDeserialize<'a> {
    Something {
        #[serde(borrow)]
        inner_errors: std::vec::Vec<SixErrorEnumWithDeserialize<'a>>,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
    },
}
impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
        'a,
    > for SixErrorWithDeserialize<'a>
{
    fn source_to_string_without_config(&self) -> String {
        match self {
            SixErrorWithDeserialize::Something {
                inner_errors,
                code_occurence: _unused_second_argument,
            } => {
                use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigWithDeserialize;
                inner_errors.few_to_string_without_config_with_deserialize()
            }
        }
    }
}
impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
    for SixErrorWithDeserialize<'a>
{
    fn get_code_occurence_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
        match self {
            SixErrorWithDeserialize::Something {
                inner_errors: _unused_first_argument,
                code_occurence,
            } => code_occurence,
        }
    }
}
impl<'a> SixError<'a> {
    pub fn into_serialize_deserialize_version(self) -> SixErrorWithDeserialize<'a> {
        match self {
            SixError::Something {
                inner_errors,
                code_occurence,
            } => SixErrorWithDeserialize::Something {
                inner_errors: inner_errors
                    .into_iter()
                    .map(|e| e.into_serialize_deserialize_version())
                    .collect(),
                code_occurence: code_occurence.into_serialize_deserialize_version(),
            },
        }
    }
}
impl<'a> std::fmt::Display for SixErrorEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
        write!(f, "{}", self.to_string_without_config())
    }
}
impl<'a> std::fmt::Display for SixErrorEnumWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
        write!(f, "{}", self.to_string_without_config_with_deserialize())
    }
}
impl < 'a, ConfigGeneric > crate :: traits :: error_logs_logic ::
to_string_with_config :: ToStringWithConfigForSourceToStringWithConfig < 'a,
ConfigGeneric, > for SixErrorEnum < 'a > where ConfigGeneric : crate :: traits
:: fields :: GetSourcePlaceType + crate :: traits :: fields :: GetTimezone +
crate :: traits :: get_server_address :: GetServerAddress,
{
    fn
    to_string_with_config_for_source_to_string_with_config(& self, config : &
    ConfigGeneric) -> String
    {
        match self
        {
            SixErrorEnum :: Seven(i) =>
            {
                i.to_string_with_config_for_source_to_string_with_config(config)
            }, SixErrorEnum :: Eight(i) =>
            {
                i.to_string_with_config_for_source_to_string_with_config(config)
            },
            SixErrorEnum :: Another(i) => i.to_string(),
            SixErrorEnum::AnotherVec(i) => {
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element = element.lines().fold(String::from(""), |mut acc, line| {
                        acc.push_str(&format!(" {}\n", line));
                        acc
                    });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            },
            SixErrorEnum::AnotherHashmap(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value.lines().fold(String::from(""), |mut accc, line| {
                        accc.push_str(&format!(" {}\n", line));
                        accc
                    });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            },
        }
    }
}
impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
    for SixErrorEnum<'a>
{
    fn to_string_without_config(&self) -> String {
        match self {
            SixErrorEnum::Seven(i) => i.to_string_without_config(),
            SixErrorEnum::Eight(i) => i.to_string_without_config(),
            SixErrorEnum::Another(i) => i.to_string(),
            SixErrorEnum::AnotherVec(i) => {
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element =
                        element.lines().fold(String::from(""), |mut acc, line| {
                            acc.push_str(&format!(" {}\n", line));
                            acc
                        });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            }
            SixErrorEnum::AnotherHashmap(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value =
                        value.lines().fold(String::from(""), |mut accc, line| {
                            accc.push_str(&format!(" {}\n", line));
                            accc
                        });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum SixErrorEnumWithDeserialize<'a> {
    #[serde(borrow)]
    Seven(SevenErrorWithDeserialize<'a>),
    #[serde(borrow)]
    Eight(EightErrorWithDeserialize<'a>),
    Another(String),
    AnotherVec(Vec<String>),
    AnotherHashmap(std::collections::HashMap<String, String>),
}
impl<'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
        'a,
    > for SixErrorEnumWithDeserialize<'a>
{
    fn to_string_without_config_with_deserialize(&self) -> String {
        match self {
            SixErrorEnumWithDeserialize::Seven(i) => i.to_string_without_config_with_deserialize(),
            SixErrorEnumWithDeserialize::Eight(i) => i.to_string_without_config_with_deserialize(),
            SixErrorEnumWithDeserialize::Another(i) => i.to_string(), //todo or to display_foreign_type()
            SixErrorEnumWithDeserialize::AnotherVec(i) => {
                //todo or to display_foreign_type()
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element =
                        element.lines().fold(String::from(""), |mut acc, line| {
                            acc.push_str(&format!(" {}\n", line));
                            acc
                        });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            }
            SixErrorEnumWithDeserialize::AnotherHashmap(i) => {
                //todo or to display_foreign_type()
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value =
                        value.lines().fold(String::from(""), |mut accc, line| {
                            accc.push_str(&format!(" {}\n", line));
                            accc
                        });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            }
        }
    }
}
impl<'a> SixErrorEnum<'a> {
    pub fn into_serialize_deserialize_version(self) -> SixErrorEnumWithDeserialize<'a> {
        match self {
            SixErrorEnum::Seven(i) => {
                SixErrorEnumWithDeserialize::Seven(i.into_serialize_deserialize_version())
            }
            SixErrorEnum::Eight(i) => {
                SixErrorEnumWithDeserialize::Eight(i.into_serialize_deserialize_version())
            }
            SixErrorEnum::Another(i) => SixErrorEnumWithDeserialize::Another(i.to_string()),
            SixErrorEnum::AnotherVec(i) => {
                //todo - display implemented or not
                // let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                //     let stringified_element =
                //         element.lines().fold(String::from(""), |mut acc, line| {
                //             acc.push_str(&format!(" {}\n", line));
                //             acc
                //         });
                //     acc.push_str(&stringified_element);
                //     acc
                // });
                // format!("[\n{}]", stringified_vec)
                SixErrorEnumWithDeserialize::AnotherVec(i)
            }
            SixErrorEnum::AnotherHashmap(i) => {
                //todo - display implemented or not
                // let stringified_hashmap =
                //     i.iter().fold(String::from(""), |mut acc, (key, value)| {
                //         let stringified_value =
                //             value.lines().fold(String::from(""), |mut accc, line| {
                //                 accc.push_str(&format!(" {}\n", line));
                //                 accc
                //             });
                //         acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                //         acc
                //     });
                SixErrorEnumWithDeserialize::AnotherHashmap(i)
            }
        }
    }
}

impl<'a> std::fmt::Display for SevenError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
        write!(f, "{}", self.to_string_without_config())
    }
}
impl<'a> std::fmt::Display for SevenErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
        write!(f, "{}", self.to_string_without_config_with_deserialize())
    }
}
impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
        'a,
        ConfigGeneric,
    > for SevenError<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        match self {
            SevenError::Something {
                error: _unused_first_argument,
                code_occurence: _unused_second_argument,
            } => {
                use crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig;
                self.source_to_string_without_config()
            }
        }
    }
}
impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
        'a,
    > for SevenError<'a>
{
    fn source_to_string_without_config(&self) -> String {
        match self {
            SevenError::Something {
                error,
                code_occurence: _unused_second_argument,
            } => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                error.to_string()
            }
        }
    }
}
impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
    for SevenError<'a>
{
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
        match self {
            SevenError::Something {
                error: _unused_first_argument,
                code_occurence,
            } => code_occurence,
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum SevenErrorWithDeserialize<'a> {
    Something {
        error: String,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
    },
}
impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
        'a,
    > for SevenErrorWithDeserialize<'a>
{
    fn source_to_string_without_config(&self) -> String {
        match self {
            SevenErrorWithDeserialize::Something {
                error,
                code_occurence: _unused_second_argument,
            } => error.to_string(),
        }
    }
}
impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
    for SevenErrorWithDeserialize<'a>
{
    fn get_code_occurence_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
        match self {
            SevenErrorWithDeserialize::Something {
                error: _unused_first_argument,
                code_occurence,
            } => code_occurence,
        }
    }
}
impl<'a> SevenError<'a> {
    pub fn into_serialize_deserialize_version(self) -> SevenErrorWithDeserialize<'a> {
        match self {
            SevenError::Something {
                error,
                code_occurence,
            } => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                SevenErrorWithDeserialize::Something {
                    error: error.to_string(),
                    code_occurence: code_occurence.into_serialize_deserialize_version(),
                }
            }
        }
    }
}
impl<'a> std::fmt::Display for EightError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
        write!(f, "{}", self.to_string_without_config())
    }
}
impl<'a> std::fmt::Display for EightErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
        write!(f, "{}", self.to_string_without_config_with_deserialize())
    }
}
impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
        'a,
        ConfigGeneric,
    > for EightError<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        match self {
            EightError::Something {
                error: _unused_first_argument,
                code_occurence: _unused_second_argument,
            } => {
                use crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig;
                self.source_to_string_without_config()
            }
        }
    }
}
impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
        'a,
    > for EightError<'a>
{
    fn source_to_string_without_config(&self) -> String {
        match self {
            EightError::Something {
                error,
                code_occurence: _unused_second_argument,
            } => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                error.to_string()
            }
        }
    }
}
impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
    for EightError<'a>
{
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
        match self {
            EightError::Something {
                error: _unused_first_argument,
                code_occurence,
            } => code_occurence,
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum EightErrorWithDeserialize<'a> {
    Something {
        error: String,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
    },
}
impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
        'a,
    > for EightErrorWithDeserialize<'a>
{
    fn source_to_string_without_config(&self) -> String {
        match self {
            EightErrorWithDeserialize::Something {
                error,
                code_occurence: _unused_second_argument,
            } => error.to_string(),
        }
    }
}
impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
    for EightErrorWithDeserialize<'a>
{
    fn get_code_occurence_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
        match self {
            EightErrorWithDeserialize::Something {
                error: _unused_first_argument,
                code_occurence,
            } => code_occurence,
        }
    }
}
impl<'a> EightError<'a> {
    pub fn into_serialize_deserialize_version(self) -> EightErrorWithDeserialize<'a> {
        match self {
            EightError::Something {
                error,
                code_occurence,
            } => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                EightErrorWithDeserialize::Something {
                    error: error.to_string(),
                    code_occurence: code_occurence.into_serialize_deserialize_version(),
                }
            }
        }
    }
}

//TODO - WHATS DO WITH THIS ? (JOIN ALL SUTIATION)

// #[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
// pub enum CheckAvailabilityError<'a> {
//     Net {
//         error: crate::server::net::net_check_availability::NetCheckAvailabilityError<'a>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     Postgres {
//         error: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityError<
//             'a,
//         >,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     Mongo {
//         error: crate::server::mongo::mongo_check_availability::MongoCheckAvailabilityError<'a>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     Many {
//         inner_errors: Vec<CheckAvailabilityErrorEnum<'a>>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
// pub enum CheckAvailabilityErrorEnum<'a> {
//     Net(crate::server::net::net_check_availability::NetCheckAvailabilityError<'a>),
//     Postgres(
//         crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityError<'a>,
//     ),
//     Mongo(crate::server::mongo::mongo_check_availability::MongoCheckAvailabilityError<'a>),
// }

//    match join!(
//         net_check_availability(net_url, false),
//         postgres_check_availability(postgres_url, false),
//         mongo_check_availability(
//             MONGO_CLIENT_OPTIONS.deref().to_owned(), //std::time::Duration::from_millis(CONFIG.mongo_connection_timeout),
//             &CONFIG.mongo_providers_logs_db_name,
//             &CONFIG.source_place_type,
//             false,
//         ),
//     ) {
//         (Ok(_), Ok(_), Ok(_)) => Ok(()),
//         (Ok(_), Ok(_), Err(m)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::Mongo {
//             error: *m,
//             code_occurence: tufa_common::code_occurence!(),
//         })),
//         (Ok(_), Err(p), Ok(_)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::Postgres {
//             error: *p,
//             code_occurence: tufa_common::code_occurence!(),
//         })),
//         (Ok(_), Err(p), Err(m)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::Many {
//             inner_errors: vec![
//                 tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorEnum::Postgres(*p),
//                 tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorEnum::Mongo(*m),
//             ],
//             code_occurence: tufa_common::code_occurence!(),
//         })),
//         (Err(n), Ok(_), Ok(_)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::Net {
//             error: *n,
//             code_occurence: tufa_common::code_occurence!(),
//         })),
//         (Err(n), Ok(_), Err(m)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::Many {
//             inner_errors: vec![
//                 tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorEnum::Net(*n),
//                 tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorEnum::Mongo(*m),
//             ],
//             code_occurence: tufa_common::code_occurence!(),
//         })),
//         (Err(n), Err(p), Ok(_)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::Many {
//             inner_errors: vec![
//                 tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorEnum::Net(*n),
//                 tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorEnum::Postgres(*p),
//             ],
//             code_occurence: tufa_common::code_occurence!(),
//         })),
//         (Err(n), Err(p), Err(m)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::Many {
//             inner_errors: vec![
//                 tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorEnum::Net(*n),
//                 tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorEnum::Postgres(*p),
//                 tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorEnum::Mongo(*m),
//             ],
//             code_occurence: tufa_common::code_occurence!(),
//         })),

///////////////////////////////////////////////////
#[derive(Debug)]
pub struct Kekw {}
#[derive(Debug, thiserror::Error)] //, error_occurence::ImplErrorOccurence
pub enum OneErrorEnum<'a> {
    Seven(SevenError<'a>),
    // Eight(EightError<'a>),
    //todo #[simple_display] and #[display_foreign_type] support
    // #[origin]
    Another(String),
    Foreign(Kekw),
    //todo #[simple_display] and #[display_foreign_type] support
    // #[origin]
    AnotherVec(Vec<String>),
    // //todo #[simple_display] and #[display_foreign_type] support
    // #[origin]
    AnotherHashmap(std::collections::HashMap<String, String>),
    ForeignVec(Vec<Kekw>),
    ForeignHashmap(std::collections::HashMap<String, Kekw>), //todo - #[display_foreign_type] and #[to_string] for key
    ForeignKVHashmap(std::collections::HashMap<Kekw, Kekw>),
}

impl crate::traits::display_foreign_type::DisplayForeignType for Kekw {
    fn display_foreign_type(&self) -> String {
        String::from("kekw")
    }
}
//////////////
impl<'a> std::fmt::Display for OneErrorEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
        write!(f, "{}", self.to_string_without_config())
    }
}
impl<'a> std::fmt::Display for OneErrorEnumWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
        write!(f, "{}", self.to_string_without_config_with_deserialize())
    }
}
impl < 'a, ConfigGeneric > crate :: traits :: error_logs_logic ::
to_string_with_config :: ToStringWithConfigForSourceToStringWithConfig < 'a,
ConfigGeneric, > for OneErrorEnum < 'a > where ConfigGeneric : crate :: traits
:: fields :: GetSourcePlaceType + crate :: traits :: fields :: GetTimezone +
crate :: traits :: get_server_address :: GetServerAddress,
{
    fn
    to_string_with_config_for_source_to_string_with_config(& self, config : &
    ConfigGeneric) -> String
    {
        match self
        {
            OneErrorEnum :: Seven(i) =>
            {
                i.to_string_with_config_for_source_to_string_with_config(config)
            }
            //here
            OneErrorEnum::Another(i) => {
                i.to_string()
            },
            OneErrorEnum::Foreign(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.display_foreign_type()
            },
            OneErrorEnum::AnotherVec(i) => {
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element = element.lines().fold(String::from(""), |mut acc, line| {
                        acc.push_str(&format!(" {}\n", line));
                        acc
                    });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            },
            OneErrorEnum::AnotherHashmap(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value.lines().fold(String::from(""), |mut accc, line| {
                        accc.push_str(&format!(" {}\n", line));
                        accc
                    });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            },
            OneErrorEnum::ForeignVec(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element = element.display_foreign_type().lines().fold(String::from(""), |mut acc, line| {
                        acc.push_str(&format!(" {}\n", line));
                        acc
                    });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            },
            OneErrorEnum::ForeignHashmap(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value.display_foreign_type().lines().fold(String::from(""), |mut accc, line| {
                        accc.push_str(&format!(" {}\n", line));
                        accc
                    });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            },
            OneErrorEnum::ForeignKVHashmap(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value.display_foreign_type().lines().fold(String::from(""), |mut accc, line| {
                        accc.push_str(&format!(" {}\n", line));
                        accc
                    });
                    acc.push_str(&format!("{} [\n{}]\n", key.display_foreign_type(), stringified_value));
                    acc
                })
            },
        }
    }
}
impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
    for OneErrorEnum<'a>
{
    fn to_string_without_config(&self) -> String {
        match self {
            OneErrorEnum::Seven(i) => i.to_string_without_config(),
            //here
            OneErrorEnum::Another(i) => i.to_string(),
            OneErrorEnum::Foreign(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.display_foreign_type()
            }
            OneErrorEnum::AnotherVec(i) => {
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element =
                        element.lines().fold(String::from(""), |mut acc, line| {
                            acc.push_str(&format!(" {}\n", line));
                            acc
                        });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            }
            OneErrorEnum::AnotherHashmap(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value =
                        value.lines().fold(String::from(""), |mut accc, line| {
                            accc.push_str(&format!(" {}\n", line));
                            accc
                        });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            }
            OneErrorEnum::ForeignVec(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element = element.display_foreign_type().lines().fold(
                        String::from(""),
                        |mut acc, line| {
                            acc.push_str(&format!(" {}\n", line));
                            acc
                        },
                    );
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            }
            OneErrorEnum::ForeignHashmap(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value.display_foreign_type().lines().fold(
                        String::from(""),
                        |mut accc, line| {
                            accc.push_str(&format!(" {}\n", line));
                            accc
                        },
                    );
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            }
            OneErrorEnum::ForeignKVHashmap(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value.display_foreign_type().lines().fold(
                        String::from(""),
                        |mut accc, line| {
                            accc.push_str(&format!(" {}\n", line));
                            accc
                        },
                    );
                    acc.push_str(&format!(
                        "{} [\n{}]\n",
                        key.display_foreign_type(),
                        stringified_value
                    ));
                    acc
                })
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum OneErrorEnumWithDeserialize<'a> {
    #[serde(borrow)]
    Seven(SevenErrorWithDeserialize<'a>),
    //here
    Another(String),
    Foreign(String),
    AnotherVec(Vec<String>),
    AnotherHashmap(std::collections::HashMap<String, String>),
    ForeignVec(Vec<String>),
    ForeignHashmap(std::collections::HashMap<String, String>), //todo - #[display_foreign_type] and #[to_string] for key
    ForeignKVHashmap(std::collections::HashMap<String, String>),
}
impl<'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
        'a,
    > for OneErrorEnumWithDeserialize<'a>
{
    fn to_string_without_config_with_deserialize(&self) -> String {
        match self {
            OneErrorEnumWithDeserialize::Seven(i) => i.to_string_without_config_with_deserialize(),
            //here
            OneErrorEnumWithDeserialize::Another(i) => i.to_string(),
            OneErrorEnumWithDeserialize::Foreign(i) => i.to_string(),
            OneErrorEnumWithDeserialize::AnotherVec(i) => {
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element =
                        element.lines().fold(String::from(""), |mut acc, line| {
                            acc.push_str(&format!(" {}\n", line));
                            acc
                        });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            }
            OneErrorEnumWithDeserialize::AnotherHashmap(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value =
                        value.lines().fold(String::from(""), |mut accc, line| {
                            accc.push_str(&format!(" {}\n", line));
                            accc
                        });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            }
            OneErrorEnumWithDeserialize::ForeignVec(i) => {
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element =
                        element.lines().fold(String::from(""), |mut acc, line| {
                            acc.push_str(&format!(" {}\n", line));
                            acc
                        });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            }
            OneErrorEnumWithDeserialize::ForeignHashmap(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value =
                        value.lines().fold(String::from(""), |mut accc, line| {
                            accc.push_str(&format!(" {}\n", line));
                            accc
                        });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            }
            OneErrorEnumWithDeserialize::ForeignKVHashmap(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value =
                        value.lines().fold(String::from(""), |mut accc, line| {
                            accc.push_str(&format!(" {}\n", line));
                            accc
                        });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            }
        }
    }
}
impl<'a> OneErrorEnum<'a> {
    pub fn into_serialize_deserialize_version(self) -> OneErrorEnumWithDeserialize<'a> {
        match self {
            OneErrorEnum::Seven(i) => {
                OneErrorEnumWithDeserialize::Seven(i.into_serialize_deserialize_version())
            }
            //here
            OneErrorEnum::Another(i) => OneErrorEnumWithDeserialize::Another(i),
            OneErrorEnum::Foreign(i) => OneErrorEnumWithDeserialize::Foreign({
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.display_foreign_type()
            }),
            OneErrorEnum::AnotherVec(i) => OneErrorEnumWithDeserialize::AnotherVec(i),
            OneErrorEnum::AnotherHashmap(i) => OneErrorEnumWithDeserialize::AnotherHashmap(i),
            OneErrorEnum::ForeignVec(i) => OneErrorEnumWithDeserialize::ForeignVec({
                i.into_iter()
                    .map(|e| {
                        use crate::traits::display_foreign_type::DisplayForeignType;
                        e.display_foreign_type()
                    })
                    .collect()
            }),
            OneErrorEnum::ForeignHashmap(i) => OneErrorEnumWithDeserialize::ForeignHashmap({
                i.into_iter()
                    .map(|(k, v)| {
                        use crate::traits::display_foreign_type::DisplayForeignType;
                        (k, v.display_foreign_type())
                    })
                    .collect()
            }),
            OneErrorEnum::ForeignKVHashmap(i) => OneErrorEnumWithDeserialize::ForeignKVHashmap({
                i.into_iter()
                    .map(|(k, v)| {
                        use crate::traits::display_foreign_type::DisplayForeignType;
                        (k.display_foreign_type(), v.display_foreign_type())
                    })
                    .collect()
            }),
        }
    }
}
