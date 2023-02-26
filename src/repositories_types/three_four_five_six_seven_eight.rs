#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum ThreeError<'a> {
    Something {
        //todo how to implement from for it?
        inner_error: FourWrapperError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl<'a> std::fmt::Display for ThreeError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for ThreeError<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             ThreeError::Something {
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
//     > for ThreeError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             ThreeError::Something {
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
//     for ThreeError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             ThreeError::Something {
//                 inner_error: _inner_error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum ThreeErrorWithDeserialize<'a> {
//     Something {
//         //todo how to implement from for it?
//         #[serde(borrow)]
//         inner_error: ThreeErrorEnumWithDeserialize<'a>,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for ThreeErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for ThreeErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             ThreeErrorWithDeserialize::Something {
//                 inner_error,
//                 code_occurence: _code_occurence,
//             } => {
//                 use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//                 inner_error.to_string_without_config_with_deserialize()
//             }
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
//     for ThreeErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             ThreeErrorWithDeserialize::Something {
//                 inner_error: _inner_error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum FourWrapperError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::collections::HashMap<String, FourErrorEnum<'a>>,
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

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum FourWrapperErrorWithDeserialize<'a> {
//     Something {
//         //todo how to implement from for it?
//         #[serde(borrow)]
//         inner_errors: std::collections::HashMap<
//             String,
//             FourErrorEnumWithDeserialize<'a>,
//         >,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for FourWrapperErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for FourWrapperErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             FourWrapperErrorWithDeserialize::Something {
//                 inner_errors,
//                 code_occurence: _code_occurence,
//             } => {
//                 use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigWithDeserialize;
//                 inner_errors.few_to_string_without_config_with_deserialize()
//             }
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
//     for FourWrapperErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             FourWrapperErrorWithDeserialize::Something {
//                 inner_errors: _inner_errors,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum FourErrorEnum<'a> {
    #[error_occurence_from_wrapper]
    Five(FiveError<'a>),
    #[error_occurence_from_wrapper]
    Six(SixError<'a>),
}

// impl<'a> std::fmt::Display for FourErrorEnum<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for FourErrorEnum<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config_for_source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             FourErrorEnum::Five(i) => i.to_string_with_config_for_source_to_string_with_config(config),
//             FourErrorEnum::Six(i) => i.to_string_with_config_for_source_to_string_with_config(config),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
//     for FourErrorEnum<'a>
// {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             FourErrorEnum::Five(i) => i.to_string_without_config(),
//             FourErrorEnum::Six(i) => i.to_string_without_config(),
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum FourErrorEnumWithDeserialize<'a> {
//     #[serde(borrow)]
//     Five(FiveErrorWithDeserialize<'a>),
//     #[serde(borrow)]
//     Six(SixErrorWithDeserialize<'a>),
// }

// impl<'a> std::fmt::Display for FourErrorEnumWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
//         'a,
//     > for FourErrorEnumWithDeserialize<'a>
// {
//     fn to_string_without_config_with_deserialize(&self) -> String {
//         match self {
//             FourErrorEnumWithDeserialize::Five(i) => {
//                 i.to_string_without_config_with_deserialize()
//             }
//             FourErrorEnumWithDeserialize::Six(i) => {
//                 i.to_string_without_config_with_deserialize()
//             }
//         }
//     }
// }

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum FiveError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::collections::HashMap<String, FiveErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl<'a> std::fmt::Display for FiveError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for FiveError<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             FiveError::Something {
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
//     > for FiveError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             FiveError::Something {
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
//     for FiveError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             FiveError::Something {
//                 inner_errors: _inner_error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum FiveErrorWithDeserialize<'a> {
//     Something {
//         //todo how to implement from for it?
//         #[serde(borrow)]
//         inner_errors: std::collections::HashMap<
//             String,
//             FiveErrorEnumWithDeserialize<'a>,
//         >,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for FiveErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for FiveErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             FiveErrorWithDeserialize::Something {
//                 inner_errors,
//                 code_occurence: _code_occurence,
//             } => {
//                 use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigWithDeserialize;
//                 inner_errors.few_to_string_without_config_with_deserialize()
//             }
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
//     for FiveErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             FiveErrorWithDeserialize::Something {
//                 inner_errors: _inner_error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum FiveErrorEnum<'a> {
    #[error_occurence_from_origin]
    FiveOne(FiveOneError<'a>),
}

// impl<'a> std::fmt::Display for FiveErrorEnum<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for FiveErrorEnum<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config_for_source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             FiveErrorEnum::FiveOne(i) => {
//                 use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithoutConfig;
//                 i.to_string_with_config_for_source_to_string_without_config(config)
//             },
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
//     for FiveErrorEnum<'a>
// {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             FiveErrorEnum::FiveOne(i) => i.to_string_without_config(),
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum FiveErrorWithEnumWithDeserialize<'a> {
//     #[serde(borrow)]
//     FiveOne(FiveOneErrorWithDeserialize<'a>),
// }

// impl<'a> std::fmt::Display for FiveErrorEnumWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
//         'a,
//     > for FiveErrorEnumWithDeserialize<'a>
// {
//     fn to_string_without_config_with_deserialize(&self) -> String {
//         match self {
//             FiveErrorEnumWithDeserialize::FiveOne(i) => {
//                 i.to_string_without_config_with_deserialize()
//             }
//         }
//     }
// }

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum FiveOneError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl<'a> std::fmt::Display for FiveOneError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for FiveOneError<'a>
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
//     > for FiveOneError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             FiveOneError::Something {
//                 error,
//                 code_occurence: _code_occurence,
//             } => error.to_string(),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
//     for FiveOneError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             FiveOneError::Something {
//                 error: _error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum FiveOneErrorWithDeserialize<'a> {
//     Something {
//         error: String,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for FiveOneErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for FiveOneErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             FiveOneErrorWithDeserialize::Something {
//                 error,
//                 code_occurence: _code_occurence,
//             } => error.to_string(),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
//     for FiveOneErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             FiveOneErrorWithDeserialize::Something {
//                 error: _error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum SixError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::vec::Vec<SixErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl<'a> std::fmt::Display for SixError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
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
//     > for SixError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             SixError::Something {
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
//     for SixError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             SixError::Something {
//                 inner_errors: _inner_errors,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum SixErrorWithDeserialize<'a> {
//     Something {
//         //todo how to implement from for it?
//         #[serde(borrow)]
//         inner_errors: Vec<SixErrorEnumWithDeserialize<'a>>,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for SixErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
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
//                 code_occurence: _code_occurence,
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
//                 inner_errors: _inner_errors,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum SixErrorEnum<'a> {
    #[error_occurence_from_origin]
    Seven(SevenError<'a>),
    #[error_occurence_from_origin]
    Eight(EightError<'a>),
}

// impl<'a> std::fmt::Display for SixErrorEnum<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for SixErrorEnum<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config_for_source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         //todo this logic must generate to_string_with_config_for_source_to_string_with_config for wrapper and to_string_with_config_for_source_to_string_without_config for origin, and maybe add trait usage if there is\are origins
//         match self {
//             SixErrorEnum::Seven(i) => {
//                 use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithoutConfig;
//                 i.to_string_with_config_for_source_to_string_without_config(config)
//             },
//             SixErrorEnum::Eight(i) => {
//                 use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithoutConfig;
//                 i.to_string_with_config_for_source_to_string_without_config(config)
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
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum SixErrorEnumWithDeserialize<'a> {
//     #[serde(borrow)]
//     Seven(SevenErrorWithDeserialize<'a>),
//     #[serde(borrow)]
//     Eight(EightErrorWithDeserialize<'a>),
// }

// impl<'a> std::fmt::Display for SixErrorEnumWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
//         'a,
//     > for SixErrorEnumWithDeserialize<'a>
// {
//     fn to_string_without_config_with_deserialize(&self) -> String {
//         match self {
//             SixErrorEnumWithDeserialize::Seven(i) => {
//                 i.to_string_without_config_with_deserialize()
//             }
//             SixErrorEnumWithDeserialize::Eight(i) => {
//                 i.to_string_without_config_with_deserialize()
//             }
//         }
//     }
// }

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum SevenError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl<'a> std::fmt::Display for SevenError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > for SevenError<'a>
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
//     > for SevenError<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             SevenError::Something {
//                 error,
//                 code_occurence: _code_occurence,
//             } => error.to_string(),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
//     for SevenError<'a>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
//         match self {
//             SevenError::Something {
//                 error: _error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum SevenErrorWithDeserialize<'a> {
//     Something {
//         error: String,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for SevenErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for SevenErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             SevenErrorWithDeserialize::Something {
//                 error,
//                 code_occurence: _code_occurence,
//             } => error.to_string(),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
//     for SevenErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             SevenErrorWithDeserialize::Something {
//                 error: _error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)] //
pub enum EightError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    SomethingElse {
        inner_error: NineError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
////////////////////////////////////////////////

///////////////////////////////+++=
// impl<'a> std::fmt::Display for EightError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
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
//             } => error.to_string(),
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
// impl<'a> std::fmt::Display for EightErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }
////////////////////////////////////////////////

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum NineError<'a> {
    NineSomething {
        //todo how to implement from for it?
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
