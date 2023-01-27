use crate::traits::error_display::{ErrorDisplayInner, ToStringHandle};
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
        write!(
            f,
            "{}\n{}",
            self.source,
            self.get_code_occurence_as_string(),
        )
    }
}

impl GetSourceAsString for ThreeWrapperError {
    fn get_source_as_string(&self) -> String {
        self.source.get_source_as_string()
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

impl GetSourceAsString for ThreeWrapperErrorEnum {
    fn get_source_as_string(&self) -> String {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => i.get_source_as_string(),
        }
    }
}

impl GetCodeOccurenceAsString for ThreeWrapperErrorEnum {
    fn get_code_occurence_as_string(&self) -> String {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => i.get_code_occurence_as_string(),
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
        write!(
            f,
            "{}{}",
            self.sources.to_string_handle(),
            self.get_code_occurence_as_string(),
        )
    }
}

impl GetSourceAsString for FourWrapperError {
    fn get_source_as_string(&self) -> String {
        self.sources.get_source_as_string()
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

impl GetSourceAsString for FourWrapperErrorEnum {
    fn get_source_as_string(&self) -> String {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => i.get_source_as_string(),
            FourWrapperErrorEnum::SixWrapper(i) => i.get_source_as_string(),
        }
    }
}

impl GetCodeOccurenceAsString for FourWrapperErrorEnum {
    fn get_code_occurence_as_string(&self) -> String {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => i.get_code_occurence_as_string(),
            FourWrapperErrorEnum::SixWrapper(i) => i.get_code_occurence_as_string(),
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
        write!(
            f,
            "{}{}",
            self.sources.to_string_handle(),
            self.get_code_occurence_as_string(),
        )
    }
}

impl GetSourceAsString for FiveWrapperError {
    fn get_source_as_string(&self) -> String {
        self.sources.get_source_as_string()
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

impl GetSourceAsString for FiveWrapperErrorEnum {
    fn get_source_as_string(&self) -> String {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => i.get_source_as_string(),
        }
    }
}

impl GetCodeOccurenceAsString for FiveWrapperErrorEnum {
    fn get_code_occurence_as_string(&self) -> String {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => i.get_code_occurence_as_string(),
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

impl GetSourceAsString for FiveOneOriginError {
    fn get_source_as_string(&self) -> String {
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
        write!(
            f,
            "{}\n{}",
            self.sources.to_string_handle(),
            self.get_code_occurence_as_string(),
        )
    }
}

impl GetSourceAsString for SixWrapperError {
    fn get_source_as_string(&self) -> String {
        self.sources.get_source_as_string()
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

impl GetSourceAsString for SixWrapperErrorEnum {
    fn get_source_as_string(&self) -> String {
        match self {
            //todo - if wrapper - with config, if origin - without
            SixWrapperErrorEnum::SevenWrapper(i) => i.get_source_as_string(),
            SixWrapperErrorEnum::EightWrapper(i) => i.get_source_as_string(),
        }
    }
}

impl GetCodeOccurenceAsString for SixWrapperErrorEnum {
    fn get_code_occurence_as_string(&self) -> String {
        match self {
            SixWrapperErrorEnum::SevenWrapper(i) => i.get_code_occurence_as_string(),
            SixWrapperErrorEnum::EightWrapper(i) => i.get_code_occurence_as_string(),
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

impl GetSourceAsString for SevenOriginError {
    fn get_source_as_string(&self) -> String {
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

impl GetSourceAsString for EightOriginError {
    fn get_source_as_string(&self) -> String {
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
