use crate::traits::error_display::{ErrorDisplayInner, ToStringHandle};
use crate::traits::error_log::ErrorLog;
use crate::traits::get_code_occurence::GetCodeOccurenceAsString;
use crate::traits::get_source::GetSourceAsString;
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
        write!(f, "{}", self.source)
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
        format!(
            "{}\n{}",
            self.source.to_string_handle(config),
            self.get_code_occurence_as_string(config),
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for ThreeWrapperError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
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
        let f = ThreeWrapperError {
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
    FourWrapper(FourWrapperError),
}

impl std::fmt::Display for ThreeWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(e) => write!(f, "{}", e),
        }
    }
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

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for ThreeWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
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
        let mut source_as_string =
            self.sources
                .iter()
                .fold(String::from(""), |mut acc, (key, value)| {
                    acc.push_str(&format!("[key: {}]\n {}\n", key, value));
                    acc
                });
        source_as_string.pop();
        write!(f, "{}", source_as_string)
    }
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric> for FourWrapperError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        format!(
            "{}{}",
            self.sources.to_string_handle(config),
            self.get_code_occurence_as_string(config),
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FourWrapperError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
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

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FourWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
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
            let f = FourWrapperError {
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
            // println!("four");
            // f.error_log(once_cell::sync::Lazy::force(
            //     &crate::global_variables::runtime::config::CONFIG,
            // ));
            // println!("fourend");
            return Err(Box::new(f));
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
        let mut source_as_string =
            self.sources
                .iter()
                .fold(String::from(""), |mut acc, (key, value)| {
                    acc.push_str(&format!("[key: \n]{} \n{}", key, value));
                    acc
                });
        source_as_string.pop();
        write!(f, "{}", source_as_string)
    }
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric> for FiveWrapperError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        format!(
            "{}{}",
            self.sources.to_string_handle(config),
            self.get_code_occurence_as_string(config),
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FiveWrapperError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
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

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FiveWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType,
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
        let f = FiveWrapperError {
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
        };
        // println!("five");
        // f.error_log(once_cell::sync::Lazy::force(
        //     &crate::global_variables::runtime::config::CONFIG,
        // ));
        // println!("fiveend");
        return Err(Box::new(f));
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
        write!(f, "{}", self.error)
    }
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric>
    for FiveOneOriginError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        self.error_display_inner(config)
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FiveOneOriginError {
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
        let mut source_as_string =
            self.sources
                .iter()
                .fold(String::from(""), |mut acc, vec_element| {
                    acc.push_str(&format!("{}\n", vec_element));
                    acc
                });
        source_as_string.pop();
        write!(f, "{}", source_as_string)
    }
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric> for SixWrapperError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        format!(
            "{}\n{}",
            self.sources.to_string_handle(config),
            self.get_code_occurence_as_string(config),
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for SixWrapperError
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
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
            let f = SixWrapperError {
                sources: vec![SixWrapperErrorEnum::SevenWrapper(*seven_error), SixWrapperErrorEnum::EightWrapper(*eight_error)],
                code_occurence: crate::common::code_occurence::CodeOccurenceOldWay::new(
                    once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    String::from(file!()),
                    line!(),
                    column!(),
                )
            };
            // println!("------");
            // f.error_log(once_cell::sync::Lazy::force(
            //     &crate::global_variables::runtime::config::CONFIG,
            // ));
            // println!("------");
            return Err(Box::new(f));
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

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for SixWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType,
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
        write!(f, "{}", self.error)
    }
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

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for SevenOriginError {
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
        write!(f, "{}", self.error)
    }
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

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for EightOriginError {
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

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
