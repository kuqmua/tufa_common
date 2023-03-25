use crate::traits::{display_foreign_type, error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize};

pub fn dev() {
    // if let Err(e) = named() {
    //     //todo - this actually must be a config struct function, not an error - config.error_log(e)
    //     println!("{}", *e);
    //     use crate::traits::error_logs_logic::error_log::ErrorLog;
    //     e.error_log(once_cell::sync::Lazy::force(
    //         //todo - this must be call once on start of the program
    //         &crate::global_variables::runtime::config::CONFIG,
    //     ));
    // }
}

// pub fn named<'a>() -> Result<(), Box<NamedError<'a>>> {
//     return Err(Box::new(NamedError::Something {
//         one: String::from("named_one_error"),
//         two: String::from("named_two_error"),
//         code_occurence: crate::code_occurence_tufa_common!(),
//     }));
// }

// #[derive(Debug, thiserror::Error, serde::Serialize)] //, error_occurence::ImplErrorOccurence
// pub enum SixError<'a> {
//     Something {
//         //todo how to implement from for it?
//         inner_errors: std::vec::Vec<SixErrorEnum<'a>>,
//         // TODO inner_errors: std::vec::Vec<String>,
//         // TODO inner_errors: std::collections::HashMap<String, String>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }

// #[derive(Debug, thiserror::Error, serde::Serialize)] //, error_occurence::ImplErrorOccurence
// pub enum SixErrorEnum<'a> {
//     Seven(SevenError<'a>),
//     Eight(EightError<'a>),
//     // //todo #[simple_display] and #[display_foreign_type] support
//     // // #[origin]
//     Another(String),
//     //todo #[simple_display] and #[display_foreign_type] support
//     // #[origin]
//     AnotherVec(Vec<String>),
//     //todo #[simple_display] and #[display_foreign_type] support
//     // #[origin]
//     AnotherHashmap(std::collections::HashMap<String, String>),
// }

#[derive(Debug, thiserror::Error, serde :: Serialize)] //, error_occurence::ImplErrorOccurence
pub enum SevenError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// #[derive(Debug, thiserror::Error, serde::Serialize)] //, error_occurence::ImplErrorOccurence
// pub enum EightError<'a> {
//     Something {
//         error: String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }

// pub fn six<'a>() -> Result<(), Box<SixError<'a>>> {
//     let thread_join_handle = std::thread::spawn(move || eight());
//     let res = thread_join_handle.join().unwrap();
//     match (seven(), res) {
//         (Ok(_), Ok(_)) => todo!(),
//         (Ok(_), Err(_)) => todo!(),
//         (Err(_), Ok(_)) => todo!(),
//         (Err(seven_error), Err(eight_error)) => {
//             let ss = String::from("kekw");
//             let yy = vec![String::from("vec1")];
//             let hh =
//                 std::collections::HashMap::from([(String::from("key"), String::from("value"))]);
//             let f = SixError::Something {
//                 inner_errors: vec![
//                     SixErrorEnum::Seven(*seven_error),
//                     SixErrorEnum::Eight(*eight_error),
//                     // SixErrorEnum::Another(ss),
//                     // SixErrorEnum::AnotherVec(yy),
//                     // SixErrorEnum::AnotherHashmap(hh),
//                 ],
//                 code_occurence: crate::code_occurence_tufa_common!(),
//             };
//             return Err(Box::new(f));
//         }
//     }
// }

// pub fn seven<'a>() -> Result<(), Box<SevenError<'a>>> {
//     return Err(Box::new(SevenError::Something {
//         error: String::from("error_eight"),
//         code_occurence: crate::code_occurence_tufa_common!(),
//     }));
// }

// pub fn eight<'a>() -> Result<(), Box<EightError<'a>>> {
//     let f = EightError::Something {
//         error: String::from("error_eight"),
//         code_occurence: crate::code_occurence_tufa_common!(),
//     };
//     // use crate::traits::error_logs_logic::error_log::ErrorLog;
//     // f.error_log(once_cell::sync::Lazy::force(
//     //     &crate::global_variables::runtime::config::CONFIG,
//     // ));
//     return Err(Box::new(f));
// }

// impl<'a> std::fmt::Display for SixError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }
// impl<'a> std::fmt::Display for SixErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }
// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for SixError<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             SixError::Something {
//                 inner_errors,
//                 code_occurence: _unused_second_argument,
//             } => {
//                 use crate::traits::error_logs_logic::few_to_string_with_config::FewToStringWithConfig;
//                 inner_errors.few_to_string_with_config(config)
//             }
//         }
//     }
// }
// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for SixError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             SixError::Something {
//                 inner_errors,
//                 code_occurence: _unused_second_argument,
//             } => {
//                 use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
//                 inner_errors.few_to_string_without_config()
//             }
//         }
//     }
// }
// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
//     for SixError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             SixError::Something {
//                 inner_errors: _unused_first_argument,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
// pub enum SixErrorWithDeserialize<'a> {
//     Something {
//         #[serde(borrow)]
//         inner_errors: std::vec::Vec<SixErrorEnumWithDeserialize<'a>>,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }
// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for SixErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             SixErrorWithDeserialize::Something {
//                 inner_errors,
//                 code_occurence: _unused_second_argument,
//             } => {
//                 use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigWithDeserialize;
//                 inner_errors.few_to_string_without_config_with_deserialize()
//             }
//         }
//     }
// }
// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
//     for SixErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             SixErrorWithDeserialize::Something {
//                 inner_errors: _unused_first_argument,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }
// impl<'a> SixError<'a> {
//     pub fn into_serialize_deserialize_version(self) -> SixErrorWithDeserialize<'a> {
//         match self {
//             SixError::Something {
//                 inner_errors,
//                 code_occurence,
//             } => SixErrorWithDeserialize::Something {
//                 inner_errors: inner_errors
//                     .into_iter()
//                     .map(|e| e.into_serialize_deserialize_version())
//                     .collect(),
//                 code_occurence: code_occurence.into_serialize_deserialize_version(),
//             },
//         }
//     }
// }
// impl<'a> std::fmt::Display for SixErrorEnum<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }
// impl<'a> std::fmt::Display for SixErrorEnumWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }
// impl < 'a, ConfigGeneric > crate :: traits :: error_logs_logic ::
// to_string_with_config :: ToStringWithConfigForSourceToStringWithConfig < 'a,
// ConfigGeneric, > for SixErrorEnum < 'a > where ConfigGeneric : crate :: traits
// :: fields :: GetSourcePlaceType + crate :: traits :: fields :: GetTimezone +
// crate :: traits :: get_server_address :: GetServerAddress,
// {
//     fn
//     to_string_with_config_for_source_to_string_with_config(& self, config : &
//     ConfigGeneric) -> String
//     {
//         match self
//         {
//             SixErrorEnum :: Seven(i) =>
//             {
//                 i.to_string_with_config_for_source_to_string_with_config(config)
//             }, SixErrorEnum :: Eight(i) =>
//             {
//                 i.to_string_with_config_for_source_to_string_with_config(config)
//             },
//             SixErrorEnum :: Another(i) => i.to_string(),
//             SixErrorEnum::AnotherVec(i) => {
//                 let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
//                     let stringified_element = element.lines().fold(String::from(""), |mut acc, line| {
//                         acc.push_str(&format!(" {}\n", line));
//                         acc
//                     });
//                     acc.push_str(&stringified_element);
//                     acc
//                 });
//                 format!("[\n{}]", stringified_vec)
//             },
//             SixErrorEnum::AnotherHashmap(i) => {
//                 i.iter().fold(String::from(""), |mut acc, (key, value)| {
//                     let stringified_value = value.lines().fold(String::from(""), |mut accc, line| {
//                         accc.push_str(&format!(" {}\n", line));
//                         accc
//                     });
//                     acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
//                     acc
//                 })
//             },
//         }
//     }
// }
// impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
//     for SixErrorEnum<'a>
// {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             SixErrorEnum::Seven(i) => i.to_string_without_config(),
//             SixErrorEnum::Eight(i) => i.to_string_without_config(),
//             SixErrorEnum::Another(i) => i.to_string(),
//             SixErrorEnum::AnotherVec(i) => {
//                 let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
//                     let stringified_element =
//                         element.lines().fold(String::from(""), |mut acc, line| {
//                             acc.push_str(&format!(" {}\n", line));
//                             acc
//                         });
//                     acc.push_str(&stringified_element);
//                     acc
//                 });
//                 format!("[\n{}]", stringified_vec)
//             }
//             SixErrorEnum::AnotherHashmap(i) => {
//                 i.iter().fold(String::from(""), |mut acc, (key, value)| {
//                     let stringified_value =
//                         value.lines().fold(String::from(""), |mut accc, line| {
//                             accc.push_str(&format!(" {}\n", line));
//                             accc
//                         });
//                     acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
//                     acc
//                 })
//             }
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
// pub enum SixErrorEnumWithDeserialize<'a> {
//     #[serde(borrow)]
//     Seven(SevenErrorWithDeserialize<'a>),
//     #[serde(borrow)]
//     Eight(EightErrorWithDeserialize<'a>),
//     Another(String),
//     AnotherVec(Vec<String>),
//     AnotherHashmap(std::collections::HashMap<String, String>),
// }
// impl<'a>
//     crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
//         'a,
//     > for SixErrorEnumWithDeserialize<'a>
// {
//     fn to_string_without_config_with_deserialize(&self) -> String {
//         match self {
//             SixErrorEnumWithDeserialize::Seven(i) => i.to_string_without_config_with_deserialize(),
//             SixErrorEnumWithDeserialize::Eight(i) => i.to_string_without_config_with_deserialize(),
//             SixErrorEnumWithDeserialize::Another(i) => i.to_string(), //todo or to display_foreign_type()
//             SixErrorEnumWithDeserialize::AnotherVec(i) => {
//                 //todo or to display_foreign_type()
//                 let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
//                     let stringified_element =
//                         element.lines().fold(String::from(""), |mut acc, line| {
//                             acc.push_str(&format!(" {}\n", line));
//                             acc
//                         });
//                     acc.push_str(&stringified_element);
//                     acc
//                 });
//                 format!("[\n{}]", stringified_vec)
//             }
//             SixErrorEnumWithDeserialize::AnotherHashmap(i) => {
//                 //todo or to display_foreign_type()
//                 i.iter().fold(String::from(""), |mut acc, (key, value)| {
//                     let stringified_value =
//                         value.lines().fold(String::from(""), |mut accc, line| {
//                             accc.push_str(&format!(" {}\n", line));
//                             accc
//                         });
//                     acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
//                     acc
//                 })
//             }
//         }
//     }
// }
// impl<'a> SixErrorEnum<'a> {
//     pub fn into_serialize_deserialize_version(self) -> SixErrorEnumWithDeserialize<'a> {
//         match self {
//             SixErrorEnum::Seven(i) => {
//                 SixErrorEnumWithDeserialize::Seven(i.into_serialize_deserialize_version())
//             }
//             SixErrorEnum::Eight(i) => {
//                 SixErrorEnumWithDeserialize::Eight(i.into_serialize_deserialize_version())
//             }
//             SixErrorEnum::Another(i) => SixErrorEnumWithDeserialize::Another(i.to_string()),
//             SixErrorEnum::AnotherVec(i) => {
//                 //todo - display implemented or not
//                 // let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
//                 //     let stringified_element =
//                 //         element.lines().fold(String::from(""), |mut acc, line| {
//                 //             acc.push_str(&format!(" {}\n", line));
//                 //             acc
//                 //         });
//                 //     acc.push_str(&stringified_element);
//                 //     acc
//                 // });
//                 // format!("[\n{}]", stringified_vec)
//                 SixErrorEnumWithDeserialize::AnotherVec(i)
//             }
//             SixErrorEnum::AnotherHashmap(i) => {
//                 //todo - display implemented or not
//                 // let stringified_hashmap =
//                 //     i.iter().fold(String::from(""), |mut acc, (key, value)| {
//                 //         let stringified_value =
//                 //             value.lines().fold(String::from(""), |mut accc, line| {
//                 //                 accc.push_str(&format!(" {}\n", line));
//                 //                 accc
//                 //             });
//                 //         acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
//                 //         acc
//                 //     });
//                 SixErrorEnumWithDeserialize::AnotherHashmap(i)
//             }
//         }
//     }
// }

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
// impl<'a> std::fmt::Display for EightError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }
// impl<'a> std::fmt::Display for EightErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }
// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for EightError<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             EightError::Something {
//                 error: _unused_first_argument,
//                 code_occurence: _unused_second_argument,
//             } => {
//                 use crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig;
//                 self.source_to_string_without_config()
//             }
//         }
//     }
// }
// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for EightError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             EightError::Something {
//                 error,
//                 code_occurence: _unused_second_argument,
//             } => {
//                 use crate::traits::display_foreign_type::DisplayForeignType;
//                 error.to_string()
//             }
//         }
//     }
// }
// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
//     for EightError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             EightError::Something {
//                 error: _unused_first_argument,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
// pub enum EightErrorWithDeserialize<'a> {
//     Something {
//         error: String,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }
// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for EightErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             EightErrorWithDeserialize::Something {
//                 error,
//                 code_occurence: _unused_second_argument,
//             } => error.to_string(),
//         }
//     }
// }
// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
//     for EightErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             EightErrorWithDeserialize::Something {
//                 error: _unused_first_argument,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }
// impl<'a> EightError<'a> {
//     pub fn into_serialize_deserialize_version(self) -> EightErrorWithDeserialize<'a> {
//         match self {
//             EightError::Something {
//                 error,
//                 code_occurence,
//             } => {
//                 use crate::traits::display_foreign_type::DisplayForeignType;
//                 EightErrorWithDeserialize::Something {
//                     error: error.to_string(),
//                     code_occurence: code_occurence.into_serialize_deserialize_version(),
//                 }
//             }
//         }
//     }
// }

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
pub struct KekwLifetimess
<
//
>
{}

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct KekwLifetime<'a> {
    s: &'a str,
}

impl<'a> crate::traits::display_foreign_type::DisplayForeignType for KekwLifetime<'a> {
    fn display_foreign_type(&self) -> String {
        String::from("kekwlifetime")
    }
}

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

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct OmegalulLifetime<'a> {
    s: &'a str,
}

impl<'a> std::fmt::Display for OmegalulLifetime<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "omegalullifetime")
    }
}
// //todo - check on lifetime - is it there or is it not - and generate/not generate it
#[derive(Debug, thiserror::Error)] //, error_occurence::ImplErrorOccurence
pub enum OneErrorEnum<'a> {
    //todo - test crate::dev::SevenError<'a> as variant but without lifetime
    // #[to_string]
    ToString(crate::dev::Omegalul),
    // #[to_string]
    ToStringLifetime(crate::dev::OmegalulLifetime<'a>),
    // #[display_foreign_type]
    DisplayForeignType(crate::dev::Kekw),
    // #[display_foreign_type]
    DisplayForeignTypeLifeTime(crate::dev::KekwLifetime<'a>),
    // #[error_occurence]
    ErrorOccurence(crate::dev::SevenError<'a>),
    // #[error_occurence]
    // ErrorOccurenceTest(crate::dev::TestErrorEnum),
    // #[vec_to_string]
    VecToString(std::vec::Vec<crate::dev::Omegalul>),
    // #[vec_display_foreign_type]
    VecDisplayForeignType(std::vec::Vec<crate::dev::Kekw>),
    // #[vec_error_occurence]
    VecErrorOccurence(std::vec::Vec<crate::dev::SevenError<'a>>),
    // #[hashmap_key_to_string_value_to_string]
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
}

impl <'error_occurence_proc_macro_reserved_lifetime_name, 'a, ConfigGeneric,> 
    crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<'error_occurence_proc_macro_reserved_lifetime_name, ConfigGeneric,>
    for
    OneErrorEnum<'a,> 
where ConfigGeneric: 
    crate::traits::fields::GetSourcePlaceType 
    + crate::traits::fields::GetTimezone 
    + crate::traits::get_server_address::GetServerAddress,
{
    fn
    to_string_with_config_for_source_to_string_with_config(& self, config : &
    ConfigGeneric) -> String
    {
        match self
        {
            OneErrorEnum :: ToString(i) => { i.to_string() }, OneErrorEnum ::
            ToStringLifetime(i) => { i.to_string() }, OneErrorEnum ::
            DisplayForeignType(i) =>
            {
                use crate :: traits :: display_foreign_type ::
                DisplayForeignType ; i.display_foreign_type()
            }, OneErrorEnum :: DisplayForeignTypeLifeTime(i) =>
            {
                use crate :: traits :: display_foreign_type ::
                DisplayForeignType ; i.display_foreign_type()
            }, OneErrorEnum :: ErrorOccurence(i) =>
            {
                i.to_string_with_config_for_source_to_string_with_config(config)
            }, OneErrorEnum :: VecToString(i) =>
            {
                let stringified_vec =
                i.iter().fold(String :: from(""), | mut acc, element |
                {
                    let stringified_element =
                    element.to_string().lines().fold(String :: from(""), | mut
                    acc, line |
                    { acc.push_str(& format! (" {}\n", line)) ; acc }) ;
                    acc.push_str(& stringified_element) ; acc
                }) ; format! ("[\n{}]", stringified_vec)
            }, OneErrorEnum :: VecDisplayForeignType(i) =>
            {
                use crate :: traits :: display_foreign_type ::
                DisplayForeignType ; let stringified_vec =
                i.iter().fold(String :: from(""), | mut acc, element |
                {
                    let stringified_element =
                    element.display_foreign_type().lines().fold(String ::
                    from(""), | mut acc, line |
                    { acc.push_str(& format! (" {}\n", line)) ; acc }) ;
                    acc.push_str(& stringified_element) ; acc
                }) ; format! ("[\n{}]", stringified_vec)
            }, OneErrorEnum :: VecErrorOccurence(i) =>
            {
                let stringified_vec =
                i.iter().fold(String :: from(""), | mut acc, element |
                {
                    let stringified_element =
                    element.to_string_with_config_for_source_to_string_with_config(config).lines().fold(String
                    :: from(""), | mut acc, line |
                    { acc.push_str(& format! (" {}\n", line)) ; acc }) ;
                    acc.push_str(& stringified_element) ; acc
                }) ; format! ("[\n{}]", stringified_vec)
            }, OneErrorEnum :: HashMapKeyToStringValueToString(i) =>
            {
                i.iter().fold(String :: from(""), | mut acc, (key, value) |
                {
                    let stringified_value =
                    value.to_string().lines().fold(String :: from(""), | mut
                    accc, line |
                    { accc.push_str(& format! (" {}\n", line)) ; accc }) ;
                    acc.push_str(& format!
                    ("{} [\n{}]\n", key, stringified_value)) ; acc
                })
            }, OneErrorEnum :: HashMapKeyToStringValueDisplayForeignType(i) =>
            {
                use crate :: traits :: display_foreign_type ::
                DisplayForeignType ;
                i.iter().fold(String :: from(""), | mut acc, (key, value) |
                {
                    let stringified_value =
                    value.display_foreign_type().lines().fold(String ::
                    from(""), | mut accc, line |
                    { accc.push_str(& format! (" {}\n", line)) ; accc }) ;
                    acc.push_str(& format!
                    ("{} [\n{}]\n", key, stringified_value)) ; acc
                })
            }, OneErrorEnum :: HashMapKeyToStringValueErrorOccurence(i) =>
            {
                i.iter().fold(String :: from(""), | mut acc, (key, value) |
                {
                    let stringified_value =
                    value.to_string_with_config_for_source_to_string_with_config(config).lines().fold(String
                    :: from(""), | mut accc, line |
                    { accc.push_str(& format! (" {}\n", line)) ; accc }) ;
                    acc.push_str(& format!
                    ("{} [\n{}]\n", key, stringified_value)) ; acc
                })
            }, OneErrorEnum :: HashMapKeyDisplayForeignTypeValueToString(i) =>
            {
                use crate :: traits :: display_foreign_type ::
                DisplayForeignType ;
                i.iter().fold(String :: from(""), | mut acc, (key, value) |
                {
                    let stringified_value =
                    value.to_string().lines().fold(String :: from(""), | mut
                    accc, line |
                    { accc.push_str(& format! (" {}\n", line)) ; accc }) ;
                    acc.push_str(& format!
                    ("{} [\n{}]\n", key.display_foreign_type(),
                    stringified_value)) ; acc
                })
            }, OneErrorEnum ::
            HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) =>
            {
                use crate :: traits :: display_foreign_type ::
                DisplayForeignType ;
                i.iter().fold(String :: from(""), | mut acc, (key, value) |
                {
                    let stringified_value =
                    value.display_foreign_type().lines().fold(String ::
                    from(""), | mut accc, line |
                    { accc.push_str(& format! (" {}\n", line)) ; accc }) ;
                    acc.push_str(& format!
                    ("{} [\n{}]\n", key.display_foreign_type(),
                    stringified_value)) ; acc
                })
            }, OneErrorEnum ::
            HashMapKeyDisplayForeignTypeValueErrorOccurence(i) =>
            {
                use crate :: traits :: display_foreign_type ::
                DisplayForeignType ;
                i.iter().fold(String :: from(""), | mut acc, (key, value) |
                {
                    let stringified_value =
                    value.to_string_with_config_for_source_to_string_with_config(config).lines().fold(String
                    :: from(""), | mut accc, line |
                    { accc.push_str(& format! (" {}\n", line)) ; accc }) ;
                    acc.push_str(& format!
                    ("{} [\n{}]\n", key.display_foreign_type(),
                    stringified_value)) ; acc
                })
            }
        }
    }
}
impl<'error_occurence_proc_macro_reserved_lifetime_name, 'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<
        'error_occurence_proc_macro_reserved_lifetime_name,
    > for OneErrorEnum<'a>
{
    fn to_string_without_config(&self) -> String {
        match self {
            OneErrorEnum::ToString(i) => i.to_string(),
            OneErrorEnum::ToStringLifetime(i) => i.to_string(),
            OneErrorEnum::DisplayForeignType(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.display_foreign_type()
            }
            OneErrorEnum::DisplayForeignTypeLifeTime(i) => {
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
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum OneErrorEnumWithDeserialize<'a> {
    ToString(crate::dev::Omegalul),
    #[serde(borrow)]
    ToStringLifetime(crate::dev::OmegalulLifetime<'a>),
    DisplayForeignType(String),
    DisplayForeignTypeLifeTime(String),
    #[serde(borrow)]
    ErrorOccurence(crate::dev::SevenErrorWithDeserialize<'a>),
    VecToString(std::vec::Vec<crate::dev::Omegalul>),
    VecDisplayForeignType(std::vec::Vec<String>),
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
    HashMapKeyDisplayForeignTypeValueErrorOccurence(
        std::collections::HashMap<String, crate::dev::SevenErrorWithDeserialize<'a>>,
    ),
}
impl<'error_occurence_proc_macro_reserved_lifetime_name, 'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
        'error_occurence_proc_macro_reserved_lifetime_name,
    > for OneErrorEnumWithDeserialize<'a>
{
    fn to_string_without_config_with_deserialize(&self) -> String {
        match self {
            OneErrorEnumWithDeserialize::ToString(i) => i.to_string(),
            OneErrorEnumWithDeserialize::ToStringLifetime(i) => i.to_string(),
            OneErrorEnumWithDeserialize::DisplayForeignType(i) => i.to_string(),
            OneErrorEnumWithDeserialize::DisplayForeignTypeLifeTime(i) => i.to_string(),
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
            }
        }
    }
}
impl<'a> OneErrorEnum<'a> {
    pub fn into_serialize_deserialize_version(self) -> OneErrorEnumWithDeserialize<'a> {
        match self {
            OneErrorEnum::ToString(i) => OneErrorEnumWithDeserialize::ToString(i),
            OneErrorEnum::ToStringLifetime(i) => OneErrorEnumWithDeserialize::ToStringLifetime(i),
            OneErrorEnum::DisplayForeignType(i) => {
                OneErrorEnumWithDeserialize::DisplayForeignType({
                    use crate::traits::display_foreign_type::DisplayForeignType;
                    i.display_foreign_type()
                })
            }
            OneErrorEnum::DisplayForeignTypeLifeTime(i) => {
                OneErrorEnumWithDeserialize::DisplayForeignTypeLifeTime({
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
            }
        }
    }
}
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

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)] //, thiserror::Error, error_occurence::ImplErrorOccurence
pub enum TestErrorEnum {
    //todo - test crate::dev::SevenError<'a> as variant but without lifetime
    #[to_string]
    ToString(crate::dev::Omegalul),
    // #[to_string]
    // ToStringLifetime(crate::dev::OmegalulLifetime<'a>),
}

#[derive(Debug)] //, error_occurence::ImplErrorOccurence
pub enum NamedError<'a> {
    Something {
        // #[to_string]
        a: crate::dev::Omegalul,
        // #[to_string]
        b: crate::dev::OmegalulLifetime<'a>,
        // #[display_foreign_type]
        c: crate::dev::Kekw,
        // #[display_foreign_type]
        d: crate::dev::KekwLifetime<'a>,
        // #[error_occurence]
        e: crate::dev::SevenError<'a>,
        // #[vec_to_string]
        f: std::vec::Vec<crate::dev::Omegalul>,
        // #[vec_to_string]
        g: std::vec::Vec<crate::dev::OmegalulLifetime<'a>>,
        // #[vec_display_foreign_type]
        h: std::vec::Vec<crate::dev::Kekw>,
        // #[vec_display_foreign_type]
        j: std::vec::Vec<crate::dev::KekwLifetime<'a>>,
        // #[vec_error_occurence]
        k: std::vec::Vec<crate::dev::SevenError<'a>>,
        // #[hashmap_key_to_string_value_to_string]
        l: std::collections::HashMap<crate::dev::Omegalul, crate::dev::Omegalul>,
        // #[hashmap_key_to_string_value_to_string]
        m: std::collections::HashMap<crate::dev::Omegalul, crate::dev::OmegalulLifetime<'a>>,
        // #[hashmap_key_to_string_value_to_string]
        n: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::Omegalul>,
        // #[hashmap_key_to_string_value_to_string]
        o: std::collections::HashMap<
            crate::dev::OmegalulLifetime<'a>,
            crate::dev::OmegalulLifetime<'a>,
        >,
        // #[hashmap_key_to_string_value_display_foreign_type]
        p: std::collections::HashMap<crate::dev::Omegalul, crate::dev::Kekw>,
        // #[hashmap_key_to_string_value_display_foreign_type]
        q: std::collections::HashMap<crate::dev::Omegalul, crate::dev::KekwLifetime<'a>>,
        // #[hashmap_key_to_string_value_display_foreign_type]
        r: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::Kekw>,
        // #[hashmap_key_to_string_value_display_foreign_type]
        s: std::collections::HashMap<
            crate::dev::OmegalulLifetime<'a>,
            crate::dev::KekwLifetime<'a>,
        >,
        // #[hashmap_key_to_string_value_error_occurence]
        t: std::collections::HashMap<crate::dev::Omegalul, crate::dev::SevenError<'a>>,
        // #[hashmap_key_to_string_value_error_occurence]
        u: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::SevenError<'a>>,
        // #[hashmap_key_display_foreign_type_value_to_string]
        v: std::collections::HashMap<crate::dev::Kekw, crate::dev::Omegalul>,
        // #[hashmap_key_display_foreign_type_value_to_string]
        w: std::collections::HashMap<
            crate::dev::Kekw,
            crate::dev::OmegalulLifetime<'a>,
        >,
        // #[hashmap_key_display_foreign_type_value_to_string]
        x: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::Omegalul>,
        // #[hashmap_key_display_foreign_type_value_to_string]
        y: std::collections::HashMap<
            crate::dev::KekwLifetime<'a>,
            crate::dev::OmegalulLifetime<'a>,
        >,
        // #[hashmap_key_display_foreign_type_value_display_foreign_type]
        z: std::collections::HashMap<crate::dev::Kekw, crate::dev::Kekw>,
        // #[hashmap_key_display_foreign_type_value_display_foreign_type]
        aa: std::collections::HashMap<crate::dev::Kekw, crate::dev::KekwLifetime<'a>>,
        // #[hashmap_key_display_foreign_type_value_display_foreign_type]
        ab: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::Kekw>,
        // #[hashmap_key_display_foreign_type_value_display_foreign_type]
        ac: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::KekwLifetime<'a>>,
        // #[hashmap_key_display_foreign_type_value_error_occurence]
        ad: std::collections::HashMap<crate::dev::Kekw, crate::dev::SevenError<'a>>,
        // #[hashmap_key_display_foreign_type_value_error_occurence]
        af: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::SevenError<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
        'a,
        ConfigGeneric,
    > for NamedError<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        match self {
            NamedError::Something {
                a: _unused_argument_0,
                b: _unused_argument_1,
                c: _unused_argument_2,
                d: _unused_argument_3,
                e: _unused_argument_4,
                f: _unused_argument_5,
                g: _unused_argument_6,
                h: _unused_argument_7,
                j: _unused_argument_8,
                k: _unused_argument_9,
                l: _unused_argument_10,
                m: _unused_argument_11,
                n: _unused_argument_12,
                o: _unused_argument_13,
                p: _unused_argument_14,
                q: _unused_argument_15,
                r: _unused_argument_16,
                s: _unused_argument_17,
                t: _unused_argument_18,
                u: _unused_argument_19,
                v: _unused_argument_20,
                w: _unused_argument_21,
                x: _unused_argument_22,
                y: _unused_argument_23,
                z: _unused_argument_24,
                aa: _unused_argument_25,
                ab: _unused_argument_26,
                ac: _unused_argument_27,
                ad: _unused_argument_28,
                af: _unused_argument_29,
                code_occurence: _unused_code_occurence_argument,
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
    > for NamedError<'a>
{
    fn source_to_string_without_config(&self) -> String {
        //todo FewToStringWithoutConfig
        match self {
            NamedError::Something {
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                j,
                k,
                l,
                m,
                n,
                o,
                p,
                q,
                r,
                s,
                t,
                u,
                v,
                w,
                x,
                y,
                z,
                aa,
                ab,
                ac,
                ad,
                af,
                code_occurence: _unused_code_occurence_argument,
            } => {
                let a_handle = {
                    a.to_string()
                };
                let b_handle = {
                    b.to_string()
                };
                let c_handle = {
                    use crate::traits::display_foreign_type::DisplayForeignType;
                    c.display_foreign_type()
                };
                let d_handle = {
                    use crate::traits::display_foreign_type::DisplayForeignType;
                    c.display_foreign_type()
                };
                let e_handle = {
                    use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
                    e.to_string_without_config()
                };
                let f_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    f.few_to_string_impl_display_impl_display()
                };
                let g_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    f.few_to_string_impl_display_impl_display()
                };
                let h_handle = {
                    use crate::traits::error_logs_logic::vec_display_foreign_type_to_string::VecDisplayForeignTypeToString;
                    h.vec_display_foreign_type_to_string()
                };
                let j_handle = {
                    use crate::traits::error_logs_logic::vec_display_foreign_type_to_string::VecDisplayForeignTypeToString;
                    j.vec_display_foreign_type_to_string()
                };
                let k_handle = {
                    use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
                    k.few_to_string_without_config()
                };
                let l_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    l.few_to_string_impl_display_impl_display()
                };
                let m_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    m.few_to_string_impl_display_impl_display()
                };
                let n_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    n.few_to_string_impl_display_impl_display()
                };
                let o_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    o.few_to_string_impl_display_impl_display()
                };
                let p_handle = {
                    use crate::traits::error_logs_logic::hashmap_impl_display_display_foreign_type_to_string::HashMapImplDisplayDisplayForeignTypeToString;
                    p.hashmap_impl_display_display_foreign_type_to_string()
                };
                let q_handle = {
                    use crate::traits::error_logs_logic::hashmap_impl_display_display_foreign_type_to_string::HashMapImplDisplayDisplayForeignTypeToString;
                    q.hashmap_impl_display_display_foreign_type_to_string()
                };
                let r_handle = {
                    use crate::traits::error_logs_logic::hashmap_impl_display_display_foreign_type_to_string::HashMapImplDisplayDisplayForeignTypeToString;
                    r.hashmap_impl_display_display_foreign_type_to_string()
                };
                let s_handle = {
                    use crate::traits::error_logs_logic::hashmap_impl_display_display_foreign_type_to_string::HashMapImplDisplayDisplayForeignTypeToString;
                    s.hashmap_impl_display_display_foreign_type_to_string()
                };
                let t_handle = {
                    use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
                    t.few_to_string_without_config()
                };
                let u_handle = {
                    use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
                    t.few_to_string_without_config()
                };
                let v_handle = {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_impl_display_to_string::HashMapDisplayForeignTypeImplDisplayToString;
                    v.hashmap_display_foreign_type_impl_display_to_string()
                };
                let w_handle = {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_impl_display_to_string::HashMapDisplayForeignTypeImplDisplayToString;
                    w.hashmap_display_foreign_type_impl_display_to_string()
                };
                let x_handle = {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_impl_display_to_string::HashMapDisplayForeignTypeImplDisplayToString;
                    x.hashmap_display_foreign_type_impl_display_to_string()
                };
                let y_handle = {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_impl_display_to_string::HashMapDisplayForeignTypeImplDisplayToString;
                    y.hashmap_display_foreign_type_impl_display_to_string()
                };
                let z_handle = {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
                    z.hashmap_display_foreign_type_display_foreign_type_to_string()
                };
                let aa_handle = {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
                    aa.hashmap_display_foreign_type_display_foreign_type_to_string()
                };
                let ab_handle = {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
                    ab.hashmap_display_foreign_type_display_foreign_type_to_string()
                };
                let ac_handle = {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
                    ab.hashmap_display_foreign_type_display_foreign_type_to_string()
                };
                let ad_handle = {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_to_string_without_config_to_string::HashMapDisplayForeignTypeToStringWithoutConfigToString;
                    ad.hashmap_display_foreign_type_to_string_without_config_to_string()
                };
                let af_handle = {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_to_string_without_config_to_string::HashMapDisplayForeignTypeToStringWithoutConfigToString;
                    af.hashmap_display_foreign_type_to_string_without_config_to_string()
                };
                format!(
                    "{{\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n}}",
                    a_handle,
                    b_handle,
                    c_handle,
                    d_handle,
                    e_handle,
                    f_handle,
                    g_handle,
                    h_handle,
                    j_handle,
                    k_handle,
                    l_handle,
                    m_handle,
                    n_handle,
                    o_handle,
                    p_handle,
                    q_handle,
                    r_handle,
                    s_handle,
                    t_handle,
                    u_handle,
                    v_handle,
                    w_handle,
                    x_handle,
                    y_handle,
                    z_handle,
                    aa_handle,
                    ab_handle,
                    ac_handle,
                    ad_handle,
                    af_handle,
                )
            },
        }
    }
}
impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
    for NamedError<'a>
{
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
        match self {
            NamedError::Something {
                a: _unused_argument_0,
                b: _unused_argument_1,
                c: _unused_argument_2,
                d: _unused_argument_3,
                e: _unused_argument_4,
                f: _unused_argument_5,
                g: _unused_argument_6,
                h: _unused_argument_7,
                j: _unused_argument_8,
                k: _unused_argument_9,
                l: _unused_argument_10,
                m: _unused_argument_11,
                n: _unused_argument_12,
                o: _unused_argument_13,
                p: _unused_argument_14,
                q: _unused_argument_15,
                r: _unused_argument_16,
                s: _unused_argument_17,
                t: _unused_argument_18,
                u: _unused_argument_19,
                v: _unused_argument_20,
                w: _unused_argument_21,
                x: _unused_argument_22,
                y: _unused_argument_23,
                z: _unused_argument_24,
                aa: _unused_argument_25,
                ab: _unused_argument_26,
                ac: _unused_argument_27,
                ad: _unused_argument_28,
                af: _unused_argument_29,
                code_occurence,
            } => code_occurence,
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum NamedErrorWithDeserialize<'a> {
    Something {
        a: crate::dev::Omegalul,
        #[serde(borrow)]
        b: crate::dev::OmegalulLifetime<'a>,
        c: std::string::String,
        d: std::string::String,
        #[serde(borrow)]
        e: crate::dev::SevenErrorWithDeserialize<'a>,
        f: std::vec::Vec<crate::dev::Omegalul>,
        #[serde(borrow)]
        g: std::vec::Vec<crate::dev::OmegalulLifetime<'a>>,
        h: std::vec::Vec<std::string::String>,
        j: std::vec::Vec<std::string::String>,
        #[serde(borrow)]
        k: std::vec::Vec<crate::dev::SevenErrorWithDeserialize<'a>>,
        l: std::collections::HashMap<crate::dev::Omegalul, crate::dev::Omegalul>,
        #[serde(borrow)]
        m: std::collections::HashMap<crate::dev::Omegalul, crate::dev::OmegalulLifetime<'a>>,
        #[serde(borrow)]
        n: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::Omegalul>,
        #[serde(borrow)]
        o: std::collections::HashMap<
            crate::dev::OmegalulLifetime<'a>,
            crate::dev::OmegalulLifetime<'a>,
        >,
        p: std::collections::HashMap<crate::dev::Omegalul, std::string::String>,
        q: std::collections::HashMap<crate::dev::Omegalul, std::string::String>,
        #[serde(borrow)]
        r: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, std::string::String>,
        #[serde(borrow)]
        s: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, std::string::String>,
        #[serde(borrow)]
        t: std::collections::HashMap<
            crate::dev::Omegalul,
            crate::dev::SevenErrorWithDeserialize<'a>,
        >,
        #[serde(borrow)]
        u: std::collections::HashMap<
            crate::dev::OmegalulLifetime<'a>,
            crate::dev::SevenErrorWithDeserialize<'a>,
        >,
        v: std::collections::HashMap<std::string::String, crate::dev::Omegalul>,
        #[serde(borrow)]
        w: std::collections::HashMap<std::string::String, crate::dev::OmegalulLifetime<'a>>,
        x: std::collections::HashMap<std::string::String, crate::dev::Omegalul>,
        #[serde(borrow)]
        y: std::collections::HashMap<std::string::String, crate::dev::OmegalulLifetime<'a>>,
        z: std::collections::HashMap<std::string::String, std::string::String>,
        aa: std::collections::HashMap<std::string::String, std::string::String>,
        ab: std::collections::HashMap<std::string::String, std::string::String>,
        ac: std::collections::HashMap<std::string::String, std::string::String>,
        #[serde(borrow)]
        ad: std::collections::HashMap<
            std::string::String,
            crate::dev::SevenErrorWithDeserialize<'a>,
        >,
        #[serde(borrow)]
        af: std::collections::HashMap<
            std::string::String,
            crate::dev::SevenErrorWithDeserialize<'a>,
        >,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
    },
}
impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
        'a,
    > for NamedErrorWithDeserialize<'a>
{
    fn source_to_string_without_config(&self) -> String {
        match self {
            NamedErrorWithDeserialize::Something {
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                j,
                k,
                l,
                m,
                n,
                o,
                p,
                q,
                r,
                s,
                t,
                u,
                v,
                w,
                x,
                y,
                z,
                aa,
                ab,
                ac,
                ad,
                af,
                code_occurence: _unused_code_occurence_argument,
            } => {
                let a_handle = {
                    a.to_string()
                };
                let b_handle = {
                    b.to_string()
                };
                let c_handle = {
                    c.to_string()
                };
                let d_handle = {
                    c.to_string()
                };
                let e_handle = {
                    use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
                    e.to_string_without_config_with_deserialize()
                };
                let f_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    f.few_to_string_impl_display_impl_display()
                };
                let g_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    g.few_to_string_impl_display_impl_display()
                };
                let h_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    h.few_to_string_impl_display_impl_display()
                };
                let j_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    j.few_to_string_impl_display_impl_display()
                };
                let k_handle = {
                    use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigWithDeserialize;
                    k.few_to_string_without_config_with_deserialize()
                };
                let l_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    l.few_to_string_impl_display_impl_display()
                };
                let m_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    m.few_to_string_impl_display_impl_display()
                };
                let n_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    n.few_to_string_impl_display_impl_display()
                };
                let o_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    o.few_to_string_impl_display_impl_display()
                };
                let p_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    p.few_to_string_impl_display_impl_display()
                };
                let q_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    q.few_to_string_impl_display_impl_display()
                };
                let r_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    r.few_to_string_impl_display_impl_display()
                };
                let s_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    f.few_to_string_impl_display_impl_display()
                };
                let t_handle = {
                    use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigWithDeserialize;
                    t.few_to_string_without_config_with_deserialize()
                };
                let u_handle = {
                    use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigWithDeserialize;
                    u.few_to_string_without_config_with_deserialize()
                };
                let v_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    v.few_to_string_impl_display_impl_display()
                };
                let w_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    w.few_to_string_impl_display_impl_display()
                };
                let x_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    x.few_to_string_impl_display_impl_display()
                };
                let y_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    y.few_to_string_impl_display_impl_display()
                };
                let z_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    z.few_to_string_impl_display_impl_display()
                };
                let aa_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    aa.few_to_string_impl_display_impl_display()
                };
                let ab_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    ab.few_to_string_impl_display_impl_display()
                };
                let ac_handle = {
                    use crate::traits::error_logs_logic::few_to_string_impl_display_impl_display::FewToStringImplDisplayImplDisplay;
                    ac.few_to_string_impl_display_impl_display()
                };
                let ad_handle = {
                    use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigWithDeserialize;
                    ad.few_to_string_without_config_with_deserialize()
                };
                let af_handle = {
                    use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigWithDeserialize;
                    ad.few_to_string_without_config_with_deserialize()
                };
                format!(
                    "{{\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n}}",
                    a_handle,
                    b_handle,
                    c_handle,
                    d_handle,
                    e_handle,
                    f_handle,
                    g_handle,
                    h_handle,
                    j_handle,
                    k_handle,
                    l_handle,
                    m_handle,
                    n_handle,
                    o_handle,
                    p_handle,
                    q_handle,
                    r_handle,
                    s_handle,
                    t_handle,
                    u_handle,
                    v_handle,
                    w_handle,
                    x_handle,
                    y_handle,
                    z_handle,
                    aa_handle,
                    ab_handle,
                    ac_handle,
                    ad_handle,
                    af_handle,
                )
            },
        }
    }
}
impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
    for NamedErrorWithDeserialize<'a>
{
    fn get_code_occurence_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
        match self {
            NamedErrorWithDeserialize::Something {
                a: _unused_argument_0,
                b: _unused_argument_1,
                c: _unused_argument_2,
                d: _unused_argument_3,
                e: _unused_argument_4,
                f: _unused_argument_5,
                g: _unused_argument_6,
                h: _unused_argument_7,
                j: _unused_argument_8,
                k: _unused_argument_9,
                l: _unused_argument_10,
                m: _unused_argument_11,
                n: _unused_argument_12,
                o: _unused_argument_13,
                p: _unused_argument_14,
                q: _unused_argument_15,
                r: _unused_argument_16,
                s: _unused_argument_17,
                t: _unused_argument_18,
                u: _unused_argument_19,
                v: _unused_argument_20,
                w: _unused_argument_21,
                x: _unused_argument_22,
                y: _unused_argument_23,
                z: _unused_argument_24,
                aa: _unused_argument_25,
                ab: _unused_argument_26,
                ac: _unused_argument_27,
                ad: _unused_argument_28,
                af: _unused_argument_29,
                code_occurence,
            } => code_occurence,
        }
    }
}
impl<'a> NamedError<'a> {
    pub fn into_serialize_deserialize_version(self) -> NamedErrorWithDeserialize<'a> {
        match self {
            NamedError::Something {
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                j,
                k,
                l,
                m,
                n,
                o,
                p,
                q,
                r,
                s,
                t,
                u,
                v,
                w,
                x,
                y,
                z,
                aa,
                ab,
                ac,
                ad,
                af,
                code_occurence,
            } => NamedErrorWithDeserialize::Something {
                a: {
                    a
                },
                b: {
                    b
                },
                c: {
                    use crate::traits::display_foreign_type::DisplayForeignType;
                    c.display_foreign_type()
                },
                d: {
                    use crate::traits::display_foreign_type::DisplayForeignType;
                    d.display_foreign_type()
                },
                e: {
                    e.into_serialize_deserialize_version()
                },
                f: {
                    f
                },
                g: {
                    g
                },
                h: {
                    use crate::traits::error_logs_logic::vec_display_foreign_type_to_vec_string::VecDisplayForeignTypeToVecString;
                    h.vec_display_foreign_type_to_vec_string()
                },
                j: {
                    use crate::traits::error_logs_logic::vec_display_foreign_type_to_vec_string::VecDisplayForeignTypeToVecString;
                    j.vec_display_foreign_type_to_vec_string()
                },
                k: {
                    k.into_iter().map(|i|{
                        i.into_serialize_deserialize_version()
                    })
                    .collect()
                },
                l: {
                    l
                },
                m: {
                    m
                },
                n: {
                    n
                },
                o: {
                    o
                },
                p: {
                    p.into_iter().map(|(k, v)|{
                        (
                            k, 
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                v.display_foreign_type()
                            }
                        )
                    })
                    .collect()
                },
                q: {
                    q.into_iter().map(|(k, v)|{
                        (
                            k, 
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                v.display_foreign_type()
                            }
                        )
                    })
                    .collect()
                },
                r: {
                    r.into_iter().map(|(k, v)|{
                        (
                            k, 
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                v.display_foreign_type()
                            }
                        )
                    })
                    .collect()
                },
                s: {
                    s.into_iter().map(|(k, v)|{
                        (
                            k, 
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                v.display_foreign_type()
                            }
                        )
                    })
                    .collect()
                },
                t: {
                    t.into_iter().map(|(k,v)|{
                        (
                            k,
                            {
                                v.into_serialize_deserialize_version()
                            }
                        )
                    })
                    .collect()
                },
                u: {
                    u.into_iter().map(|(k,v)|{
                        (
                            k,
                            {
                                v.into_serialize_deserialize_version()
                            }
                        )
                    })
                    .collect()
                },
                v: {
                   v.into_iter().map(|(k, v)|{
                        (
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                k.display_foreign_type()
                            },
                            v 
                        )
                    })
                    .collect()
                },
                w: {
                   w.into_iter().map(|(k, v)|{
                        (
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                k.display_foreign_type()
                            },
                            v 
                        )
                    })
                    .collect()
                },
                x: {
                   x.into_iter().map(|(k, v)|{
                        (
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                k.display_foreign_type()
                            },
                            v 
                        )
                    })
                    .collect()
                },
                y: {
                   y.into_iter().map(|(k, v)|{
                        (
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                k.display_foreign_type()
                            },
                            v 
                        )
                    })
                    .collect()
                },
                z: {
                   z.into_iter().map(|(k, v)|{
                        (
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                k.display_foreign_type()
                            },
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                v.display_foreign_type()
                            },
                        )
                    })
                    .collect()
                },
                aa: {
                   aa.into_iter().map(|(k, v)|{
                        (
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                k.display_foreign_type()
                            },
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                v.display_foreign_type()
                            },
                        )
                    })
                    .collect()
                },
                ab: {
                   ab.into_iter().map(|(k, v)|{
                        (
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                k.display_foreign_type()
                            },
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                v.display_foreign_type()
                            },
                        )
                    })
                    .collect()
                },
                ac: {
                   ac.into_iter().map(|(k, v)|{
                        (
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                k.display_foreign_type()
                            },
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                v.display_foreign_type()
                            },
                        )
                    })
                    .collect()
                },
                ad: {
                   ad.into_iter().map(|(k, v)|{
                        (
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                k.display_foreign_type()
                            },
                            {
                                v.into_serialize_deserialize_version()
                            }
                        )
                    })
                    .collect()
                },
                af: {
                   af.into_iter().map(|(k, v)|{
                        (
                            {
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                k.display_foreign_type()
                            },
                            {
                                v.into_serialize_deserialize_version()
                            }
                        )
                    })
                    .collect()
                },
                code_occurence: code_occurence.into_serialize_deserialize_version(),
            },
        }
    }
}
impl<'a> std::fmt::Display for NamedError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
        write!(f, "{}", self.to_string_without_config())
    }
}
impl<'a> std::fmt::Display for NamedErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
        write!(f, "{}", self.to_string_without_config_with_deserialize())
    }
}
