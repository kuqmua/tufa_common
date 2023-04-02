use crate::traits::{
    display_foreign_type,
    error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize,
};

pub fn dev() {
    if let Err(e) = named() {
        //todo - this actually must be a config struct function, not an error - config.error_log(e)
        println!("{}", *e);
        use crate::traits::error_logs_logic::error_log::ErrorLog;
        e.error_log(once_cell::sync::Lazy::force(
            //todo - this must be call once on start of the program
            &crate::global_variables::runtime::config::CONFIG,
        ));
        let ed = e.into_serialize_deserialize_version();
        println!("{ed}");
        // ed.error_log(once_cell::sync::Lazy::force(
        //     //todo - this must be call once on start of the program
        //     &crate::global_variables::runtime::config::CONFIG,
        // ));
    }
}

pub fn named<'a>() -> Result<(), Box<NamedError<'a>>> {
    return Err(Box::new(NamedError::Something {
        a: crate::dev::Omegalul {},
        b: crate::dev::OmegalulLifetime {
            s: "omegalullifetime",
        },
        c: crate::dev::Kekw {},
        d: crate::dev::KekwLifetime { s: "kekwlifetime" },
        e: crate::dev::SevenError::Something {
            error: String::from("seven_error"),
            code_occurence: crate::code_occurence_tufa_common!(),
        },
        f: vec![crate::dev::Omegalul {}],
        g: vec![crate::dev::OmegalulLifetime {
            s: "omegalullifetime",
        }],
        h: vec![crate::dev::Kekw {}],
        j: vec![crate::dev::KekwLifetime { s: "kekwlifetime" }],
        k: vec![crate::dev::SevenError::Something {
            error: String::from("seven_error"),
            code_occurence: crate::code_occurence_tufa_common!(),
        }],
        l: std::collections::HashMap::from([(crate::dev::Omegalul {}, crate::dev::Omegalul {})]),
        m: std::collections::HashMap::from([(
            crate::dev::Omegalul {},
            crate::dev::OmegalulLifetime {
                s: "omegalullifetime",
            },
        )]),
        n: std::collections::HashMap::from([(
            crate::dev::OmegalulLifetime {
                s: "omegalullifetime",
            },
            crate::dev::Omegalul {},
        )]),
        o: std::collections::HashMap::from([(
            crate::dev::OmegalulLifetime {
                s: "omegalullifetime",
            },
            crate::dev::OmegalulLifetime {
                s: "omegalullifetime",
            },
        )]),
        p: std::collections::HashMap::from([(crate::dev::Omegalul {}, crate::dev::Kekw {})]),
        q: std::collections::HashMap::from([(
            crate::dev::Omegalul {},
            crate::dev::KekwLifetime { s: "kekwlifetime" },
        )]),
        r: std::collections::HashMap::from([(
            crate::dev::OmegalulLifetime {
                s: "omegalullifetime",
            },
            crate::dev::Kekw {},
        )]),
        s: std::collections::HashMap::from([(
            crate::dev::OmegalulLifetime {
                s: "omegalullifetime",
            },
            crate::dev::KekwLifetime { s: "kekwlifetime" },
        )]),
        t: std::collections::HashMap::from([(
            crate::dev::Omegalul {},
            crate::dev::SevenError::Something {
                error: String::from("seven_error"),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
        )]),
        u: std::collections::HashMap::from([(
            crate::dev::OmegalulLifetime {
                s: "omegalullifetime",
            },
            crate::dev::SevenError::Something {
                error: String::from("seven_error"),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
        )]),
        v: std::collections::HashMap::from([(crate::dev::Kekw {}, crate::dev::Omegalul {})]),
        w: std::collections::HashMap::from([(
            crate::dev::Kekw {},
            crate::dev::OmegalulLifetime {
                s: "omegalullifetime",
            },
        )]),
        x: std::collections::HashMap::from([(
            crate::dev::KekwLifetime { s: "kekwlifetime" },
            crate::dev::Omegalul {},
        )]),
        y: std::collections::HashMap::from([(
            crate::dev::KekwLifetime { s: "kekwlifetime" },
            crate::dev::OmegalulLifetime {
                s: "omegalullifetime",
            },
        )]),
        z: std::collections::HashMap::from([(crate::dev::Kekw {}, crate::dev::Kekw {})]),
        aa: std::collections::HashMap::from([(
            crate::dev::Kekw {},
            crate::dev::KekwLifetime { s: "kekwlifetime" },
        )]),
        ab: std::collections::HashMap::from([(
            crate::dev::KekwLifetime { s: "kekwlifetime" },
            crate::dev::Kekw {},
        )]),
        ac: std::collections::HashMap::from([(
            crate::dev::KekwLifetime { s: "kekwlifetime" },
            crate::dev::KekwLifetime { s: "kekwlifetime" },
        )]),
        ad: std::collections::HashMap::from([(
            crate::dev::Kekw {},
            crate::dev::SevenError::Something {
                error: String::from("seven_error"),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
        )]),
        af: std::collections::HashMap::from([(
            crate::dev::KekwLifetime { s: "kekwlifetime" },
            crate::dev::SevenError::Something {
                error: String::from("seven_error"),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
        )]),
        // ag: vec![OneErrorEnum::HashMapKeyToStringValueToString(
        //     std::collections::HashMap::from([(crate::dev::Omegalul {}, crate::dev::Omegalul {})]),
        // )],
        code_occurence: crate::code_occurence_tufa_common!(),
    }));
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

#[derive(Debug, thiserror::Error, serde :: Serialize)] //, error_occurence::ImplErrorOccurence
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
                use crate::traits::error_logs_logic::vec_to_string_with_config_to_string::VecToStringWithConfigToString;
                inner_errors.vec_to_string_with_config_to_string(config)
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
                use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToString;
                inner_errors.vec_to_string_without_config_to_string()
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
                use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToStringWithDeserialize;
                inner_errors.vec_to_string_without_config_to_string_with_deserialize()
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
    fn display_foreign_type(&self) -> &'static str {
        "kekwlifetime"
    }
}

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Kekw {}

impl crate::traits::display_foreign_type::DisplayForeignType for Kekw {
    fn display_foreign_type(&self) -> &'static str {
        "kekw"
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

impl < 'error_occurence_proc_macro_reserved_lifetime_name, 'a, ConfigGeneric,
> crate :: traits :: error_logs_logic :: to_string_with_config ::
ToStringWithConfigForSourceToStringWithConfig <
'error_occurence_proc_macro_reserved_lifetime_name, ConfigGeneric, > for
OneErrorEnum < 'a, > where ConfigGeneric : crate :: traits :: fields ::
GetSourcePlaceType + crate :: traits :: fields :: GetTimezone + crate ::
traits :: get_server_address :: GetServerAddress,
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
                DisplayForeignType ; i.display_foreign_type().to_string()
            }, OneErrorEnum :: DisplayForeignTypeLifeTime(i) =>
            {
                use crate :: traits :: display_foreign_type ::
                DisplayForeignType ; i.display_foreign_type().to_string()
            }, OneErrorEnum :: ErrorOccurence(i) =>
            {
                i.to_string_with_config_for_source_to_string_with_config(config)
            }, OneErrorEnum :: VecToString(i) =>
            {
                use crate :: traits :: error_logs_logic ::
                vec_display_to_string :: VecDisplayToString ;
                i.vec_display_to_string()
            }, OneErrorEnum :: VecDisplayForeignType(i) =>
            {
                use crate :: traits :: error_logs_logic ::
                vec_display_foreign_type_to_string ::
                VecDisplayForeignTypeToString ;
                i.vec_display_foreign_type_to_string()
            }, OneErrorEnum :: VecErrorOccurence(i) =>
            {
                use crate :: traits :: error_logs_logic ::
                vec_to_string_with_config_to_string ::
                VecToStringWithConfigToString ;
                i.vec_to_string_with_config_to_string(config)
            }, OneErrorEnum :: HashMapKeyToStringValueToString(i) =>
            {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_display_to_string ::
                HashmapDisplayDisplayToString ;
                i.hashmap_display_display_to_string()
            }, OneErrorEnum :: HashMapKeyToStringValueDisplayForeignType(i) =>
            {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_display_foreign_type_to_string ::
                HashMapDisplayDisplayForeignTypeToString ;
                i.hashmap_display_display_foreign_type_to_string()
            }, OneErrorEnum :: HashMapKeyToStringValueErrorOccurence(i) =>
            {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_to_string_with_config_to_string ::
                HashMapDisplayToStringWithConfigToString ;
                i.hashmap_display_to_string_with_config_to_string(config)
            }, OneErrorEnum :: HashMapKeyDisplayForeignTypeValueToString(i) =>
            {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_foreign_type_display_to_string ::
                HashMapDisplayForeignTypeDisplayToString ;
                i.hashmap_display_foreign_type_display_to_string()
            }, OneErrorEnum ::
            HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) =>
            {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_foreign_type_display_foreign_type_to_string ::
                HashMapDisplayForeignTypeDisplayForeignTypeToString ;
                i.hashmap_display_foreign_type_display_foreign_type_to_string()
            }, OneErrorEnum ::
            HashMapKeyDisplayForeignTypeValueErrorOccurence(i) =>
            {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_foreign_type_to_string_with_config_to_string
                :: HashMapDisplayForeignTypeToStringWithConfigToString ;
                i.hashmap_display_foreign_type_to_string_with_config_to_string(config)
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
                i.display_foreign_type().to_string()
            }
            OneErrorEnum::DisplayForeignTypeLifeTime(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.display_foreign_type().to_string()
            }
            OneErrorEnum::ErrorOccurence(i) => i.to_string_without_config(),
            OneErrorEnum::VecToString(i) => {
                use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                i.vec_display_to_string()
            }
            OneErrorEnum::VecDisplayForeignType(i) => {
                use crate::traits::error_logs_logic::vec_display_foreign_type_to_string::VecDisplayForeignTypeToString;
                i.vec_display_foreign_type_to_string()
            }
            OneErrorEnum::VecErrorOccurence(i) => {
                use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToString;
                i.vec_to_string_without_config_to_string()
            }
            OneErrorEnum::HashMapKeyToStringValueToString(i) => {
                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                i.hashmap_display_display_to_string()
            }
            OneErrorEnum::HashMapKeyToStringValueDisplayForeignType(i) => {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_display_foreign_type_to_string ::
                HashMapDisplayDisplayForeignTypeToString ;
                i.hashmap_display_display_foreign_type_to_string()
            }
            OneErrorEnum::HashMapKeyToStringValueErrorOccurence(i) => {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_to_string_without_config_to_string ::
                HashmapDisplayToStringWithoutConfigToString ;
                i.hashmap_display_to_string_without_config_to_string()
            }
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueToString(i) => {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_foreign_type_display_to_string ::
                HashMapDisplayForeignTypeDisplayToString ;
                i.hashmap_display_foreign_type_display_to_string()
            }
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) => {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_foreign_type_display_foreign_type_to_string ::
                HashMapDisplayForeignTypeDisplayForeignTypeToString ;
                i.hashmap_display_foreign_type_display_foreign_type_to_string()
            }
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueErrorOccurence(i) => {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_foreign_type_to_string_without_config_to_string
                :: HashMapDisplayForeignTypeToStringWithoutConfigToString ;
                i.hashmap_display_foreign_type_to_string_without_config_to_string()
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum OneErrorEnumWithDeserialize<'a> {
    ToString(crate::dev::Omegalul),
    #[serde(borrow)]
    ToStringLifetime(crate::dev::OmegalulLifetime<'a>),
    DisplayForeignType(&'static str),
    DisplayForeignTypeLifeTime(&'static str),
    #[serde(borrow)]
    ErrorOccurence(crate::dev::SevenErrorWithDeserialize<'a>),
    VecToString(std::vec::Vec<crate::dev::Omegalul>),
    VecDisplayForeignType(std::vec::Vec<&'static str>),
    VecErrorOccurence(std::vec::Vec<crate::dev::SevenErrorWithDeserialize<'a>>),
    HashMapKeyToStringValueToString(
        std::collections::HashMap<crate::dev::Omegalul, crate::dev::Omegalul>,
    ),
    HashMapKeyToStringValueDisplayForeignType(
        std::collections::HashMap<crate::dev::Omegalul, &'static str>,
    ),
    HashMapKeyToStringValueErrorOccurence(
        std::collections::HashMap<crate::dev::Omegalul, crate::dev::SevenErrorWithDeserialize<'a>>,
    ),
    HashMapKeyDisplayForeignTypeValueToString(
        std::collections::HashMap<&'static str, crate::dev::Omegalul>,
    ),
    HashMapKeyDisplayForeignTypeValueDisplayForeignType(
        std::collections::HashMap<&'static str, &'static str>,
    ),
    HashMapKeyDisplayForeignTypeValueErrorOccurence(
        std::collections::HashMap<&'static str, crate::dev::SevenErrorWithDeserialize<'a>>,
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
                use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                i.vec_display_to_string()
            }
            OneErrorEnumWithDeserialize::VecDisplayForeignType(i) => {
                use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                i.vec_display_to_string()
            }
            OneErrorEnumWithDeserialize::VecErrorOccurence(i) => {
                use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToStringWithDeserialize;
                i.vec_to_string_without_config_to_string_with_deserialize()
            }
            OneErrorEnumWithDeserialize::HashMapKeyToStringValueToString(i) => {
                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                i.hashmap_display_display_to_string()
            }
            OneErrorEnumWithDeserialize::HashMapKeyToStringValueDisplayForeignType(i) => {
                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                i.hashmap_display_display_to_string()
            }
            OneErrorEnumWithDeserialize::HashMapKeyToStringValueErrorOccurence(i) => {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_to_string_without_config_to_string ::
                HashmapDisplayToStringWithoutConfigToStringWithDeserialize ;
                i.hashmap_display_to_string_without_config_to_string_with_deserialize()
            }
            OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueToString(i) => {
                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                i.hashmap_display_display_to_string()
            }
            OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) => {
                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                i.hashmap_display_display_to_string()
            }
            OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueErrorOccurence(i) => {
                use crate :: traits :: error_logs_logic ::
                hashmap_display_to_string_without_config_to_string ::
                HashmapDisplayToStringWithoutConfigToStringWithDeserialize ;
                i.hashmap_display_to_string_without_config_to_string_with_deserialize()
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
                    use crate::traits::error_logs_logic::vec_display_foreign_type_into_vec_string::VecDisplayForeignTypeIntoVecString;
                    i.vec_display_foreign_type_into_vec_string()
                })
            }
            OneErrorEnum::VecErrorOccurence(i) => OneErrorEnumWithDeserialize::VecErrorOccurence({
                i.into_iter()
                    .map(|e| e.into_serialize_deserialize_version())
                    .collect()
            }),
            OneErrorEnum::HashMapKeyToStringValueToString(i) => {
                OneErrorEnumWithDeserialize::HashMapKeyToStringValueToString(i)
            }
            OneErrorEnum::HashMapKeyToStringValueDisplayForeignType(i) => {
                OneErrorEnumWithDeserialize::HashMapKeyToStringValueDisplayForeignType({
                    use crate :: traits :: error_logs_logic ::
                    hashmap_display_display_foreign_type_into_hashmap_display_string
                    :: HashmapDisplayDisplayForeignTypeIntoHashmapDisplayString
                    ;
                    i.hashmap_display_display_foreign_type_into_hashmap_display_string()
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
                    use crate :: traits :: error_logs_logic ::
                    hashmap_display_foreign_type_display_into_hashmap_string_display
                    :: HashmapDisplayForeignTypeDisplayIntoHashMapStringDisplay
                    ;
                    i.hashmap_display_foreign_type_display_into_hashmap_string_display()
                })
            }
            OneErrorEnum::HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) => {
                OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueDisplayForeignType({
                    use crate :: traits :: error_logs_logic ::
                    hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string
                    ::
                    HashmapDisplayForeignTypeDisplayForeignTypeIntoHashMapStringString
                    ;
                    i.hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string()
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

// impl <'error_occurence_proc_macro_reserved_lifetime_name, 'a, ConfigGeneric,>
//     crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<'error_occurence_proc_macro_reserved_lifetime_name, ConfigGeneric,>
//     for
//     OneErrorEnum<'a,>
// where ConfigGeneric:
//     crate::traits::fields::GetSourcePlaceType
//     + crate::traits::fields::GetTimezone
//     + crate::traits::get_server_address::GetServerAddress,
// {
//     fn
//     to_string_with_config_for_source_to_string_with_config(& self, config : &
//     ConfigGeneric) -> String
//     {
//         match self
//         {
//             OneErrorEnum :: ToString(i) => { i.to_string() },
//             OneErrorEnum::ToStringLifetime(i) => { i.to_string() },
//             OneErrorEnum::DisplayForeignType(i) => {
//                 use crate::traits::display_foreign_type::DisplayForeignType;
//                 i.display_foreign_type()
//             },
//             OneErrorEnum :: DisplayForeignTypeLifeTime(i) => {
//                 use crate::traits::display_foreign_type::DisplayForeignType;
//                 i.display_foreign_type()
//             },
//             OneErrorEnum::ErrorOccurence(i) => {
//                 i.to_string_with_config_for_source_to_string_with_config(config)
//             },
//             OneErrorEnum::VecToString(i) => {
//                 use crate::traits::error_logs_logic::vec_impl_display_to_string::VecImplDisplayToString;
//                 i.vec_impl_display_to_string()
//             },
//             OneErrorEnum::VecDisplayForeignType(i) => {
//                 use crate::traits::error_logs_logic::vec_display_foreign_type_to_string::VecDisplayForeignTypeToString;
//                 i.vec_display_foreign_type_to_string()
//             },
//             OneErrorEnum::VecErrorOccurence(i) => {
//                 use crate::traits::error_logs_logic::vec_to_string_with_config_to_string::VecToStringWithConfigToString;
//                 i.vec_to_string_with_config_to_string(config)
//             },
//             OneErrorEnum::HashMapKeyToStringValueToString(i) => {
//                 use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
//                 i.hashmap_display_display_to_string()
//             },
//             OneErrorEnum::HashMapKeyToStringValueDisplayForeignType(i) => {
//                 use crate::traits::error_logs_logic::hashmap_impl_display_display_foreign_type_to_string::HashMapImplDisplayDisplayForeignTypeToString;
//                 i.hashmap_impl_display_display_foreign_type_to_string()
//             },
//             OneErrorEnum::HashMapKeyToStringValueErrorOccurence(i) => {
//                 use crate::traits::error_logs_logic::hashmap_impl_display_to_string_with_config_to_string::HashMapImplDisplayToStringWithConfigToString;
//                 i.hashmap_impl_display_to_string_with_config_to_string(config)
//             },
//             OneErrorEnum::HashMapKeyDisplayForeignTypeValueToString(i) => {
//                 use crate::traits::error_logs_logic::hashmap_display_foreign_type_impl_display_to_string::HashMapDisplayForeignTypeImplDisplayToString;
//                 i.hashmap_display_foreign_type_impl_display_to_string()
//             },
//             OneErrorEnum::HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) => {
//                 use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
//                 i.hashmap_display_foreign_type_display_foreign_type_to_string()
//             },
//             OneErrorEnum::HashMapKeyDisplayForeignTypeValueErrorOccurence(i) => {
//                 use crate::traits::error_logs_logic::hashmap_display_foreign_type_to_string_with_config_to_string::HashMapDisplayForeignTypeToStringWithConfigToString;
//                 i.hashmap_display_foreign_type_to_string_with_config_to_string(config)
//             }
//         }
//     }
// }
// impl<'error_occurence_proc_macro_reserved_lifetime_name, 'a>
//     crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<
//         'error_occurence_proc_macro_reserved_lifetime_name,
//     > for OneErrorEnum<'a>
// {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             OneErrorEnum::ToString(i) => i.to_string(),
//             OneErrorEnum::ToStringLifetime(i) => i.to_string(),
//             OneErrorEnum::DisplayForeignType(i) => {
//                 use crate::traits::display_foreign_type::DisplayForeignType;
//                 i.display_foreign_type()
//             }
//             OneErrorEnum::DisplayForeignTypeLifeTime(i) => {
//                 use crate::traits::display_foreign_type::DisplayForeignType;
//                 i.display_foreign_type()
//             }
//             OneErrorEnum::ErrorOccurence(i) => i.to_string_without_config(),
//             OneErrorEnum::VecToString(i) => {
//                 use crate::traits::error_logs_logic::vec_impl_display_to_string::VecImplDisplayToString;
//                 i.vec_impl_display_to_string()
//             }
//             OneErrorEnum::VecDisplayForeignType(i) => {
//                 use crate::traits::error_logs_logic::vec_display_foreign_type_to_string::VecDisplayForeignTypeToString;
//                 i.vec_display_foreign_type_to_string()
//             }
//             OneErrorEnum::VecErrorOccurence(i) => {
//                 use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToString;
//                 i.vec_to_string_without_config_to_string()
//             }
//             OneErrorEnum::HashMapKeyToStringValueToString(i) => {
//                 use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
//                 i.hashmap_display_display_to_string()
//             }
//             OneErrorEnum::HashMapKeyToStringValueDisplayForeignType(i) => {
//                 use crate::traits::error_logs_logic::hashmap_impl_display_display_foreign_type_to_string::HashMapImplDisplayDisplayForeignTypeToString;
//                 i.hashmap_impl_display_display_foreign_type_to_string()
//             }
//             OneErrorEnum::HashMapKeyToStringValueErrorOccurence(i) => {
//                 use crate::traits::error_logs_logic::hashmap_impl_display_to_string_without_config_to_string::HashmapImplDisplayToStringWithoutConfigToString;
//                 i.hashmap_impl_display_to_string_without_config_to_string()
//             }
//             OneErrorEnum::HashMapKeyDisplayForeignTypeValueToString(i) => {
//                 use crate::traits::error_logs_logic::hashmap_display_foreign_type_impl_display_to_string::HashMapDisplayForeignTypeImplDisplayToString;
//                 i.hashmap_display_foreign_type_impl_display_to_string()
//             }
//             OneErrorEnum::HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) => {
//                 use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
//                 i.hashmap_display_foreign_type_display_foreign_type_to_string()
//             }
//             OneErrorEnum::HashMapKeyDisplayForeignTypeValueErrorOccurence(i) => {
//                 use crate::traits::error_logs_logic::hashmap_display_foreign_type_to_string_without_config_to_string::HashMapDisplayForeignTypeToStringWithoutConfigToString;
//                 i.hashmap_display_foreign_type_to_string_without_config_to_string()
//             }
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
// pub enum OneErrorEnumWithDeserialize<'a> {
//     ToString(crate::dev::Omegalul),
//     #[serde(borrow)]
//     ToStringLifetime(crate::dev::OmegalulLifetime<'a>),
//     DisplayForeignType(String),
//     DisplayForeignTypeLifeTime(String),
//     #[serde(borrow)]
//     ErrorOccurence(crate::dev::SevenErrorWithDeserialize<'a>),
//     VecToString(std::vec::Vec<crate::dev::Omegalul>),
//     VecDisplayForeignType(std::vec::Vec<String>),
//     VecErrorOccurence(std::vec::Vec<crate::dev::SevenErrorWithDeserialize<'a>>),
//     HashMapKeyToStringValueToString(
//         std::collections::HashMap<crate::dev::Omegalul, crate::dev::Omegalul>,
//     ),
//     HashMapKeyToStringValueDisplayForeignType(
//         std::collections::HashMap<crate::dev::Omegalul, String>,
//     ),
//     HashMapKeyToStringValueErrorOccurence(
//         std::collections::HashMap<crate::dev::Omegalul, crate::dev::SevenErrorWithDeserialize<'a>>,
//     ),
//     HashMapKeyDisplayForeignTypeValueToString(
//         std::collections::HashMap<String, crate::dev::Omegalul>,
//     ),
//     HashMapKeyDisplayForeignTypeValueDisplayForeignType(std::collections::HashMap<String, String>),
//     HashMapKeyDisplayForeignTypeValueErrorOccurence(
//         std::collections::HashMap<String, crate::dev::SevenErrorWithDeserialize<'a>>,
//     ),
// }
// impl<'error_occurence_proc_macro_reserved_lifetime_name, 'a>
//     crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
//         'error_occurence_proc_macro_reserved_lifetime_name,
//     > for OneErrorEnumWithDeserialize<'a>
// {
//     fn to_string_without_config_with_deserialize(&self) -> String {
//         match self {
//             OneErrorEnumWithDeserialize::ToString(i) => i.to_string(),
//             OneErrorEnumWithDeserialize::ToStringLifetime(i) => i.to_string(),
//             OneErrorEnumWithDeserialize::DisplayForeignType(i) => i.to_string(),
//             OneErrorEnumWithDeserialize::DisplayForeignTypeLifeTime(i) => i.to_string(),
//             OneErrorEnumWithDeserialize::ErrorOccurence(i) => {
//                 i.to_string_without_config_with_deserialize()
//             }
//             OneErrorEnumWithDeserialize::VecToString(i) => {
//                 use crate::traits::error_logs_logic::vec_impl_display_to_string::VecImplDisplayToString;
//                 i.vec_impl_display_to_string()
//             }
//             OneErrorEnumWithDeserialize::VecDisplayForeignType(i) => {
//                 use crate::traits::error_logs_logic::vec_impl_display_to_string::VecImplDisplayToString;
//                 i.vec_impl_display_to_string()
//             }
//             OneErrorEnumWithDeserialize::VecErrorOccurence(i) => {
//                 use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToStringWithDeserialize;
//                 i.vec_to_string_without_config_to_string_with_deserialize()
//             }
//             OneErrorEnumWithDeserialize::HashMapKeyToStringValueToString(i) => {
//                 use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
//                 i.hashmap_display_display_to_string()
//             }
//             OneErrorEnumWithDeserialize::HashMapKeyToStringValueDisplayForeignType(i) => {
//                 use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
//                 i.hashmap_display_display_to_string()
//             }
//             OneErrorEnumWithDeserialize::HashMapKeyToStringValueErrorOccurence(i) => {
//                 use crate::traits::error_logs_logic::hashmap_impl_display_to_string_without_config_to_string::HashmapImplDisplayToStringWithoutConfigToStringWithDeserialize;
//                 i.hashmap_impl_display_to_string_without_config_to_string_with_deserialize()
//             }
//             OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueToString(i) => {
//                 use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
//                 i.hashmap_display_display_to_string()
//             }
//             OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) => {
//                 use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
//                 i.hashmap_display_display_to_string()
//             }
//             OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueErrorOccurence(i) => {
//                 use crate::traits::error_logs_logic::hashmap_impl_display_to_string_without_config_to_string::HashmapImplDisplayToStringWithoutConfigToStringWithDeserialize;
//                 i.hashmap_impl_display_to_string_without_config_to_string_with_deserialize()
//             }
//         }
//     }
// }
// impl<'a> OneErrorEnum<'a> {
//     pub fn into_serialize_deserialize_version(self) -> OneErrorEnumWithDeserialize<'a> {
//         match self {
//             OneErrorEnum::ToString(i) => OneErrorEnumWithDeserialize::ToString(i),
//             OneErrorEnum::ToStringLifetime(i) => OneErrorEnumWithDeserialize::ToStringLifetime(i),
//             OneErrorEnum::DisplayForeignType(i) => {
//                 OneErrorEnumWithDeserialize::DisplayForeignType({
//                     use crate::traits::display_foreign_type::DisplayForeignType;
//                     i.display_foreign_type()
//                 })
//             }
//             OneErrorEnum::DisplayForeignTypeLifeTime(i) => {
//                 OneErrorEnumWithDeserialize::DisplayForeignTypeLifeTime({
//                     use crate::traits::display_foreign_type::DisplayForeignType;
//                     i.display_foreign_type()
//                 })
//             }
//             OneErrorEnum::ErrorOccurence(i) => {
//                 OneErrorEnumWithDeserialize::ErrorOccurence(i.into_serialize_deserialize_version())
//             }
//             OneErrorEnum::VecToString(i) => OneErrorEnumWithDeserialize::VecToString(i),
//             OneErrorEnum::VecDisplayForeignType(i) => {
//                 OneErrorEnumWithDeserialize::VecDisplayForeignType({
//                     use crate::traits::error_logs_logic::vec_display_foreign_type_to_vec_string::VecDisplayForeignTypeToVecString;
//                     i.vec_display_foreign_type_to_vec_string()
//                 })
//             }
//             OneErrorEnum::VecErrorOccurence(i) => OneErrorEnumWithDeserialize::VecErrorOccurence({
//                 i.into_iter()
//                     .map(|e| e.into_serialize_deserialize_version())
//                     .collect()
//             }),
//             OneErrorEnum::HashMapKeyToStringValueToString(i) => {
//                 OneErrorEnumWithDeserialize::HashMapKeyToStringValueToString(i)
//             }
//             OneErrorEnum::HashMapKeyToStringValueDisplayForeignType(i) => {
//                 OneErrorEnumWithDeserialize::HashMapKeyToStringValueDisplayForeignType({
//                     use crate::traits::error_logs_logic::hashmap_impl_display_display_foreign_type_to_hashmap_impl_display_string::HashmapImplDisplayDisplayForeignTypeToHashmapImplDisplayString;
//                     i.hashmap_impl_display_display_foreign_type_to_hashmap_impl_display_string()
//                 })
//             }
//             OneErrorEnum::HashMapKeyToStringValueErrorOccurence(i) => {
//                 OneErrorEnumWithDeserialize::HashMapKeyToStringValueErrorOccurence({
//                     i.into_iter()
//                         .map(|(k, v)| (k, v.into_serialize_deserialize_version()))
//                         .collect()
//                 })
//             }
//             OneErrorEnum::HashMapKeyDisplayForeignTypeValueToString(i) => {
//                 OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueToString({
//                     use crate::traits::error_logs_logic::hashmap_display_foreign_type_impl_display_to_hashmap_string_impl_display::HashmapDisplayForeignTypeImplDisplayToHashMapStringImplDisplay;
//                     i.hashmap_display_foreign_type_impl_display_to_hashmap_string_impl_display()
//                 })
//             }
//             OneErrorEnum::HashMapKeyDisplayForeignTypeValueDisplayForeignType(i) => {
//                 OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueDisplayForeignType({
//                     use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_hashmap_string_string::HashmapDisplayForeignTypeDisplayForeignTypeToHashMapStringString;
//                     i.hashmap_display_foreign_type_display_foreign_type_to_hashmap_string_string()
//                 })
//             }
//             OneErrorEnum::HashMapKeyDisplayForeignTypeValueErrorOccurence(i) => {
//                 OneErrorEnumWithDeserialize::HashMapKeyDisplayForeignTypeValueErrorOccurence({
//                     i.into_iter()
//                         .map(|(k, v)| {
//                             use crate::traits::display_foreign_type::DisplayForeignType;
//                             (
//                                 k.display_foreign_type(),
//                                 v.into_serialize_deserialize_version(),
//                             )
//                         })
//                         .collect()
//                 })
//             }
//         }
//     }
// }
// impl<'a> std::fmt::Display for OneErrorEnum<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }
// impl<'a> std::fmt::Display for OneErrorEnumWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// #[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)] //, thiserror::Error, error_occurence::ImplErrorOccurence
// pub enum TestErrorEnum {
//     //todo - test crate::dev::SevenError<'a> as variant but without lifetime
//     #[to_string]
//     ToString(crate::dev::Omegalul),
//     // #[to_string]
//     // ToStringLifetime(crate::dev::OmegalulLifetime<'a>),
// }

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
        w: std::collections::HashMap<crate::dev::Kekw, crate::dev::OmegalulLifetime<'a>>,
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
        // #[vec_error_occurence]
        // ag: std::vec::Vec<OneErrorEnum<'a>>,
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
                // ag: _unused_argument_30,
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
                // ag,
                code_occurence: _unused_code_occurence_argument,
            } => {
                format!(
                    "{{\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}}}", //{}
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        a.lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        b.lines_space_backslash()
                    },
                    {
                        use crate::traits::display_foreign_type::DisplayForeignType;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        c.display_foreign_type().lines_space_backslash()
                    },
                    {
                        use crate::traits::display_foreign_type::DisplayForeignType;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        c.display_foreign_type().lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
                        e.to_string_without_config().lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                        f.vec_display_to_string().lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                        g.vec_display_to_string().lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::vec_display_foreign_type_to_string::VecDisplayForeignTypeToString;
                        h.vec_display_foreign_type_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::vec_display_foreign_type_to_string::VecDisplayForeignTypeToString;
                        j.vec_display_foreign_type_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToString;
                        k.vec_to_string_without_config_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        l.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        m.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        n.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        o.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_to_string::HashMapDisplayDisplayForeignTypeToString;
                        p.hashmap_display_display_foreign_type_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_to_string::HashMapDisplayDisplayForeignTypeToString;
                        q.hashmap_display_display_foreign_type_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_to_string::HashMapDisplayDisplayForeignTypeToString;
                        r.hashmap_display_display_foreign_type_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_to_string::HashMapDisplayDisplayForeignTypeToString;
                        s.hashmap_display_display_foreign_type_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_to_string_without_config_to_string::HashmapDisplayToStringWithoutConfigToString;
                        t.hashmap_display_to_string_without_config_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_to_string_without_config_to_string::HashmapDisplayToStringWithoutConfigToString;
                        u.hashmap_display_to_string_without_config_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_to_string::HashMapDisplayForeignTypeDisplayToString;
                        v.hashmap_display_foreign_type_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_to_string::HashMapDisplayForeignTypeDisplayToString;
                        w.hashmap_display_foreign_type_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_to_string::HashMapDisplayForeignTypeDisplayToString;
                        x.hashmap_display_foreign_type_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_to_string::HashMapDisplayForeignTypeDisplayToString;
                        y.hashmap_display_foreign_type_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
                        z.hashmap_display_foreign_type_display_foreign_type_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
                        aa.hashmap_display_foreign_type_display_foreign_type_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
                        ab.hashmap_display_foreign_type_display_foreign_type_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
                        ac.hashmap_display_foreign_type_display_foreign_type_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_foreign_type_to_string_without_config_to_string::HashMapDisplayForeignTypeToStringWithoutConfigToString;
                        ad.hashmap_display_foreign_type_to_string_without_config_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_foreign_type_to_string_without_config_to_string::HashMapDisplayForeignTypeToStringWithoutConfigToString;
                        af.hashmap_display_foreign_type_to_string_without_config_to_string()
                            .lines_space_backslash()
                    },
                    // {
                    //     use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                    //     use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToString;
                    //     ag.vec_to_string_without_config_to_string()
                    //         .lines_space_backslash()
                    // }
                )
            }
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
                // ag: _unused_argument_30,
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
        c: &'static str,
        d: &'static str,
        #[serde(borrow)]
        e: crate::dev::SevenErrorWithDeserialize<'a>,
        f: std::vec::Vec<crate::dev::Omegalul>,
        #[serde(borrow)]
        g: std::vec::Vec<crate::dev::OmegalulLifetime<'a>>,
        h: std::vec::Vec<&'static str>,
        j: std::vec::Vec<&'static str>,
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
        p: std::collections::HashMap<crate::dev::Omegalul, &'static str>,
        q: std::collections::HashMap<crate::dev::Omegalul, &'static str>,
        #[serde(borrow)]
        r: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, &'static str>,
        #[serde(borrow)]
        s: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, &'static str>,
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
        v: std::collections::HashMap<&'static str, crate::dev::Omegalul>,
        #[serde(borrow)]
        w: std::collections::HashMap<&'static str, crate::dev::OmegalulLifetime<'a>>,
        x: std::collections::HashMap<&'static str, crate::dev::Omegalul>,
        #[serde(borrow)]
        y: std::collections::HashMap<&'static str, crate::dev::OmegalulLifetime<'a>>,
        z: std::collections::HashMap<&'static str, &'static str>,
        aa: std::collections::HashMap<&'static str, &'static str>,
        ab: std::collections::HashMap<&'static str, &'static str>,
        ac: std::collections::HashMap<&'static str, &'static str>,
        #[serde(borrow)]
        ad: std::collections::HashMap<&'static str, crate::dev::SevenErrorWithDeserialize<'a>>,
        #[serde(borrow)]
        af: std::collections::HashMap<&'static str, crate::dev::SevenErrorWithDeserialize<'a>>,
        // #[serde(borrow)]
        // ag: std::vec::Vec<crate::dev::OneErrorEnumWithDeserialize<'a>>,
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
                // ag,
                code_occurence: _unused_code_occurence_argument,
            } => {
                format!(
                    "{{\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}}}", //{}
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        a.lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        b.lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        c.lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        d.lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
                        e.to_string_without_config_with_deserialize()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                        f.vec_display_to_string().lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                        g.vec_display_to_string().lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                        h.vec_display_to_string().lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                        j.vec_display_to_string().lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToStringWithDeserialize;
                        k.vec_to_string_without_config_to_string_with_deserialize()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        l.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        m.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        n.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        o.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        p.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        q.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        r.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        s.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_to_string_without_config_to_string::HashmapDisplayToStringWithoutConfigToStringWithDeserialize;
                        t.hashmap_display_to_string_without_config_to_string_with_deserialize()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_to_string_without_config_to_string::HashmapDisplayToStringWithoutConfigToStringWithDeserialize;
                        u.hashmap_display_to_string_without_config_to_string_with_deserialize()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        v.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        w.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        x.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        y.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        z.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        aa.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        ab.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        ac.hashmap_display_display_to_string()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_to_string_without_config_to_string::HashmapDisplayToStringWithoutConfigToStringWithDeserialize;
                        ad.hashmap_display_to_string_without_config_to_string_with_deserialize()
                            .lines_space_backslash()
                    },
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::hashmap_display_to_string_without_config_to_string::HashmapDisplayToStringWithoutConfigToStringWithDeserialize;
                        af.hashmap_display_to_string_without_config_to_string_with_deserialize()
                            .lines_space_backslash()
                    },
                    // {
                    //     use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                    //     use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToStringWithDeserialize;
                    //     ag.vec_to_string_without_config_to_string_with_deserialize()
                    //         .lines_space_backslash()
                    // },
                )
            }
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
                // ag: _unused_argument_30,
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
                // ag,
                code_occurence,
            } => NamedErrorWithDeserialize::Something {
                a: { a },
                b: { b },
                c: {
                    use crate::traits::display_foreign_type::DisplayForeignType;
                    c.display_foreign_type()
                },
                d: {
                    use crate::traits::display_foreign_type::DisplayForeignType;
                    d.display_foreign_type()
                },
                e: { e.into_serialize_deserialize_version() },
                f: { f },
                g: { g },
                h: {
                    use crate::traits::error_logs_logic::vec_display_foreign_type_into_vec_string::VecDisplayForeignTypeIntoVecString;
                    h.vec_display_foreign_type_into_vec_string()
                },
                j: {
                    use crate::traits::error_logs_logic::vec_display_foreign_type_into_vec_string::VecDisplayForeignTypeIntoVecString;
                    j.vec_display_foreign_type_into_vec_string()
                },
                k: {
                    k.into_iter()
                        .map(|i| i.into_serialize_deserialize_version())
                        .collect()
                },
                l: { l },
                m: { m },
                n: { n },
                o: { o },
                p: {
                    use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_into_hashmap_display_string::HashmapDisplayDisplayForeignTypeIntoHashmapDisplayString;
                    p.hashmap_display_display_foreign_type_into_hashmap_display_string()
                },
                q: {
                    use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_into_hashmap_display_string::HashmapDisplayDisplayForeignTypeIntoHashmapDisplayString;
                    q.hashmap_display_display_foreign_type_into_hashmap_display_string()
                },
                r: {
                    use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_into_hashmap_display_string::HashmapDisplayDisplayForeignTypeIntoHashmapDisplayString;
                    r.hashmap_display_display_foreign_type_into_hashmap_display_string()
                },
                s: {
                    use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_into_hashmap_display_string::HashmapDisplayDisplayForeignTypeIntoHashmapDisplayString;
                    s.hashmap_display_display_foreign_type_into_hashmap_display_string()
                },
                t: {
                    t.into_iter()
                        .map(|(k, v)| (k, { v.into_serialize_deserialize_version() }))
                        .collect()
                },
                u: {
                    u.into_iter()
                        .map(|(k, v)| (k, { v.into_serialize_deserialize_version() }))
                        .collect()
                },
                v: {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_into_hashmap_string_display::HashmapDisplayForeignTypeDisplayIntoHashMapStringDisplay;
                    v.hashmap_display_foreign_type_display_into_hashmap_string_display()
                },
                w: {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_into_hashmap_string_display::HashmapDisplayForeignTypeDisplayIntoHashMapStringDisplay;
                    w.hashmap_display_foreign_type_display_into_hashmap_string_display()
                },
                x: {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_into_hashmap_string_display::HashmapDisplayForeignTypeDisplayIntoHashMapStringDisplay;
                    x.hashmap_display_foreign_type_display_into_hashmap_string_display()
                },
                y: {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_into_hashmap_string_display::HashmapDisplayForeignTypeDisplayIntoHashMapStringDisplay;
                    y.hashmap_display_foreign_type_display_into_hashmap_string_display()
                },
                z: {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string::HashmapDisplayForeignTypeDisplayForeignTypeIntoHashMapStringString;
                    z.hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string()
                },
                aa: {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string::HashmapDisplayForeignTypeDisplayForeignTypeIntoHashMapStringString;
                    aa.hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string(
                    )
                },
                ab: {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string::HashmapDisplayForeignTypeDisplayForeignTypeIntoHashMapStringString;
                    ab.hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string(
                    )
                },
                ac: {
                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string::HashmapDisplayForeignTypeDisplayForeignTypeIntoHashMapStringString;
                    ac.hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string(
                    )
                },
                ad: {
                    ad.into_iter()
                        .map(|(k, v)| {
                            (
                                {
                                    use crate::traits::display_foreign_type::DisplayForeignType;
                                    k.display_foreign_type()
                                },
                                { v.into_serialize_deserialize_version() },
                            )
                        })
                        .collect()
                },
                af: {
                    af.into_iter()
                        .map(|(k, v)| {
                            (
                                {
                                    use crate::traits::display_foreign_type::DisplayForeignType;
                                    k.display_foreign_type()
                                },
                                { v.into_serialize_deserialize_version() },
                            )
                        })
                        .collect()
                },
                // ag: {
                //     ag.into_iter()
                //         .map(|i| i.into_serialize_deserialize_version())
                //         .collect()
                // },
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
