#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum ThreeWrapperError<'a> {
    Something {
        //todo how to implement from for it?
        inner_error: ThreeWrapperErrorEnum<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl<'a> std::fmt::Display for ThreeWrapperError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for ThreeWrapperError<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             ThreeWrapperError::Something {
//                 inner_error,
//                 code_occurence: _code_occurence,
//             } => {
//                 use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig;
//                 inner_error.to_string_with_config_for_source_to_string_with_config(config)
//             }
//         }
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for ThreeWrapperError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             ThreeWrapperError::Something {
//                 inner_error,
//                 code_occurence: _code_occurence,
//             } => {
//                 use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//                 inner_error.to_string_without_config()
//             }
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
//     for ThreeWrapperError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             ThreeWrapperError::Something {
//                 inner_error: _inner_error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum ThreeWrapperErrorEnum<'a> {
    FourWrapper(FourWrapperError<'a>),
}

// impl<'a> std::fmt::Display for ThreeWrapperErrorEnum<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for ThreeWrapperErrorEnum<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config_for_source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             ThreeWrapperErrorEnum::FourWrapper(i) => i.to_string_with_config_for_source_to_string_with_config(config),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
//     for ThreeWrapperErrorEnum<'a>
// {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             ThreeWrapperErrorEnum::FourWrapper(i) => i.to_string_without_config(),
//         }
//     }
// }

pub fn three<'a>() -> Result<(), Box<ThreeWrapperError<'a>>> {
    if let Err(e) = four() {
        let f = ThreeWrapperError::Something {
            inner_error: ThreeWrapperErrorEnum::FourWrapper(*e),
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    };
    Ok(())
}

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum FourWrapperError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::collections::HashMap<String, FourWrapperErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl<'a> std::fmt::Display for FourWrapperError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for FourWrapperError<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             FourWrapperError::Something {
//                 inner_errors,
//                 code_occurence: _code_occurence,
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
//     > for FourWrapperError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             FourWrapperError::Something {
//                 inner_errors,
//                 code_occurence: _code_occurence,
//             } => {
//                 use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
//                 inner_errors.few_to_string_without_config()
//             }
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
//     for FourWrapperError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             FourWrapperError::Something {
//                 inner_errors: _inner_errors,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum FourWrapperErrorEnum<'a> {
    FiveWrapper(FiveWrapperError<'a>),
    SixWrapper(SixWrapperError<'a>),
}

// impl<'a> std::fmt::Display for FourWrapperErrorEnum<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for FourWrapperErrorEnum<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config_for_source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             FourWrapperErrorEnum::FiveWrapper(i) => i.to_string_with_config_for_source_to_string_with_config(config),
//             FourWrapperErrorEnum::SixWrapper(i) => i.to_string_with_config_for_source_to_string_with_config(config),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
//     for FourWrapperErrorEnum<'a>
// {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             FourWrapperErrorEnum::FiveWrapper(i) => i.to_string_without_config(),
//             FourWrapperErrorEnum::SixWrapper(i) => i.to_string_without_config(),
//         }
//     }
// }

pub fn four<'a>() -> Result<(), Box<FourWrapperError<'a>>> {
    match (five(), six()) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            let f = FourWrapperError::Something {
                inner_errors: std::collections::HashMap::from([
                    (
                        String::from("five_hashmap_key"),
                        FourWrapperErrorEnum::FiveWrapper(*f),
                    ),
                    (
                        String::from("six_hashmap_key"),
                        FourWrapperErrorEnum::SixWrapper(*s),
                    ),
                ]),
                code_occurence: crate::code_occurence_tufa_common!(),
            };
            return Err(Box::new(f));
        }
    }
}

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum FiveWrapperError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::collections::HashMap<String, crate::dev::FiveWrapperErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl<'a> std::fmt::Display for FiveWrapperError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for FiveWrapperError<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             FiveWrapperError::Something {
//                 inner_errors,
//                 code_occurence: _code_occurence,
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
//     > for FiveWrapperError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             FiveWrapperError::Something {
//                 inner_errors,
//                 code_occurence: _code_occurence,
//             } => {
//                 use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
//                 inner_errors.few_to_string_without_config()
//             }
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
//     for FiveWrapperError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             FiveWrapperError::Something {
//                 inner_errors: _inner_error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum FiveWrapperErrorEnum<'a> {
    FiveOneOrigin(FiveOneOriginError<'a>),
}

// impl<'a> std::fmt::Display for FiveWrapperErrorEnum<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for FiveWrapperErrorEnum<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config_for_source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             FiveWrapperErrorEnum::FiveOneOrigin(i) => {
//                 use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithoutConfig;
//                 i.to_string_with_config_for_source_to_string_without_config(config)
//             },
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
//     for FiveWrapperErrorEnum<'a>
// {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             FiveWrapperErrorEnum::FiveOneOrigin(i) => i.to_string_without_config(),
//         }
//     }
// }

pub fn five<'a>() -> Result<(), Box<FiveWrapperError<'a>>> {
    if let Err(e) = five_one() {
        let f = FiveWrapperError::Something {
            inner_errors: std::collections::HashMap::from([(
                String::from("five_one_hashmap_key"),
                FiveWrapperErrorEnum::FiveOneOrigin(*e),
            )]),
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    }
    Ok(())
}

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum FiveOneOriginError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl<'a> std::fmt::Display for FiveOneOriginError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for FiveOneOriginError<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, _config: &ConfigGeneric) -> String {
//         use crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig;
//         self.source_to_string_without_config()
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for FiveOneOriginError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             FiveOneOriginError::Something {
//                 error,
//                 code_occurence: _code_occurence,
//             } => format!("{}", error),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
//     for FiveOneOriginError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             FiveOneOriginError::Something {
//                 error: _error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

pub fn five_one<'a>() -> Result<(), Box<FiveOneOriginError<'a>>> {
    return Err(Box::new(FiveOneOriginError::Something {
        error: String::from("five_one error"),
        code_occurence: crate::code_occurence_tufa_common!(),
    }));
}

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum SixWrapperError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::vec::Vec<crate::dev::SixWrapperErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl<'a> std::fmt::Display for SixWrapperError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for SixWrapperError<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             SixWrapperError::Something {
//                 inner_errors,
//                 code_occurence: _code_occurence,
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
//     > for SixWrapperError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             SixWrapperError::Something {
//                 inner_errors,
//                 code_occurence: _code_occurence,
//             } => {
//                 use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
//                 inner_errors.few_to_string_without_config()
//             }
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
//     for SixWrapperError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             SixWrapperError::Something {
//                 inner_errors: _inner_errors,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum SixWrapperErrorEnum<'a> {
    SevenWrapper(crate::dev::SevenOriginError<'a>),
    EightWrapper(EightOriginError<'a>),
}

// impl<'a> std::fmt::Display for SixWrapperErrorEnum<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for SixWrapperErrorEnum<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config_for_source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         //todo this logic must generate to_string_with_config_for_source_to_string_with_config for wrapper and to_string_with_config_for_source_to_string_without_config for origin, and maybe add trait usage if there is\are origins
//         match self {
//             SixWrapperErrorEnum::SevenWrapper(i) => {
//                 use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithoutConfig;
//                 i.to_string_with_config_for_source_to_string_without_config(config)
//             },
//             SixWrapperErrorEnum::EightWrapper(i) => {
//                 use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithoutConfig;
//                 i.to_string_with_config_for_source_to_string_without_config(config)
//             },
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
//     for SixWrapperErrorEnum<'a>
// {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             SixWrapperErrorEnum::SevenWrapper(i) => i.to_string_without_config(),
//             SixWrapperErrorEnum::EightWrapper(i) => i.to_string_without_config(),
//         }
//     }
// }

pub fn six<'a>() -> Result<(), Box<SixWrapperError<'a>>> {
    let thread_join_handle = std::thread::spawn(move || eight());
    let res = thread_join_handle.join().unwrap();
    match (seven(), res) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let f = SixWrapperError::Something {
                inner_errors: vec![
                    SixWrapperErrorEnum::SevenWrapper(*seven_error),
                    SixWrapperErrorEnum::EightWrapper(*eight_error),
                ],
                code_occurence: crate::code_occurence_tufa_common!(),
            };
            return Err(Box::new(f));
        }
    }
}

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum SevenOriginError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl<'a> std::fmt::Display for SevenOriginError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for SevenOriginError<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, _config: &ConfigGeneric) -> String {
//         use crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig;
//         self.source_to_string_without_config()
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for SevenOriginError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             SevenOriginError::Something {
//                 error,
//                 code_occurence: _code_occurence,
//             } => format!("{}", error),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
//     for SevenOriginError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             SevenOriginError::Something {
//                 error: _error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

pub fn seven<'a>() -> Result<(), Box<SevenOriginError<'a>>> {
    return Err(Box::new(SevenOriginError::Something {
        error: String::from("error_eight"),
        code_occurence: crate::code_occurence_tufa_common!(),
    }));
}

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum EightOriginError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl<'a> std::fmt::Display for EightOriginError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for EightOriginError<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, _config: &ConfigGeneric) -> String {
//         use crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig;
//         self.source_to_string_without_config()
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for EightOriginError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             EightOriginError::Something {
//                 error,
//                 code_occurence: _code_occurence,
//             } => format!("{}", error),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
//     for EightOriginError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             EightOriginError::Something {
//                 error: _error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

pub fn eight<'a>() -> Result<(), Box<EightOriginError<'a>>> {
    let f = EightOriginError::Something {
        error: String::from("error_eight"),
        code_occurence: crate::code_occurence_tufa_common!(),
    };
    // use crate::traits::error_logs_logic::error_log::ErrorLog;
    // f.error_log(once_cell::sync::Lazy::force(
    //     &crate::global_variables::runtime::config::CONFIG,
    // ));
    return Err(Box::new(f));
}
