use crate::traits::display_foreign_type;

pub fn dev() {
    if let Err(e) = six() {
        //todo - this actually must be a config struct function, not an error - config.error_log(e)
        println!("{}", *e);
        use crate::traits::error_logs_logic::error_log::ErrorLog;
        e.error_log(once_cell::sync::Lazy::force(
            //todo - this must be call once on start of the program
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

#[derive(Debug, Hash, thiserror::Error, serde::Serialize)] //, error_occurence::ImplErrorOccurence
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
#[derive(Debug, Hash, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
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
// #[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)] //
// pub enum TwoErrorEnum<'a> {
//     Seven(SevenError<'a>),
//     // // #[to_string]
//     // Another(String),
//     // // #[display_foreign_type]
//     // Foreign(Kekw),
//     // // #[to_string]
//     // AnotherVec(std::vec::Vec<String>),
//     #[hashmap_key_to_string_value_to_string]
//     AnotherHashmap(std::collections::HashMap<std::string::String, std::string::String>),
//     // // #[display_foreign_type]
//     // ForeignVec(std::vec::Vec<Kekw>),
//     // // #[hashmap_key_to_string_value_display_foreign_type]
//     // ForeignHashmap(std::collections::HashMap<String, Kekw>),
//     // // #[hashmap_key_display_foreign_type_value_display_foreign_type]
//     // ForeignKVHashmap(std::collections::HashMap<Kekw, Kekw>),
//     // // #[hashmap_key_display_foreign_type_value_to_string]
//     // ForeignKHashmap(std::collections::HashMap<Kekw, String>),
// }
#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Kekw {}

impl crate::traits::display_foreign_type::DisplayForeignType for Kekw {
    fn display_foreign_type(&self) -> String {
        String::from("kekw")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Omegalul {}

impl std::fmt::Display for Omegalul {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "omegalul")
    }
}
//todo - check on lifetime - is it there or is it not - and generate/not generate it
#[derive(Debug, thiserror::Error)] //, error_occurence::ImplErrorOccurence
pub enum OneErrorEnum<'a> {
    // #[to_string]
    ToString(crate::dev::Omegalul),
    // #[display_foreign_type]
    DisplayForeignType(crate::dev::Kekw),
    // #[error_occurence]
    ErrorOccurence(crate::dev::SevenError<'a>),
    // #[vec_to_string]
    VecToString(std::vec::Vec<crate::dev::Omegalul>),
    // #[vec_display_foreign_type]
    VecDisplayForeignType(std::vec::Vec<crate::dev::Kekw>),
    // #[vec_error_occurence]
    VecErrorOccurence(std::vec::Vec<crate::dev::SevenError<'a>>),
    // // #[hashmap_key_to_string_value_to_string]
    HashMapKeyToStringValueToString(
        std::collections::HashMap<crate::dev::Omegalul, crate::dev::Omegalul>,
    ),
    // #[hashmap_key_to_string_value_display_foreign_type]
    HashMapKeyToStringValueDisplayForeignType(
        std::collections::HashMap<crate::dev::Omegalul, crate::dev::Kekw>,
    ),
    // #[hashmap_key_to_string_value_error_occurence]
    HashMapKeyToStringValueErrorOccurence(
        std::collections::HashMap<crate::dev::Omegalul, crate::dev::SevenError<'a>>,
    ),
    // #[hashmap_key_display_foreign_type_value_to_string]
    HashMapKeyDisplayForeignTypeValueToString(
        std::collections::HashMap<crate::dev::Kekw, crate::dev::Omegalul>,
    ),
    // #[hashmap_key_display_foreign_type_value_display_foreign_type]
    HashMapKeyDisplayForeignTypeValueDisplayForeignType(
        std::collections::HashMap<crate::dev::Kekw, crate::dev::Kekw>,
    ),
    // #[hashmap_key_display_foreign_type_value_error_occurence]
    HashMapKeyDisplayForeignTypeValueErrorOccurence(
        std::collections::HashMap<crate::dev::Kekw, crate::dev::SevenError<'a>>,
    ),
    // // #[hashmap_key_error_occurence_value_to_string]
    // HashMapKeyErrorOccurenceValueToString(
    //     std::collections::HashMap<crate::dev::SevenError<'a>, crate::dev::Omegalul>,
    // ),
    // // #[hashmap_key_error_occurence_value_display_foreign_type]
    // HashMapKeyErrorOccurenceValueDisplayForeignType(
    //     std::collections::HashMap<crate::dev::SevenError<'a>, crate::dev::Kekw>,
    // ),
    // // #[hashmap_key_error_occurence_value_error_occurence]
    // HashMapKeyErrorOccurenceValueErrorOccurence(
    //     std::collections::HashMap<crate::dev::SevenError<'a>, crate::dev::SevenError<'a>>,
    // ),
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
            OneErrorEnum::ToString(i) => {
                i.to_string()
            }
            OneErrorEnum::DisplayForeignType(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.display_foreign_type()
            },
            OneErrorEnum::ErrorOccurence(i) =>
            {
                i.to_string_with_config_for_source_to_string_with_config(config)
            }
            OneErrorEnum::VecToString(i) => {
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element = element.to_string().lines().fold(String::from(""), |mut acc, line| {
                        acc.push_str(&format!(" {}\n", line));
                        acc
                    });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            },
            OneErrorEnum::VecDisplayForeignType(i) => {
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
            OneErrorEnum::VecErrorOccurence(i) => {
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element = element.to_string_with_config_for_source_to_string_with_config(config).lines().fold(String::from(""), |mut acc, line| {
                        acc.push_str(&format!(" {}\n", line));
                        acc
                    });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            },
            OneErrorEnum::HashMapKeyToStringValueToString(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value.to_string().lines().fold(String::from(""), |mut accc, line| {
                        accc.push_str(&format!(" {}\n", line));
                        accc
                    });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            },
            OneErrorEnum::HashMapKeyToStringValueDisplayForeignType(i) => {
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
            OneErrorEnum::HashMapKeyToStringValueErrorOccurence(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value.to_string_with_config_for_source_to_string_with_config(config).lines().fold(String::from(""), |mut accc, line| {
                        accc.push_str(&format!(" {}\n", line));
                        accc
                    });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            },
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueToString(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value.to_string().lines().fold(String::from(""), |mut accc, line| {
                        accc.push_str(&format!(" {}\n", line));
                        accc
                    });
                    acc.push_str(&format!("{} [\n{}]\n", key.display_foreign_type(), stringified_value));
                    acc
                })
            },
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) => {
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
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueErrorOccurence(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value.to_string_with_config_for_source_to_string_with_config(config).lines().fold(String::from(""), |mut accc, line| {
                        accc.push_str(&format!(" {}\n", line));
                        accc
                    });
                    acc.push_str(&format!("{} [\n{}]\n", key.display_foreign_type(), stringified_value));
                    acc
                })
            }
            // OneErrorEnum::HashMapKeyErrorOccurenceValueToString(i) => {
            //     i.iter().fold(String::from(""), |mut acc, (key, value)| {
            //         let stringified_value = value.to_string().lines().fold(String::from(""), |mut accc, line| {
            //             accc.push_str(&format!(" {}\n", line));
            //             accc
            //         });
            //         acc.push_str(&format!("{} [\n{}]\n", key.to_string_with_config_for_source_to_string_with_config(config), stringified_value));
            //         acc
            //     })
            // },
            // OneErrorEnum::HashMapKeyErrorOccurenceValueDisplayForeignType(i) => {
            //     use crate::traits::display_foreign_type::DisplayForeignType;
            //     i.iter().fold(String::from(""), |mut acc, (key, value)| {
            //         let stringified_value = value.display_foreign_type().lines().fold(String::from(""), |mut accc, line| {
            //             accc.push_str(&format!(" {}\n", line));
            //             accc
            //         });
            //         acc.push_str(&format!("{} [\n{}]\n", key.to_string_with_config_for_source_to_string_with_config(config), stringified_value));
            //         acc
            //     })
            // },
            // OneErrorEnum::HashMapKeyErrorOccurenceValueErrorOccurence(i) => {
            //     i.iter().fold(String::from(""), |mut acc, (key, value)| {
            //         let stringified_value = value.to_string_with_config_for_source_to_string_with_config(config).lines().fold(String::from(""), |mut accc, line| {
            //             accc.push_str(&format!(" {}\n", line));
            //             accc
            //         });
            //         acc.push_str(&format!("{} [\n{}]\n", key.to_string_with_config_for_source_to_string_with_config(config), stringified_value));
            //         acc
            //     })
            // },
        }
    }
}
impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
    for OneErrorEnum<'a>
{
    fn to_string_without_config(&self) -> String {
        match self {
            OneErrorEnum::ToString(i) => i.to_string(),
            OneErrorEnum::DisplayForeignType(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.display_foreign_type()
            }
            OneErrorEnum::ErrorOccurence(i) => i.to_string_without_config(),
            OneErrorEnum::VecToString(i) => {
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element =
                        element
                            .to_string()
                            .lines()
                            .fold(String::from(""), |mut acc, line| {
                                acc.push_str(&format!(" {}\n", line));
                                acc
                            });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            }
            OneErrorEnum::VecDisplayForeignType(i) => {
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
            OneErrorEnum::VecErrorOccurence(i) => {
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element = element.to_string_without_config().lines().fold(
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
            OneErrorEnum::HashMapKeyToStringValueToString(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value =
                        value
                            .to_string()
                            .lines()
                            .fold(String::from(""), |mut accc, line| {
                                accc.push_str(&format!(" {}\n", line));
                                accc
                            });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            }
            OneErrorEnum::HashMapKeyToStringValueDisplayForeignType(i) => {
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
            OneErrorEnum::HashMapKeyToStringValueErrorOccurence(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value.to_string_without_config().lines().fold(
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
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueToString(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value =
                        value
                            .to_string()
                            .lines()
                            .fold(String::from(""), |mut accc, line| {
                                accc.push_str(&format!(" {}\n", line));
                                accc
                            });
                    acc.push_str(&format!(
                        "{} [\n{}]\n",
                        key.display_foreign_type(),
                        stringified_value
                    ));
                    acc
                })
            }
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) => {
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
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueErrorOccurence(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value.to_string_without_config().lines().fold(
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
            } // OneErrorEnum::HashMapKeyErrorOccurenceValueToString(i) => {
              //     i.iter().fold(String::from(""), |mut acc, (key, value)| {
              //         let stringified_value =
              //             value
              //                 .to_string()
              //                 .lines()
              //                 .fold(String::from(""), |mut accc, line| {
              //                     accc.push_str(&format!(" {}\n", line));
              //                     accc
              //                 });
              //         acc.push_str(&format!(
              //             "{} [\n{}]\n",
              //             key.to_string_without_config(),
              //             stringified_value
              //         ));
              //         acc
              //     })
              // }
              // OneErrorEnum::HashMapKeyErrorOccurenceValueDisplayForeignType(i) => {
              //     use crate::traits::display_foreign_type::DisplayForeignType;
              //     i.iter().fold(String::from(""), |mut acc, (key, value)| {
              //         let stringified_value = value.display_foreign_type().lines().fold(
              //             String::from(""),
              //             |mut accc, line| {
              //                 accc.push_str(&format!(" {}\n", line));
              //                 accc
              //             },
              //         );
              //         acc.push_str(&format!(
              //             "{} [\n{}]\n",
              //             key.to_string_without_config(),
              //             stringified_value
              //         ));
              //         acc
              //     })
              // }
              // OneErrorEnum::HashMapKeyErrorOccurenceValueErrorOccurence(i) => {
              //     i.iter().fold(String::from(""), |mut acc, (key, value)| {
              //         let stringified_value = value.to_string_without_config().lines().fold(
              //             String::from(""),
              //             |mut accc, line| {
              //                 accc.push_str(&format!(" {}\n", line));
              //                 accc
              //             },
              //         );
              //         acc.push_str(&format!(
              //             "{} [\n{}]\n",
              //             key.to_string_without_config(),
              //             stringified_value
              //         ));
              //         acc
              //     })
              // }
        }
    }
}
#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)] //PartialEq, Eq, Hash,
pub enum OneErrorEnumWithDeserialize<'a> {
    ToString(crate::dev::Omegalul),
    DisplayForeignType(String),
    #[serde(borrow)]
    ErrorOccurence(crate::dev::SevenErrorWithDeserialize<'a>),
    VecToString(std::vec::Vec<crate::dev::Omegalul>),
    VecDisplayForeignType(std::vec::Vec<String>),
    #[serde(borrow)]
    VecErrorOccurence(std::vec::Vec<crate::dev::SevenErrorWithDeserialize<'a>>),
    HashMapKeyToStringValueToString(
        std::collections::HashMap<crate::dev::Omegalul, crate::dev::Omegalul>,
    ),
    HashMapKeyToStringValueDisplayForeignType(
        std::collections::HashMap<crate::dev::Omegalul, String>,
    ),
    HashMapKeyToStringValueErrorOccurence(
        std::collections::HashMap<crate::dev::Omegalul, crate::dev::SevenErrorWithDeserialize<'a>>,
    ),
    HashMapKeyDisplayForeignTypeValueToString(
        std::collections::HashMap<String, crate::dev::Omegalul>,
    ),
    HashMapKeyDisplayForeignTypeValueDisplayForeignType(std::collections::HashMap<String, String>),
    #[serde(borrow)]
    HashMapKeyDisplayForeignTypeValueErrorOccurence(
        std::collections::HashMap<String, crate::dev::SevenErrorWithDeserialize<'a>>,
    ),
    // #[serde(borrow)]
    // HashMapKeyErrorOccurenceValueToString(
    //     std::collections::HashMap<crate::dev::SevenErrorWithDeserialize<'a>, crate::dev::Omegalul>,
    // ),
    // #[serde(borrow)]
    // HashMapKeyErrorOccurenceValueDisplayForeignType(
    //     std::collections::HashMap<crate::dev::SevenErrorWithDeserialize<'a>, String>,
    // ),
    // #[serde(borrow)]
    // HashMapKeyErrorOccurenceValueErrorOccurence(
    //     std::collections::HashMap<
    //         crate::dev::SevenErrorWithDeserialize<'a>,
    //         crate::dev::SevenErrorWithDeserialize<'a>,
    //     >,
    // ),
}
impl<'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
        'a,
    > for OneErrorEnumWithDeserialize<'a>
{
    fn to_string_without_config_with_deserialize(&self) -> String {
        match self {
            OneErrorEnumWithDeserialize::ToString(i) => i.to_string(),
            OneErrorEnumWithDeserialize::DisplayForeignType(i) => i.to_string(),
            OneErrorEnumWithDeserialize::ErrorOccurence(i) => {
                i.to_string_without_config_with_deserialize()
            }
            OneErrorEnumWithDeserialize::VecToString(i) => {
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element =
                        element
                            .to_string()
                            .lines()
                            .fold(String::from(""), |mut acc, line| {
                                acc.push_str(&format!(" {}\n", line));
                                acc
                            });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            }
            OneErrorEnumWithDeserialize::VecDisplayForeignType(i) => {
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
            OneErrorEnumWithDeserialize::VecErrorOccurence(i) => {
                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                    let stringified_element = element
                        .to_string_without_config_with_deserialize()
                        .lines()
                        .fold(String::from(""), |mut acc, line| {
                            acc.push_str(&format!(" {}\n", line));
                            acc
                        });
                    acc.push_str(&stringified_element);
                    acc
                });
                format!("[\n{}]", stringified_vec)
            }
            OneErrorEnumWithDeserialize::HashMapKeyToStringValueToString(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value =
                        value
                            .to_string()
                            .lines()
                            .fold(String::from(""), |mut accc, line| {
                                accc.push_str(&format!(" {}\n", line));
                                accc
                            });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            }
            OneErrorEnumWithDeserialize::HashMapKeyToStringValueDisplayForeignType(i) => {
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
            OneErrorEnumWithDeserialize::HashMapKeyToStringValueErrorOccurence(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value
                        .to_string_without_config_with_deserialize()
                        .lines()
                        .fold(String::from(""), |mut accc, line| {
                            accc.push_str(&format!(" {}\n", line));
                            accc
                        });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            }
            OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueToString(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value =
                        value
                            .to_string()
                            .lines()
                            .fold(String::from(""), |mut accc, line| {
                                accc.push_str(&format!(" {}\n", line));
                                accc
                            });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            }
            OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) => {
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
            OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueErrorOccurence(i) => {
                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                    let stringified_value = value
                        .to_string_without_config_with_deserialize()
                        .lines()
                        .fold(String::from(""), |mut accc, line| {
                            accc.push_str(&format!(" {}\n", line));
                            accc
                        });
                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                    acc
                })
            } // OneErrorEnumWithDeserialize::HashMapKeyErrorOccurenceValueToString(i) => {
              //     i.iter().fold(String::from(""), |mut acc, (key, value)| {
              //         let stringified_value =
              //             value
              //                 .to_string()
              //                 .lines()
              //                 .fold(String::from(""), |mut accc, line| {
              //                     accc.push_str(&format!(" {}\n", line));
              //                     accc
              //                 });
              //         acc.push_str(&format!(
              //             "{} [\n{}]\n",
              //             key.to_string_without_config_with_deserialize(),
              //             stringified_value
              //         ));
              //         acc
              //     })
              // }
              // OneErrorEnumWithDeserialize::HashMapKeyErrorOccurenceValueDisplayForeignType(i) => {
              //     i.iter().fold(String::from(""), |mut acc, (key, value)| {
              //         let stringified_value =
              //             value.lines().fold(String::from(""), |mut accc, line| {
              //                 accc.push_str(&format!(" {}\n", line));
              //                 accc
              //             });
              //         acc.push_str(&format!(
              //             "{} [\n{}]\n",
              //             key.to_string_without_config_with_deserialize(),
              //             stringified_value
              //         ));
              //         acc
              //     })
              // }
              // OneErrorEnumWithDeserialize::HashMapKeyErrorOccurenceValueErrorOccurence(i) => {
              //     i.iter().fold(String::from(""), |mut acc, (key, value)| {
              //         let stringified_value = value
              //             .to_string_without_config_with_deserialize()
              //             .lines()
              //             .fold(String::from(""), |mut accc, line| {
              //                 accc.push_str(&format!(" {}\n", line));
              //                 accc
              //             });
              //         acc.push_str(&format!(
              //             "{} [\n{}]\n",
              //             key.to_string_without_config_with_deserialize(),
              //             stringified_value
              //         ));
              //         acc
              //     })
              // }
        }
    }
}
impl<'a> OneErrorEnum<'a> {
    pub fn into_serialize_deserialize_version(self) -> OneErrorEnumWithDeserialize<'a> {
        match self {
            OneErrorEnum::ToString(i) => OneErrorEnumWithDeserialize::ToString(i),
            OneErrorEnum::DisplayForeignType(i) => {
                OneErrorEnumWithDeserialize::DisplayForeignType({
                    use crate::traits::display_foreign_type::DisplayForeignType;
                    i.display_foreign_type()
                })
            }
            OneErrorEnum::ErrorOccurence(i) => {
                OneErrorEnumWithDeserialize::ErrorOccurence(i.into_serialize_deserialize_version())
            }
            OneErrorEnum::VecToString(i) => OneErrorEnumWithDeserialize::VecToString(i),
            OneErrorEnum::VecDisplayForeignType(i) => {
                OneErrorEnumWithDeserialize::VecDisplayForeignType({
                    i.into_iter()
                        .map(|e| {
                            use crate::traits::display_foreign_type::DisplayForeignType;
                            e.display_foreign_type()
                        })
                        .collect()
                })
            }
            OneErrorEnum::VecErrorOccurence(i) => OneErrorEnumWithDeserialize::VecErrorOccurence({
                i.into_iter()
                    .map(|e| {
                        use crate::traits::display_foreign_type::DisplayForeignType;
                        e.into_serialize_deserialize_version()
                    })
                    .collect()
            }),
            OneErrorEnum::HashMapKeyToStringValueToString(i) => {
                OneErrorEnumWithDeserialize::HashMapKeyToStringValueToString(i)
            }
            OneErrorEnum::HashMapKeyToStringValueDisplayForeignType(i) => {
                OneErrorEnumWithDeserialize::HashMapKeyToStringValueDisplayForeignType({
                    i.into_iter()
                        .map(|(k, v)| {
                            use crate::traits::display_foreign_type::DisplayForeignType;
                            (k, v.display_foreign_type())
                        })
                        .collect()
                })
            }
            OneErrorEnum::HashMapKeyToStringValueErrorOccurence(i) => {
                OneErrorEnumWithDeserialize::HashMapKeyToStringValueErrorOccurence({
                    i.into_iter()
                        .map(|(k, v)| (k, v.into_serialize_deserialize_version()))
                        .collect()
                })
            }
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueToString(i) => {
                OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueToString({
                    i.into_iter()
                        .map(|(k, v)| {
                            use crate::traits::display_foreign_type::DisplayForeignType;
                            (k.display_foreign_type(), v)
                        })
                        .collect()
                })
            }
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) => {
                OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueDisplayForeignType({
                    i.into_iter()
                        .map(|(k, v)| {
                            use crate::traits::display_foreign_type::DisplayForeignType;
                            (k.display_foreign_type(), v.display_foreign_type())
                        })
                        .collect()
                })
            }
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueErrorOccurence(i) => {
                OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueErrorOccurence({
                    i.into_iter()
                        .map(|(k, v)| {
                            use crate::traits::display_foreign_type::DisplayForeignType;
                            (
                                k.display_foreign_type(),
                                v.into_serialize_deserialize_version(),
                            )
                        })
                        .collect()
                })
            } // OneErrorEnum::HashMapKeyErrorOccurenceValueToString(i) => {
              //     OneErrorEnumWithDeserialize::HashMapKeyErrorOccurenceValueToString({
              //         i.into_iter()
              //             .map(|(k, v)| (k.into_serialize_deserialize_version(), v))
              //             .collect()
              //     })
              // }
              // OneErrorEnum::HashMapKeyErrorOccurenceValueDisplayForeignType(i) => {
              //     OneErrorEnumWithDeserialize::HashMapKeyErrorOccurenceValueDisplayForeignType({
              //         i.into_iter()
              //             .map(|(k, v)| {
              //                 use crate::traits::display_foreign_type::DisplayForeignType;
              //                 (
              //                     k.into_serialize_deserialize_version(),
              //                     v.display_foreign_type(),
              //                 )
              //             })
              //             .collect()
              //     })
              // }
              // OneErrorEnum::HashMapKeyErrorOccurenceValueErrorOccurence(i) => {
              //     OneErrorEnumWithDeserialize::HashMapKeyErrorOccurenceValueErrorOccurence({
              //         i.into_iter()
              //             .map(|(k, v)| {
              //                 (
              //                     k.into_serialize_deserialize_version(),
              //                     v.into_serialize_deserialize_version(),
              //                 )
              //             })
              //             .collect()
              //     })
              // }
        }
    }
}
