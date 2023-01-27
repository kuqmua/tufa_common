use crate::traits::error_display::{ErrorDisplayInner, ToStringHandle};
use crate::traits::get_code_occurence::GetCodeOccurenceAsString;
use crate::traits::get_source::GetSourceAsString;
use crate::traits::separator_symbol::SeparatorSymbol;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::vec;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Error)]
pub struct ThreeWrapperError {
    source: ThreeWrapperErrorEnum,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for ThreeWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let config =
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG);
        write!(
            f,
            "{}{}{}",
            self.source,
            config.symbol(),
            self.get_code_occurence_as_string(config),
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for ThreeWrapperError
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        self.source.get_source_as_string(config)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for ThreeWrapperError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

pub fn three() -> Result<(), Box<ThreeWrapperError>> {
    if let Err(e) = four() {
        return Err(Box::new(ThreeWrapperError {
            source: ThreeWrapperErrorEnum::FourWrapper(*e),
            code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
                once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    String::from(file!()),
                    line!(),
                    column!(),
                )
        }));
    };
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum ThreeWrapperErrorEnum {
    FourWrapper(FourWrapperError),
}

impl std::fmt::Display for ThreeWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(e) => write!(f, "{}", e),
        }
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for ThreeWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => i.get_source_as_string(config),
        }
    }
}

impl<ConfigGeneric> GetCodeOccurenceAsString<ConfigGeneric> for ThreeWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => i.get_code_occurence_as_string(config),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub struct FourWrapperError {
    //todo how to implement from for it?
    sources: HashMap<String, FourWrapperErrorEnum>,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for FourWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let config =
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG);
        write!(
            f,
            "{}{}",
            self.sources.to_string_handle(config),
            self.get_code_occurence_as_string(config),
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FourWrapperError
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        self.sources.get_source_as_string(config)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for FourWrapperError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum FourWrapperErrorEnum {
    FiveWrapper(FiveWrapperError),
    SixWrapper(SixWrapperError),
}

impl std::fmt::Display for FourWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FourWrapperErrorEnum::FiveWrapper(e) => write!(f, "{}", e),
            FourWrapperErrorEnum::SixWrapper(e) => write!(f, "{}", e),
        }
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FourWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => i.get_source_as_string(config),
            FourWrapperErrorEnum::SixWrapper(i) => i.get_source_as_string(config),
        }
    }
}

impl<ConfigGeneric> GetCodeOccurenceAsString<ConfigGeneric> for FourWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => i.get_code_occurence_as_string(config),
            FourWrapperErrorEnum::SixWrapper(i) => i.get_code_occurence_as_string(config),
        }
    }
}

pub fn four() -> Result<(), Box<FourWrapperError>> {
    match (five(), six()) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            return Err(Box::new(FourWrapperError {
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
            }));
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub struct FiveWrapperError {
    //todo how to implement from for it?
    sources: HashMap<String, FiveWrapperErrorEnum>,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for FiveWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let config =
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG);
        write!(
            f,
            "{}{}",
            self.sources.to_string_handle(config),
            self.get_code_occurence_as_string(config),
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FiveWrapperError
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        self.sources.get_source_as_string(config)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for FiveWrapperError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum FiveWrapperErrorEnum {
    FiveOneOrigin(FiveOneOriginError),
}

impl std::fmt::Display for FiveWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(e) => write!(f, "{}", e),
        }
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FiveWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetLogType + crate::traits::fields::GetSourcePlaceType,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => i.get_source_as_string(config),
        }
    }
}

impl<ConfigGeneric> GetCodeOccurenceAsString<ConfigGeneric> for FiveWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => i.get_code_occurence_as_string(config),
        }
    }
}

pub fn five() -> Result<(), Box<FiveWrapperError>> {
    if let Err(e) = five_one() {
        return Err(Box::new(FiveWrapperError {
            sources: HashMap::from([
                (
                    String::from("five_one_hashmap key"),
                    FiveWrapperErrorEnum::FiveOneOrigin(*e),
                )
            ]),
            code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
                once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    String::from(file!()),
                    line!(),
                    column!(),
                )
        }));
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub struct FiveOneOriginError {
    error: String,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for FiveOneOriginError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.error_display_inner(once_cell::sync::Lazy::force(
                &crate::global_variables::runtime::config::CONFIG
            ))
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FiveOneOriginError
where
    ConfigGeneric: crate::traits::fields::GetLogType + crate::traits::fields::GetSourcePlaceType,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        format!("{}", self.error)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for FiveOneOriginError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

pub fn five_one() -> Result<(), Box<FiveOneOriginError>> {
    return Err(Box::new(FiveOneOriginError {
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
pub struct SixWrapperError {
    //todo how to implement from for it?
    sources: Vec<SixWrapperErrorEnum>,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for SixWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let config =
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG);
        let symbol = config.symbol();
        write!(
            f,
            "{}{}{}",
            self.sources.to_string_handle(config),
            symbol,
            self.get_code_occurence_as_string(config),
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for SixWrapperError
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        self.sources.get_source_as_string(config)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for SixWrapperError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
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
            return Err(Box::new(SixWrapperError {
                sources: vec![SixWrapperErrorEnum::SevenWrapper(*seven_error), SixWrapperErrorEnum::EightWrapper(*eight_error)],
                code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
                    once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    String::from(file!()),
                    line!(),
                    column!(),
                )
            }));
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum SixWrapperErrorEnum {
    SevenWrapper(SevenOriginError),
    EightWrapper(EightOriginError),
}

impl std::fmt::Display for SixWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SixWrapperErrorEnum::SevenWrapper(e) => write!(f, "{}", e),
            SixWrapperErrorEnum::EightWrapper(e) => write!(f, "{}", e),
        }
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for SixWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetLogType + crate::traits::fields::GetSourcePlaceType,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            //todo - if wrapper - with config, if origin - without
            SixWrapperErrorEnum::SevenWrapper(i) => i.get_source_as_string(config),
            SixWrapperErrorEnum::EightWrapper(i) => i.get_source_as_string(config),
        }
    }
}

impl<ConfigGeneric> GetCodeOccurenceAsString<ConfigGeneric> for SixWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            SixWrapperErrorEnum::SevenWrapper(i) => i.get_code_occurence_as_string(config),
            SixWrapperErrorEnum::EightWrapper(i) => i.get_code_occurence_as_string(config),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub struct SevenOriginError {
    error: String,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for SevenOriginError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.error_display_inner(once_cell::sync::Lazy::force(
                &crate::global_variables::runtime::config::CONFIG
            ))
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for SevenOriginError
where
    ConfigGeneric: crate::traits::fields::GetLogType + crate::traits::fields::GetSourcePlaceType,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        format!("{}", self.error)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for SevenOriginError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

pub fn seven() -> Result<(), Box<SevenOriginError>> {
    return Err(Box::new(SevenOriginError {
        error: String::from("error_seven"),
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                String::from(file!()),
                line!(),
                column!(),
            )
    }));
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub struct EightOriginError {
    error: String,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for EightOriginError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.error_display_inner(once_cell::sync::Lazy::force(
                &crate::global_variables::runtime::config::CONFIG
            ))
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for EightOriginError
where
    ConfigGeneric: crate::traits::fields::GetLogType + crate::traits::fields::GetSourcePlaceType,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        format!("{}", self.error)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for EightOriginError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

pub fn eight() -> Result<(), Box<EightOriginError>> {
    return Err(Box::new(EightOriginError {
        error: String::from("error_eight"),
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                String::from(file!()),
                line!(),
                column!(),
            )
    }));
}
