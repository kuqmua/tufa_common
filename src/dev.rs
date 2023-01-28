use crate::traits::error_display::{ErrorDisplayInner, ToStringHandle};
use crate::traits::error_log::ErrorLog;
use crate::traits::get_code_occurence::GetCodeOccurenceOldWay;
use crate::traits::get_source::GetOriginSourceAsString;
use crate::traits::error_display::ToStringHandleWithoutConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::vec;
use thiserror::Error;
use crate::traits::error_display::ToStringHandleCodeOccurence;

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum ThreeWrapperError {
    #[error("{source}\n{code_occurence}")]
    Something {
        source: ThreeWrapperErrorEnum,
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
    }
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric>
    for ThreeWrapperError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        match self {
            ThreeWrapperError::Something { source, code_occurence } => format!(
                "{}\n{}",
                source.to_string_handle(config),
                self.get_code_occurence_old_way().to_string_handle_code_occurence(config),
            ),
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for ThreeWrapperError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        match self {
            ThreeWrapperError::Something { source, code_occurence } => code_occurence,
        }
    }
}

pub fn three() -> Result<(), Box<ThreeWrapperError>> {
    if let Err(e) = four() {
        let f = ThreeWrapperError::Something { 
            source: ThreeWrapperErrorEnum::FourWrapper(*e), 
            code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
                once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    String::from(file!()),
                    line!(),
                    column!(),
                )
        };
        // println!("three");
        // f.error_log(once_cell::sync::Lazy::force(
        //     &crate::global_variables::runtime::config::CONFIG,
        // ));
        // println!("threeend");
        return Err(Box::new(f));
    };
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum ThreeWrapperErrorEnum {
    #[error("{0}")]
    FourWrapper(FourWrapperError),
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric>
    for ThreeWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => i.to_string_handle(config),
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for ThreeWrapperErrorEnum
{
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => i.get_code_occurence_old_way(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum FourWrapperError {
    Something {
        //todo how to implement from for it?
        sources: HashMap<String, FourWrapperErrorEnum>,
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
    }
}

impl std::fmt::Display for FourWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FourWrapperError::Something { sources, code_occurence } => write!(f, "{}{}", sources.to_string_handle_without_config(), code_occurence),
        }
    }
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric> for FourWrapperError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        match self {
            FourWrapperError::Something { sources, code_occurence } => {
                format!(
                    "{}{}",
                    sources.to_string_handle(config),
                    self.get_code_occurence_old_way().to_string_handle_code_occurence(config),
                )
            },
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for FourWrapperError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        match self {
            FourWrapperError::Something { sources, code_occurence } => code_occurence,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum FourWrapperErrorEnum {
    #[error("{0}")]
    FiveWrapper(FiveWrapperError),
    #[error("{0}")]
    SixWrapper(SixWrapperError),
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric>
    for FourWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => i.to_string_handle(config),
            FourWrapperErrorEnum::SixWrapper(i) => i.to_string_handle(config),
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for FourWrapperErrorEnum
{
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => i.get_code_occurence_old_way(),
            FourWrapperErrorEnum::SixWrapper(i) => i.get_code_occurence_old_way(),
        }
    }
}

pub fn four() -> Result<(), Box<FourWrapperError>> {
    match (five(), six()) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            let f = FourWrapperError::Something { 
                sources: HashMap::from([
                    (
                        String::from("five_hashmap_key"),
                        FourWrapperErrorEnum::FiveWrapper(*f),
                    ),
                    (
                        String::from("six_hashmap_key"),
                        FourWrapperErrorEnum::SixWrapper(*s),
                    ),
                ]), 
                code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
                    once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                        String::from(file!()),
                        line!(),
                        column!(),
                    )
            };
            // println!("=======");
            // println!("{}", f);
            // f.error_log(once_cell::sync::Lazy::force(
            //     &crate::global_variables::runtime::config::CONFIG,
            // ));
            // println!("=======");
            return Err(Box::new(f));
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum FiveWrapperError {
    Something{
        //todo how to implement from for it?
        sources: HashMap<String, FiveWrapperErrorEnum>,
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
    }
}

impl std::fmt::Display for FiveWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FiveWrapperError::Something { sources, code_occurence } => write!(f, "{}{}", sources.to_string_handle_without_config(), code_occurence),
        }
    }
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric> for FiveWrapperError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        match self {
            FiveWrapperError::Something { sources, code_occurence } => format!(
                "{}{}",
                sources.to_string_handle(config),
                self.get_code_occurence_old_way().to_string_handle_code_occurence(config),
            ),
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for FiveWrapperError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        match self {
            FiveWrapperError::Something { sources, code_occurence } => code_occurence
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum FiveWrapperErrorEnum {
    #[error("{0}")]
    FiveOneOrigin(FiveOneOriginError),
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric>
    for FiveWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => i.to_string_handle(config),
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for FiveWrapperErrorEnum
{
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => i.get_code_occurence_old_way(),
        }
    }
}

pub fn five() -> Result<(), Box<FiveWrapperError>> {
    if let Err(e) = five_one() {
        let f = FiveWrapperError::Something { 
            sources: HashMap::from([
                (
                    String::from("five_one_hashmap_key"),
                    FiveWrapperErrorEnum::FiveOneOrigin(*e),
                )
            ]), 
            code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
                once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    String::from(file!()),
                    line!(),
                    column!(),
                ) 
        };
        // println!("++++++++++");
        // println!("{}", f);
        // f.error_log(once_cell::sync::Lazy::force(
        //     &crate::global_variables::runtime::config::CONFIG,
        // ));
        // println!("+++++++++");
        return Err(Box::new(f));
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum FiveOneOriginError {
    #[error("{error}\n{code_occurence}")]
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
    },
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric> for FiveOneOriginError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        self.error_display_inner(config)
    }
}

impl<ConfigGeneric> GetOriginSourceAsString<ConfigGeneric> for FiveOneOriginError {
    fn get_origin_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            FiveOneOriginError::Something { error, code_occurence } => format!("{}", error),
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for FiveOneOriginError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        match self {
            FiveOneOriginError::Something { error, code_occurence } => code_occurence,
        }
    }
}

pub fn five_one() -> Result<(), Box<FiveOneOriginError>> {
    return Err(Box::new(FiveOneOriginError::Something { 
        error: String::from("five_one error"), 
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                String::from(file!()),
                line!(),
                column!(),
            )
    }));
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum SixWrapperError {
    // #[error("{sources:#?}")]
    Something {
        //todo how to implement from for it?
        sources: Vec<SixWrapperErrorEnum>,
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
    }
}

impl std::fmt::Display for SixWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SixWrapperError::Something { sources, code_occurence } => write!(f, "{}", sources.to_string_handle_without_config()),
        }
    }
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric> for SixWrapperError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        match self {
            SixWrapperError::Something { sources, code_occurence } => format!(
                "{}\n{}",
                sources.to_string_handle(config),
                self.get_code_occurence_old_way().to_string_handle_code_occurence(config),
            ),
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for SixWrapperError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        match self {
            SixWrapperError::Something { sources, code_occurence } => code_occurence,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum SixWrapperErrorEnum {
    #[error("{0}")]
    SevenWrapper(SevenOriginError),
    #[error("{0}")]
    EightWrapper(EightOriginError),
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric>
    for SixWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        match self {
            SixWrapperErrorEnum::SevenWrapper(i) => i.to_string_handle(config),
            SixWrapperErrorEnum::EightWrapper(i) => i.to_string_handle(config),
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for SixWrapperErrorEnum
{
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        match self {
            SixWrapperErrorEnum::SevenWrapper(i) => i.get_code_occurence_old_way(),
            SixWrapperErrorEnum::EightWrapper(i) => i.get_code_occurence_old_way(),
        }
    }
}

pub fn six() -> Result<(), Box<SixWrapperError>> {
    let thread_join_handle = std::thread::spawn(move || eight());
    // some work here
    let res = thread_join_handle.join().unwrap();
    match (seven(), res) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let f = SixWrapperError::Something { 
                sources: vec![SixWrapperErrorEnum::SevenWrapper(*seven_error), SixWrapperErrorEnum::EightWrapper(*eight_error)], 
                code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
                    once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    String::from(file!()),
                    line!(),
                    column!(),
                ) 
            };
            // println!("------");
            // println!("{}", f);
            // f.error_log(once_cell::sync::Lazy::force(
            //     &crate::global_variables::runtime::config::CONFIG,
            // ));
            // println!("------");
            return Err(Box::new(f));
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum SevenOriginError {
    #[error("{error}\n{code_occurence}")]
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
    },
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric> for SevenOriginError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        self.error_display_inner(config)
    }
}

impl<ConfigGeneric> GetOriginSourceAsString<ConfigGeneric> for SevenOriginError {
    fn get_origin_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            SevenOriginError::Something { error, code_occurence } => format!("{}", error),
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for SevenOriginError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        match self {
            SevenOriginError::Something { error, code_occurence } => code_occurence,
        }
    }
}

pub fn seven() -> Result<(), Box<SevenOriginError>> {
    return Err(
        Box::new(
            SevenOriginError::Something { error: String::from("error_seven"), code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                String::from(file!()),
                line!(),
                column!(),
            )} 
        )
    );
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum EightOriginError {
    #[error("{error}\n{code_occurence}")]
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
    },
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric> for EightOriginError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        self.error_display_inner(config)
    }
}

impl<ConfigGeneric> GetOriginSourceAsString<ConfigGeneric> for EightOriginError {
    fn get_origin_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            EightOriginError::Something { error, code_occurence } => format!("{}", error),
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for EightOriginError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        match self {
            EightOriginError::Something { error, code_occurence } => code_occurence,
        }
    }
}

pub fn eight() -> Result<(), Box<EightOriginError>> {
    return Err(
        Box::new(
            EightOriginError::Something { error: String::from("error_eight"), code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                String::from(file!()),
                line!(),
                column!(),
            )} 
        )
    );
}