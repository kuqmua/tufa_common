use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
// use crate::traits::error_logs_logic::error_log::ErrorLog;

// #[derive(Debug, Serialize, Deserialize, Error)]
// pub enum ThreeWrapperError {
//     Something {
//         source: ThreeWrapperErrorEnum,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     }
// }
// //cannot make it with generics
// impl std::fmt::Display for ThreeWrapperError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<ConfigGeneric> crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<ConfigGeneric> for ThreeWrapperError
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfig;
//         match self {
//             ThreeWrapperError::Something { source, code_occurence } => source.to_string_with_config(config),
//         }
//     }
// }

// impl crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig for ThreeWrapperError {
//     fn source_to_string_without_config(&self) -> String {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         match self {
//             ThreeWrapperError::Something { source, code_occurence } => source.to_string_without_config(),
//         }
//     }
// }

// impl crate::traits::get_code_occurence::GetCodeOccurence for ThreeWrapperError {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
//         match self {
//             ThreeWrapperError::Something { source, code_occurence } => code_occurence,
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Error)]
// pub enum ThreeWrapperErrorEnum {
//     FourWrapper(FourWrapperError),
// }

// impl std::fmt::Display for ThreeWrapperErrorEnum {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<ConfigGeneric> crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfig<ConfigGeneric> for ThreeWrapperErrorEnum
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             ThreeWrapperErrorEnum::FourWrapper(i) => i.to_string_with_config(config),
//         }
//     }
// }

// impl crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig for ThreeWrapperErrorEnum {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             ThreeWrapperErrorEnum::FourWrapper(i) => i.to_string_without_config(),
//         }
//     }
// }

// pub fn three() -> Result<(), Box<ThreeWrapperError>> {
//     if let Err(e) = four() {
//         let f = ThreeWrapperError::Something {
//             source: ThreeWrapperErrorEnum::FourWrapper(*e),
//             code_occurence: crate::common::code_occurence::CodeOccurence::new(
//                 once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
//                     String::from(file!()),
//                     line!(),
//                     column!(),
//                 )
//         };
//         // println!("three");
//         // f.error_log(once_cell::sync::Lazy::force(
//         //     &crate::global_variables::runtime::config::CONFIG,
//         // ));
//         // println!("threeend");
//         return Err(Box::new(f));
//     };
//     Ok(())
// }

// #[derive(Debug, Serialize, Deserialize, Error)]
// pub enum FourWrapperError {
//     Something {
//         //todo how to implement from for it?
//         sources: std::collections::HashMap<String, FourWrapperErrorEnum>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     }
// }

// //cannot make it with generics
// impl std::fmt::Display for FourWrapperError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<ConfigGeneric> crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<ConfigGeneric> for FourWrapperError
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         use crate::traits::error_logs_logic::few_to_string_with_config::FewToStringWithConfig;
//         match self {
//             FourWrapperError::Something { sources, code_occurence } => sources.few_to_string_with_config(config),
//         }
//     }
// }

// impl crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig for FourWrapperError {
//     fn source_to_string_without_config(&self) -> String {
//         use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
//         match self {
//             FourWrapperError::Something { sources, code_occurence } => sources.few_to_string_without_config(),
//         }
//     }
// }

// impl crate::traits::get_code_occurence::GetCodeOccurence for FourWrapperError {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
//         match self {
//             FourWrapperError::Something { sources, code_occurence } => code_occurence,
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Error)]
// pub enum FourWrapperErrorEnum {
//     FiveWrapper(FiveWrapperError),
//     SixWrapper(SixWrapperError),
// }

// impl std::fmt::Display for FourWrapperErrorEnum {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<ConfigGeneric> crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfig<ConfigGeneric> for FourWrapperErrorEnum
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         match self {
//             FourWrapperErrorEnum::FiveWrapper(i) => i.to_string_with_config(config),
//             FourWrapperErrorEnum::SixWrapper(i) => i.to_string_with_config(config),
//         }
//     }
// }

// impl crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig for FourWrapperErrorEnum {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             FourWrapperErrorEnum::FiveWrapper(i) => i.to_string_without_config(),
//             FourWrapperErrorEnum::SixWrapper(i) => i.to_string_without_config(),
//         }
//     }
// }

// pub fn four() -> Result<(), Box<FourWrapperError>> {
//     match (five(), six()) {
//         (Ok(_), Ok(_)) => todo!(),
//         (Ok(_), Err(_)) => todo!(),
//         (Err(_), Ok(_)) => todo!(),
//         (Err(f), Err(s)) => {
//             let f = FourWrapperError::Something {
//                 sources: std::collections::HashMap::from([
//                     (
//                         String::from("five_hashmap_key"),
//                         FourWrapperErrorEnum::FiveWrapper(*f),
//                     ),
//                     (
//                         String::from("six_hashmap_key"),
//                         FourWrapperErrorEnum::SixWrapper(*s),
//                     ),
//                 ]),
//                 code_occurence: crate::common::code_occurence::CodeOccurence::new(
//                     once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
//                         String::from(file!()),
//                         line!(),
//                         column!(),
//                     )
//             };
//             // println!("=======");
//             // println!("{}", f);
//             // f.error_log(once_cell::sync::Lazy::force(
//             //     &crate::global_variables::runtime::config::CONFIG,
//             // ));
//             // println!("=======");
//             return Err(Box::new(f));
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Error)]
// pub enum FiveWrapperError {
//     Something{
//         //todo how to implement from for it?
//         sources: std::collections::HashMap<String, FiveWrapperErrorEnum>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     }
// }
// //cannot make it with generics
// impl std::fmt::Display for FiveWrapperError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<ConfigGeneric> crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<ConfigGeneric> for FiveWrapperError
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         use crate::traits::error_logs_logic::few_to_string_with_config::FewToStringWithConfig;
//         match self {
//             FiveWrapperError::Something { sources, code_occurence } => sources.few_to_string_with_config(config),
//         }
//     }
// }

// impl crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig for FiveWrapperError {
//     fn source_to_string_without_config(&self) -> String {
//         use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
//         match self {
//             FiveWrapperError::Something { sources, code_occurence } => sources.few_to_string_without_config(),
//         }
//     }
// }

// impl crate::traits::get_code_occurence::GetCodeOccurence for FiveWrapperError {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
//         match self {
//             FiveWrapperError::Something { sources, code_occurence } => code_occurence
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Error)]
// pub enum FiveWrapperErrorEnum {
//     FiveOneOrigin(FiveOneOriginError),
// }

// impl std::fmt::Display for FiveWrapperErrorEnum {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<ConfigGeneric> crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfig<ConfigGeneric> for FiveWrapperErrorEnum
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         use crate::traits::error_logs_logic::origin_to_string_with_config::OriginToStringWithConfig;
//         match self {
//             FiveWrapperErrorEnum::FiveOneOrigin(i) => i.origin_to_string_with_config(config),
//         }
//     }
// }

// impl crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig for FiveWrapperErrorEnum {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             FiveWrapperErrorEnum::FiveOneOrigin(i) => i.to_string_without_config(),
//         }
//     }
// }

// pub fn five() -> Result<(), Box<FiveWrapperError>> {
//     if let Err(e) = five_one() {
//         let f = FiveWrapperError::Something {
//             sources: std::collections::HashMap::from([
//                 (
//                     String::from("five_one_hashmap_key"),
//                     FiveWrapperErrorEnum::FiveOneOrigin(*e),
//                 )
//             ]),
//             code_occurence: crate::common::code_occurence::CodeOccurence::new(
//                 once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
//                     String::from(file!()),
//                     line!(),
//                     column!(),
//                 )
//         };
//         // println!("++++++++++");
//         // println!("{}", f);
//         // f.error_log(once_cell::sync::Lazy::force(
//         //     &crate::global_variables::runtime::config::CONFIG,
//         // ));
//         // println!("+++++++++");
//         return Err(Box::new(f));
//     }
//     Ok(())
// }

// #[derive(Debug, Serialize, Deserialize, Error)]
// pub enum FiveOneOriginError {
//     Something {
//         error: String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// //cannot make it with generics
// impl std::fmt::Display for FiveOneOriginError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig for FiveOneOriginError {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             FiveOneOriginError::Something { error, code_occurence } => format!("{}", error),
//         }
//     }
// }

// impl crate::traits::get_code_occurence::GetCodeOccurence for FiveOneOriginError {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
//         match self {
//             FiveOneOriginError::Something { error, code_occurence } => code_occurence,
//         }
//     }
// }

// pub fn five_one() -> Result<(), Box<FiveOneOriginError>> {
//     return Err(Box::new(FiveOneOriginError::Something {
//         error: String::from("five_one error"),
//         code_occurence: crate::common::code_occurence::CodeOccurence::new(
//             once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
//                 String::from(file!()),
//                 line!(),
//                 column!(),
//             )
//     }));
// }

// #[derive(Debug, Serialize, Deserialize, Error)]
// pub enum SixWrapperError {
//     Something {
//         //todo how to implement from for it?
//         sources: Vec<SixWrapperErrorEnum>,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     }
// }
// //cannot make it with generics
// impl std::fmt::Display for SixWrapperError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<ConfigGeneric> crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<ConfigGeneric> for SixWrapperError
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         use crate::traits::error_logs_logic::few_to_string_with_config::FewToStringWithConfig;
//         match self {
//             SixWrapperError::Something { sources, code_occurence } => sources.few_to_string_with_config(config),
//         }
//     }
// }

// impl crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig for SixWrapperError {
//     fn source_to_string_without_config(&self) -> String {
//         use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
//         match self {
//             SixWrapperError::Something { sources, code_occurence } => sources.few_to_string_without_config(),
//         }
//     }
// }

// impl crate::traits::get_code_occurence::GetCodeOccurence for SixWrapperError {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
//         match self {
//             SixWrapperError::Something { sources, code_occurence } => code_occurence,
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Error)]
// pub enum SixWrapperErrorEnum {
//     SevenWrapper(SevenOriginError),
//     EightWrapper(EightOriginError),
// }

// impl std::fmt::Display for SixWrapperErrorEnum {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl<ConfigGeneric> crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfig<ConfigGeneric> for SixWrapperErrorEnum
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         use crate::traits::error_logs_logic::origin_to_string_with_config::OriginToStringWithConfig;
//         match self {
//             SixWrapperErrorEnum::SevenWrapper(i) => i.origin_to_string_with_config(config),
//             SixWrapperErrorEnum::EightWrapper(i) => i.origin_to_string_with_config(config),
//         }
//     }
// }

// impl crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig for SixWrapperErrorEnum {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             SixWrapperErrorEnum::SevenWrapper(i) => i.to_string_without_config(),
//             SixWrapperErrorEnum::EightWrapper(i) => i.to_string_without_config(),
//         }
//     }
// }

// pub fn six() -> Result<(), Box<SixWrapperError>> {
//     let thread_join_handle = std::thread::spawn(move || eight());
//     let res = thread_join_handle.join().unwrap();
//     match (seven(), res) {
//         (Ok(_), Ok(_)) => todo!(),
//         (Ok(_), Err(_)) => todo!(),
//         (Err(_), Ok(_)) => todo!(),
//         (Err(seven_error), Err(eight_error)) => {
//             let f = SixWrapperError::Something {
//                 sources: vec![SixWrapperErrorEnum::SevenWrapper(*seven_error), SixWrapperErrorEnum::EightWrapper(*eight_error)],
//                 code_occurence: crate::common::code_occurence::CodeOccurence::new(
//                     once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
//                     String::from(file!()),
//                     line!(),
//                     column!(),
//                 )
//             };
//             // println!("------");
//             // println!("{}", f);
//             // f.error_log(once_cell::sync::Lazy::force(
//             //     &crate::global_variables::runtime::config::CONFIG,
//             // ));
//             // println!("------");
//             return Err(Box::new(f));
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Error)]
// pub enum SevenOriginError {
//     Something {
//         error: String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
// //cannot make it with generics
// impl std::fmt::Display for SevenOriginError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
//         write!(f, "{}", self.to_string_without_config())
//     }
// }

// impl crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig for SevenOriginError {
//     fn source_to_string_without_config(&self) -> String {
//         match self {
//             SevenOriginError::Something { error, code_occurence } => format!("{}", error),
//         }
//     }
// }

// impl crate::traits::get_code_occurence::GetCodeOccurence for SevenOriginError {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
//         match self {
//             SevenOriginError::Something { error, code_occurence } => code_occurence,
//         }
//     }
// }

// pub fn seven() -> Result<(), Box<SevenOriginError>> {
//     return Err(
//         Box::new(
//             SevenOriginError::Something { error: String::from("error_seven"), code_occurence: crate::common::code_occurence::CodeOccurence::new(
//             once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
//                 String::from(file!()),
//                 line!(),
//                 column!(),
//             )}
//         )
//     );
// }

#[derive(Debug, Error, Serialize)]
pub enum EightOriginError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetime<'a>,
    },
}

impl<'a> std::fmt::Display for EightOriginError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a>
    for EightOriginError<'a>
{
    fn source_to_string_without_config_lifetime(&self) -> String {
        match self {
            EightOriginError::Something {
                error,
                code_occurence,
            } => format!("{}", error),
        }
    }
}

impl<'a> crate::traits::get_code_occurence::GetCodeOccurenceLifetime<'a> for EightOriginError<'a> {
    fn get_code_occurence_lifetime(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetime<'a> {
        match self {
            EightOriginError::Something {
                error,
                code_occurence,
            } => code_occurence,
        }
    }
}

// #[derive(Debug, Error, Serialize, Deserialize)]
// pub enum EightOriginErrorWithSerializeDeserialize<'a> {
//     Something {
//         error: String,
//         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize<'a>,
//     },
// }

// impl<'a> std::fmt::Display for EightOriginErrorWithSerializeDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
//         write!(f, "{}", self.to_string_without_config_lifetime())
//     }
// }

// impl<'a>
//     crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a>
//     for EightOriginErrorWithSerializeDeserialize<'a>
// {
//     fn source_to_string_without_config_lifetime(&self) -> String {
//         match self {
//             EightOriginErrorWithSerializeDeserialize::Something {
//                 error,
//                 code_occurence,
//             } => format!("{}", error),
//         }
//     }
// }

// impl<'a> crate::traits::get_code_occurence::GetCodeOccurenceLifetimeWithSerializeDeserialize<'a>
//     for EightOriginErrorWithSerializeDeserialize<'a>
// {
//     fn get_code_occurence_lifetime(
//         &self,
//     ) -> &crate::common::code_occurence::CodeOccurenceLifetimeWithSerializeDeserialize<'a> {
//         match self {
//             EightOriginErrorWithSerializeDeserialize::Something {
//                 error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

pub fn eight() -> Result<(), Box<EightOriginError<'static>>> {
    let g = crate::common::code_occurence::CodeOccurenceLifetime::new(
        &crate::global_variables::compile_time::git_info::GIT_INFO,
        // once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
        file!(),
        line!(),
        column!(),
    );
    println!(
        "11 {}",
        std::mem::size_of::<crate::common::code_occurence::CodeOccurenceLifetime>()
    );
    println!("22 {}", std::mem::size_of_val(&g));
    let f = EightOriginError::Something {
        error: String::from("error_eight"),
        code_occurence: g,
    };
    println!("33 {}", std::mem::size_of::<EightOriginError>());
    println!("44 {}", std::mem::size_of_val(&f));
    return Err(Box::new(f));
}
