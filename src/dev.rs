use crate::common::file_line_column::FileLineColumn;
use crate::common::git::git_info::GitInformation;
use crate::common::git::git_info::GitInformationWithoutLifetimes;
use crate::common::source_and_code_occurence;
use crate::common::time_file_line_column;
use crate::common::time_file_line_column::TimeFileLineColumn;
use crate::common::where_was::WhereWas;
use crate::config_mods::log_type::LogType;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::global_variables::runtime::config::CONFIG;
use crate::traits::code_occurence_methods;
use crate::traits::code_occurence_methods::CodeOccurenceToString;
use crate::traits::code_path::CodePath;
use crate::traits::console::Console;
use crate::traits::fields::GetLogType;
use crate::traits::fields::GetSourcePlaceType;
use crate::traits::get_code_occurence::GetCodeOccurence;
use crate::traits::get_color::ErrorColorBold;
use crate::traits::get_git_source_file_link::GetGitSourceFileLink;
use crate::traits::get_source_and_code_occurence;
use crate::traits::get_source_and_code_occurence::GetSourceAndCodeOccurence;
use crate::traits::get_source_value::GetSourceValue;
use crate::traits::init_error::InitError;
use crate::traits::new_error_with_one_addition::NewErrorWithOneAddition;
use crate::traits::readable_time::ReadableTime;
use crate::traits::readable_time_string::ReadableTimeString;
use crate::traits::separator_symbol::SeparatorSymbol;
use ansi_term::Colour::RGB;
use chrono::prelude::DateTime;
use chrono::Utc;
use impl_get_source::ImplGetSourceFromCrate;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::format;
use std::fmt::{self, Display};
use std::vec;

#[derive(Debug)]
pub struct ThreeWrapperError {
    source: ThreeWrapperErrorEnum,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl ThreeWrapperError {
    pub fn get_source_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        format!("{}", self.source.get_source_as_string(config))
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        self.code_occurence.time_file_line_column.get_code_path(
            &self.code_occurence.git_info,
            config.get_source_place_type(),
        )
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct, //todo maybe remove
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let mut sources_for_tracing: Vec<Vec<(String, Vec<String>)>> = vec![];
        let mut vec = self
            .source
            .get_inner_source_and_code_occurence_as_string(config);
        vec.iter_mut().for_each(|n| {
            n.increment += 1;
            n.source.iter().for_each(|f| {
                sources_for_tracing.push(f.clone());
            });
        });
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: sources_for_tracing.clone(),
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        vec
    }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        log_type.console(
            &config.get_error_color_bold(),
            format!(
                "{}{}{}",
                self.get_source_as_string(config),
                log_type.symbol(),
                self.get_code_occurence_as_string(config)
            ),
        )
    }
}

use crate::traits::my_custom_display::DisplayError;
pub fn three(should_trace: bool) -> Result<(), Box<ThreeWrapperError>> {
    if let Err(e) = four(false) {
        let f = ThreeWrapperError {
            source: ThreeWrapperErrorEnum::FourWrapper(*e),
            code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                    String::from(file!()),
                    line!(),
                    column!(),
                ),
            }
        };
        return Err(Box::new(f));
    };
    Ok(())
}

#[derive(Debug)]
pub enum ThreeWrapperErrorEnum {
    FourWrapper(FourWrapperError),
}

impl ThreeWrapperErrorEnum {
    pub fn get_source_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => i.get_source_as_string(config),
        }
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => i.get_code_occurence_as_string(config),
        }
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => {
                i.get_inner_source_and_code_occurence_as_string(config)
            }
        }
    }
}

#[derive(Debug)]
pub struct FourWrapperError {
    source: HashMap<String, FourWrapperErrorEnum>,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl FourWrapperError {
    pub fn get_source_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        let log_type = config.get_log_type();
        let symbol = log_type.symbol();
        let mut source_as_string =
            self.source
                .iter()
                .fold(String::from(""), |mut acc, (key, value)| {
                    let source_as_string = value.get_source_as_string(config);
                    let get_code_occurence_as_string = value.get_code_occurence_as_string(config);
                    acc.push_str(&format!(
                        "[key: {}]{} {}{} {}{}",
                        key, symbol, source_as_string, symbol, get_code_occurence_as_string, symbol
                    ));
                    acc
                });
        log_type.pop_last(&mut source_as_string);
        source_as_string
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        self.code_occurence.time_file_line_column.get_code_path(
            &self.code_occurence.git_info,
            config.get_source_place_type(),
        )
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let len = self.source.len() + 1;
        let mut sources_for_tracing: Vec<Vec<(String, Vec<String>)>> = vec![];
        let mut vec = self
            .source
            .iter()
            .fold(Vec::with_capacity(len), |mut acc, (key, value)| {
                value
                    .get_inner_source_and_code_occurence_as_string(config)
                    .iter()
                    .for_each(|e| {
                        e.source.iter().for_each(|hm| {
                            let mut new_hm = Vec::new();
                            hm.iter().for_each(|(k, v)| {
                                let mut new_v = v.clone();
                                new_v.push(key.clone());
                                new_hm.push((k.clone(), new_v.clone()));
                            });
                            sources_for_tracing.push(new_hm.clone());
                        });
                        acc.push(e.clone().add_one());
                    });
                acc
            });
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: sources_for_tracing.clone(),
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        vec
    }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        log_type.console(
            &config.get_error_color_bold(),
            format!(
                "{}{}{}",
                self.get_source_as_string(config),
                log_type.symbol(),
                self.get_code_occurence_as_string(config)
            ),
        )
    }
}

#[derive(Debug)]
pub enum FourWrapperErrorEnum {
    FiveWrapper(FiveWrapperError),
    SixWrapper(SixWrapperError),
}

impl FourWrapperErrorEnum {
    pub fn get_source_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        match self {
            //todo - if wrapper - with config, if origin - without
            FourWrapperErrorEnum::FiveWrapper(i) => i.get_source_as_string(config),
            FourWrapperErrorEnum::SixWrapper(i) => i.get_source_as_string(config),
        }
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => i.get_code_occurence_as_string(config),
            FourWrapperErrorEnum::SixWrapper(i) => i.get_code_occurence_as_string(config),
        }
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => {
                i.get_inner_source_and_code_occurence_as_string(config)
            }
            FourWrapperErrorEnum::SixWrapper(i) => {
                i.get_inner_source_and_code_occurence_as_string(config)
            }
        }
    }
}
use crate::traits::get_source::GetSource;

use crate::traits::code_occurence_methods::CodeOccurenceNew;
pub fn four(should_trace: bool) -> Result<(), Box<FourWrapperError>> {
    match (five(false), six(false)) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            let f = FourWrapperError {
                source: HashMap::from([
                    (
                        String::from("five_hashmap_key"),
                        FourWrapperErrorEnum::FiveWrapper(*f),
                    ),
                    (
                        String::from("six_hashmap_key"),
                        FourWrapperErrorEnum::SixWrapper(*s),
                    ),
                ]),
                code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                    git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                        String::from(file!()),
                        line!(),
                        column!(),
                    ),
                }
            };
            return Err(Box::new(f));
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct FiveWrapperError {
    source: HashMap<String, FiveWrapperErrorEnum>,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl FiveWrapperError {
    pub fn get_source_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        let log_type = config.get_log_type();
        let symbol = log_type.symbol();
        let mut source_as_string =
            self.source
                .iter()
                .fold(String::from(""), |mut acc, (key, value)| {
                    let source_as_string = value.get_source_as_string(config);
                    let get_code_occurence_as_string = value.get_code_occurence_as_string(config);
                    //todo maybe space symbol
                    acc.push_str(&format!(
                        "[key: {}]{} {}{} {}{}",
                        key, symbol, source_as_string, symbol, get_code_occurence_as_string, symbol
                    ));
                    acc
                });
        log_type.pop_last(&mut source_as_string);
        source_as_string
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        self.code_occurence.time_file_line_column.get_code_path(
            &self.code_occurence.git_info,
            config.get_source_place_type(),
        )
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let len = self.source.len() + 1;
        let mut sources_for_tracing: Vec<Vec<(String, Vec<String>)>> = vec![];
        let mut vec = self
            .source
            .iter()
            .fold(Vec::with_capacity(len), |mut acc, (key, value)| {
                value
                    .get_inner_source_and_code_occurence_as_string(config)
                    .into_iter()
                    .for_each(|e| {
                        e.source.iter().for_each(|hm| {
                            let mut hm_handle = Vec::new(); //todo optimize
                            hm.iter().for_each(|(source_error, key_v)| {
                                let mut key_vv = key_v.clone();
                                key_vv.push(key.clone());
                                hm_handle.push((source_error.clone(), key_vv.clone()));
                            });
                            sources_for_tracing.push(hm_handle.clone());
                        });
                        acc.push(e.add_one());
                    });
                acc
            });
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: sources_for_tracing.clone(),
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        vec
    }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        log_type.console(
            &config.get_error_color_bold(),
            format!(
                "{}{}{}",
                self.get_source_as_string(config),
                log_type.symbol(),
                self.get_code_occurence_as_string(config)
            ),
        )
    }
}

#[derive(Debug)]
pub enum FiveWrapperErrorEnum {
    FiveOneOrigin(FiveOneOriginError),
}

impl FiveWrapperErrorEnum {
    pub fn get_source_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => i.get_source_as_string(),
        }
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => i.get_code_occurence_as_string(config),
        }
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => {
                i.get_inner_source_and_code_occurence_as_string(config)
            }
        }
    }
}

pub fn five(should_trace: bool) -> Result<(), Box<FiveWrapperError>> {
    let five_one_result = five_one(false);
    if let Err(e) = five_one_result {
        let mut hm = HashMap::new();
        hm.insert(
            String::from("five_one_hashmap key"),
            FiveWrapperErrorEnum::FiveOneOrigin(*e),
        );
        let f = FiveWrapperError {
            source: hm,
            code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
               String::from(file!()),
                    line!(),
                    column!(),
                ),
            }
        };
        return Err(Box::new(f));
    }
    Ok(())
}

#[derive(Debug)]
pub struct FiveOneOriginError {
    source: String,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl FiveOneOriginError {
    pub fn get_source_as_string(&self) -> String {
        format!("{}", self.source)
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        self.code_occurence.time_file_line_column.get_code_path(
            &self.code_occurence.git_info,
            config.get_source_place_type(),
        )
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct, //todo maybe remove
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        vec![
            crate::dev::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: vec![vec![(self.get_source_as_string(), vec![])]],
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        ]
    }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        log_type.console(
            &config.get_error_color_bold(),
            format!(
                "{}{}{}",
                self.get_source_as_string(),
                log_type.symbol(),
                self.get_code_occurence_as_string(config)
            ),
        )
    }
}

pub fn five_one(should_trace: bool) -> Result<(), Box<FiveOneOriginError>> {
    let f = FiveOneOriginError {
        source: String::from("five_one error"),
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
            git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
            time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                String::from(file!()),
                line!(),
                column!(),
            ),
        }
    };
    return Err(Box::new(f));
}

#[derive(Debug)]
pub struct SixWrapperError {
    source: Vec<SixWrapperErrorEnum>,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl SixWrapperError {
    pub fn get_source_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        let log_type = config.get_log_type();
        let symbol = log_type.symbol();
        let mut source_as_string =
            self.source
                .iter()
                .fold(String::from(""), |mut acc, vec_element| {
                    acc.push_str(&format!(
                        "{}{} {}{}",
                        vec_element.get_source_as_string(config),
                        symbol,
                        vec_element.get_code_occurence_as_string(config),
                        symbol
                    ));
                    acc
                });
        log_type.pop_last(&mut source_as_string);
        source_as_string
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        self.code_occurence.time_file_line_column.get_code_path(
            &self.code_occurence.git_info,
            config.get_source_place_type(),
        )
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let len = self.source.len() + 1;
        let mut sources_for_tracing: Vec<Vec<(String, Vec<String>)>> = vec![];
        let mut vec = self
            .source
            .iter()
            .fold(Vec::with_capacity(len), |mut acc, vec_element| {
                vec_element
                    .get_inner_source_and_code_occurence_as_string(config)
                    .into_iter()
                    .for_each(|e| {
                        e.source.iter().for_each(|f| {
                            sources_for_tracing.push(f.clone());
                        });
                        acc.push(e.add_one());
                    });
                acc
            });
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: sources_for_tracing,
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        vec
    }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        log_type.console(
            &config.get_error_color_bold(),
            format!(
                "{}{}{}",
                self.get_source_as_string(config),
                log_type.symbol(),
                self.get_code_occurence_as_string(config)
            ),
        )
    }
}

pub fn six(should_trace: bool) -> Result<(), Box<SixWrapperError>> {
    match (seven(false), eight(false)) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let f = SixWrapperError {
                source: vec![SixWrapperErrorEnum::SevenWrapper(*seven_error), SixWrapperErrorEnum::EightWrapper(*eight_error)],
                code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                    git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                        String::from(file!()),
                        line!(),
                        column!(),
                    ),
                }
            };
            return Err(Box::new(f));
        }
    }
    Ok(())
}

#[derive(Debug)]
pub enum SixWrapperErrorEnum {
    SevenWrapper(SevenOriginError),
    EightWrapper(EightOriginError),
}

impl SixWrapperErrorEnum {
    pub fn get_source_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        match self {
            //todo - if wrapper - with config, if origin - without
            SixWrapperErrorEnum::SevenWrapper(i) => i.get_source_as_string(),
            SixWrapperErrorEnum::EightWrapper(i) => i.get_source_as_string(),
        }
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        match self {
            SixWrapperErrorEnum::SevenWrapper(i) => i.get_code_occurence_as_string(config),
            SixWrapperErrorEnum::EightWrapper(i) => i.get_code_occurence_as_string(config),
        }
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        match self {
            SixWrapperErrorEnum::SevenWrapper(i) => {
                i.get_inner_source_and_code_occurence_as_string(config)
            }
            SixWrapperErrorEnum::EightWrapper(i) => {
                i.get_inner_source_and_code_occurence_as_string(config)
            }
        }
    }
}

#[derive(Debug)]
pub struct SevenOriginError {
    source: String,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl SevenOriginError {
    pub fn get_source_as_string(&self) -> String {
        format!("{}", self.source)
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        self.code_occurence.time_file_line_column.get_code_path(
            &self.code_occurence.git_info,
            config.get_source_place_type(),
        )
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct, //todo maybe remove
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        vec![
            crate::dev::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: vec![vec![(self.get_source_as_string(), vec![])]],
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        ]
    }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        log_type.console(
            &config.get_error_color_bold(),
            format!(
                "{}{}{}",
                self.get_source_as_string(),
                log_type.symbol(),
                self.get_code_occurence_as_string(config)
            ),
        )
    }
}

pub fn seven(should_trace: bool) -> Result<(), Box<SevenOriginError>> {
    let f = SevenOriginError {
        source: String::from("error_seven"),
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
            git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
            time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                String::from(file!()),
                line!(),
                column!(),
            ),
        }
    };
    return Err(Box::new(f));
}

#[derive(Debug)]
pub struct EightOriginError {
    source: String,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl EightOriginError {
    pub fn get_source_as_string(&self) -> String {
        format!("{}", self.source)
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        self.code_occurence.time_file_line_column.get_code_path(
            &self.code_occurence.git_info,
            config.get_source_place_type(),
        )
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct, //todo maybe remove
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        vec![
            crate::dev::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: vec![vec![(self.get_source_as_string(), vec![])]],
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        ]
    }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        log_type.console(
            &config.get_error_color_bold(),
            format!(
                "{}{}{}",
                self.get_source_as_string(),
                log_type.symbol(),
                self.get_code_occurence_as_string(config)
            ),
        )
    }
}

pub fn eight(should_trace: bool) -> Result<(), Box<EightOriginError>> {
    let f = EightOriginError {
        source: String::from("error_eight"),
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
            git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
            time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                String::from(file!()),
                line!(),
                column!(),
            ),
        }
    };
    return Err(Box::new(f));
}
