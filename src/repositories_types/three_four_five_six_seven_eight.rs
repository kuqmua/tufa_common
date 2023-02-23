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

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum ThreeWrapperErrorWithDeserialize<'a> {
//     Something {
//         //todo how to implement from for it?
//         #[serde(borrow)]
//         inner_error: ThreeWrapperErrorEnumWithDeserialize<'a>,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for ThreeWrapperErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for ThreeWrapperErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             ThreeWrapperErrorWithDeserialize::Something {
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
//     for ThreeWrapperErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             ThreeWrapperErrorWithDeserialize::Something {
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
    Four(FourWrapperError<'a>),
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
//             ThreeWrapperErrorEnum::Four(i) => i.to_string_with_config_for_source_to_string_with_config(config),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
//     for ThreeWrapperErrorEnum<'a>
// {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             ThreeWrapperErrorEnum::Four(i) => i.to_string_without_config(),
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum ThreeWrapperErrorEnumWithDeserialize<'a> {
//     #[serde(borrow)]
//     Four(FourWrapperErrorWithDeserialize<'a>),
// }

// impl<'a> std::fmt::Display for ThreeWrapperErrorEnumWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
//         'a,
//     > for ThreeWrapperErrorEnumWithDeserialize<'a>
// {
//     fn to_string_without_config_with_deserialize(&self) -> String {
//         match self {
//             ThreeWrapperErrorEnumWithDeserialize::Four(i) => {
//                 i.to_string_without_config_with_deserialize()
//             }
//         }
//     }
// }

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

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum FourWrapperErrorWithDeserialize<'a> {
//     Something {
//         //todo how to implement from for it?
//         #[serde(borrow)]
//         inner_errors: std::collections::HashMap<
//             String,
//             FourWrapperErrorEnumWithDeserialize<'a>,
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

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum FourWrapperErrorEnum<'a> {
    Five(FiveWrapperError<'a>),
    Six(SixWrapperError<'a>),
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
//             FourWrapperErrorEnum::Five(i) => i.to_string_with_config_for_source_to_string_with_config(config),
//             FourWrapperErrorEnum::Six(i) => i.to_string_with_config_for_source_to_string_with_config(config),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
//     for FourWrapperErrorEnum<'a>
// {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             FourWrapperErrorEnum::Five(i) => i.to_string_without_config(),
//             FourWrapperErrorEnum::Six(i) => i.to_string_without_config(),
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum FourWrapperErrorEnumWithDeserialize<'a> {
//     #[serde(borrow)]
//     Five(FiveWrapperErrorWithDeserialize<'a>),
//     #[serde(borrow)]
//     Six(SixWrapperErrorWithDeserialize<'a>),
// }

// impl<'a> std::fmt::Display for FourWrapperErrorEnumWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
//         'a,
//     > for FourWrapperErrorEnumWithDeserialize<'a>
// {
//     fn to_string_without_config_with_deserialize(&self) -> String {
//         match self {
//             FourWrapperErrorEnumWithDeserialize::Five(i) => {
//                 i.to_string_without_config_with_deserialize()
//             }
//             FourWrapperErrorEnumWithDeserialize::Six(i) => {
//                 i.to_string_without_config_with_deserialize()
//             }
//         }
//     }
// }

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum FiveWrapperError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::collections::HashMap<String, FiveWrapperErrorEnum<'a>>,
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

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum FiveWrapperErrorWithDeserialize<'a> {
//     Something {
//         //todo how to implement from for it?
//         #[serde(borrow)]
//         inner_errors: std::collections::HashMap<
//             String,
//             FiveWrapperErrorEnumWithDeserialize<'a>,
//         >,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for FiveWrapperErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for FiveWrapperErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             FiveWrapperErrorWithDeserialize::Something {
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
//     for FiveWrapperErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             FiveWrapperErrorWithDeserialize::Something {
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
    FiveOne(FiveOneOriginError<'a>),
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
//             FiveWrapperErrorEnum::FiveOne(i) => {
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
//             FiveWrapperErrorEnum::FiveOne(i) => i.to_string_without_config(),
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum FiveWrapperErrorWithEnumWithDeserialize<'a> {
//     #[serde(borrow)]
//     FiveOne(FiveOneOriginErrorWithDeserialize<'a>),
// }

// impl<'a> std::fmt::Display for FiveWrapperErrorEnumWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
//         'a,
//     > for FiveWrapperErrorEnumWithDeserialize<'a>
// {
//     fn to_string_without_config_with_deserialize(&self) -> String {
//         match self {
//             FiveWrapperErrorEnumWithDeserialize::FiveOne(i) => {
//                 i.to_string_without_config_with_deserialize()
//             }
//         }
//     }
// }

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

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum FiveOneOriginErrorWithDeserialize<'a> {
//     Something {
//         error: String,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for FiveOneOriginErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for FiveOneOriginErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             FiveOneOriginErrorWithDeserialize::Something {
//                 error,
//                 code_occurence: _code_occurence,
//             } => format!("{}", error),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
//     for FiveOneOriginErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             FiveOneOriginErrorWithDeserialize::Something {
//                 error: _error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

#[derive(
    Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurenceFromCrate,
)]
pub enum SixWrapperError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::vec::Vec<SixWrapperErrorEnum<'a>>,
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

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum SixWrapperErrorWithDeserialize<'a> {
//     Something {
//         //todo how to implement from for it?
//         #[serde(borrow)]
//         inner_errors: Vec<SixWrapperErrorEnumWithDeserialize<'a>>,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for SixWrapperErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for SixWrapperErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             SixWrapperErrorWithDeserialize::Something {
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
//     for SixWrapperErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             SixWrapperErrorWithDeserialize::Something {
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
    Seven(SevenOriginError<'a>),
    Eight(EightOriginError<'a>),
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
//             SixWrapperErrorEnum::Seven(i) => {
//                 use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithoutConfig;
//                 i.to_string_with_config_for_source_to_string_without_config(config)
//             },
//             SixWrapperErrorEnum::Eight(i) => {
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
//             SixWrapperErrorEnum::Seven(i) => i.to_string_without_config(),
//             SixWrapperErrorEnum::Eight(i) => i.to_string_without_config(),
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum SixWrapperErrorEnumWithDeserialize<'a> {
//     #[serde(borrow)]
//     Seven(SevenOriginErrorWithDeserialize<'a>),
//     #[serde(borrow)]
//     Eight(EightOriginErrorWithDeserialize<'a>),
// }

// impl<'a> std::fmt::Display for SixWrapperErrorEnumWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
//         'a,
//     > for SixWrapperErrorEnumWithDeserialize<'a>
// {
//     fn to_string_without_config_with_deserialize(&self) -> String {
//         match self {
//             SixWrapperErrorEnumWithDeserialize::Seven(i) => {
//                 i.to_string_without_config_with_deserialize()
//             }
//             SixWrapperErrorEnumWithDeserialize::Eight(i) => {
//                 i.to_string_without_config_with_deserialize()
//             }
//         }
//     }
// }

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

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum SevenOriginErrorWithDeserialize<'a> {
//     Something {
//         error: String,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for SevenOriginErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for SevenOriginErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             SevenOriginErrorWithDeserialize::Something {
//                 error,
//                 code_occurence: _code_occurence,
//             } => format!("{}", error),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
//     for SevenOriginErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             SevenOriginErrorWithDeserialize::Something {
//                 error: _error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

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

// #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
// pub enum EightOriginErrorWithDeserialize<'a> {
//     Something {
//         error: String,
//         #[serde(borrow)]
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for EightOriginErrorWithDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
//         write!(f, "{}", self.to_string_without_config_with_deserialize())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
//         'a,
//     > for EightOriginErrorWithDeserialize<'a>
// {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             EightOriginErrorWithDeserialize::Something {
//                 error,
//                 code_occurence: _code_occurence,
//             } => format!("{}", error),
//         }
//     }
// }

// impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
//     for EightOriginErrorWithDeserialize<'a>
// {
//     fn get_code_occurence_with_deserialize(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
//         match self {
//             EightOriginErrorWithDeserialize::Something {
//                 error: _error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }
