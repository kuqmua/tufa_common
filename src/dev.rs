use crate::common::source_and_code_occurence;
use crate::traits::code_path::CodePath;
use crate::traits::config_log::ConfigLog;
use crate::traits::console::Console;
use crate::traits::fields::GetLogType;
use crate::traits::fields::GetSourcePlaceType;
use crate::traits::get_color::ErrorColorBold;
use crate::traits::separator_symbol::SeparatorSymbol;
use itertools::Itertools;
use std::collections::HashMap;
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
        self.source.get_source_as_string(config)
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        self.code_occurence
            .get_code_path(config.get_source_place_type())
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct, //todo maybe remove
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let vec = self
            .source
            .get_inner_source_and_code_occurence_as_string(config);
        let mut new_vec = Vec::with_capacity(vec.len() + 1);
        let mut sources_for_tracing: Vec<
            Vec<(
                crate::common::source_and_code_occurence::Source,
                Vec<crate::common::source_and_code_occurence::Key>,
            )>,
        > = Vec::with_capacity(
            vec.iter()
                .map(|e| e.source.len())
                .collect::<Vec<usize>>()
                .iter()
                .sum(),
        );
        vec.into_iter().for_each(|mut v| {
            v.source.iter().for_each(|f| {
                sources_for_tracing.push(f.clone());
            });
            v.add_one();
            new_vec.push(v);
        });
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        new_vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: sources_for_tracing,
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        new_vec
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

pub fn three(should_trace: bool) -> Result<(), Box<ThreeWrapperError>> {
    if let Err(e) = four(false) {
        return Err(Box::new(ThreeWrapperError {
            source: ThreeWrapperErrorEnum::FourWrapper(*e),
            code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                    String::from(file!()),
                    line!(),
                    column!(),
                ),
            }
        }));
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
    //maybe not need?
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
        self.code_occurence
            .get_code_path(config.get_source_place_type())
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let mut sources_for_tracing: Vec<
            Vec<(
                crate::common::source_and_code_occurence::Source,
                Vec<crate::common::source_and_code_occurence::Key>,
            )>,
        > = vec![];
        let mut vec = self.source.iter().fold(
            Vec::with_capacity(self.source.len() + 1),
            |mut acc, (key, value)| {
                value
                    .get_inner_source_and_code_occurence_as_string(config)
                    .into_iter()
                    .for_each(|mut e| {
                        e.source.iter().for_each(|hm| {
                            let mut new_hm = Vec::with_capacity(hm.len());
                            hm.iter().for_each(|(k, v)| {
                                let mut new_v = Vec::with_capacity(v.len() + 1);
                                v.iter().for_each(|v_element| {
                                    new_v.push(v_element.clone());
                                });
                                new_v.push(crate::common::source_and_code_occurence::Key {
                                    key: key.clone(),
                                    uuid: uuid::Uuid::new_v4(),
                                });
                                new_hm.push((k.clone(), new_v.clone()));
                            });
                            sources_for_tracing.push(new_hm);
                        });
                        e.add_one();
                        acc.push(e);
                    });
                acc
            },
        );
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

pub fn four(should_trace: bool) -> Result<(), Box<FourWrapperError>> {
    match (five(false), six(false)) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            return Err(Box::new(FourWrapperError {
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
            }));
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
        self.code_occurence
            .get_code_path(config.get_source_place_type())
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let mut sources_for_tracing: Vec<
            Vec<(
                crate::common::source_and_code_occurence::Source,
                Vec<crate::common::source_and_code_occurence::Key>,
            )>,
        > = Vec::new();
        let mut vec = self.source.iter().fold(
            Vec::with_capacity(self.source.len() + 1),
            |mut acc, (key, value)| {
                value
                    .get_inner_source_and_code_occurence_as_string(config)
                    .into_iter()
                    .for_each(|mut e| {
                        e.source.iter().for_each(|hm| {
                            let mut hm_handle = Vec::with_capacity(hm.len());
                            hm.iter().for_each(|(k, v)| {
                                let mut keys_vec_handle = Vec::with_capacity(v.len() + 1);
                                v.iter().for_each(|v_element| {
                                    keys_vec_handle.push(v_element.clone());
                                });
                                keys_vec_handle.push(
                                    crate::common::source_and_code_occurence::Key {
                                        key: key.clone(),
                                        uuid: uuid::Uuid::new_v4(),
                                    },
                                );
                                hm_handle.push((k.clone(), keys_vec_handle));
                            });
                            sources_for_tracing.push(hm_handle);
                        });
                        e.add_one();
                        acc.push(e);
                    });
                acc
            },
        );
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
    if let Err(e) = five_one(false) {
        return Err(Box::new(FiveWrapperError {
            source: HashMap::from([
                (
                    String::from("five_one_hashmap key"),
                    FiveWrapperErrorEnum::FiveOneOrigin(*e),
                )
            ]),
            code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
               String::from(file!()),
                    line!(),
                    column!(),
                ),
            }
        }));
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
        self.code_occurence
            .get_code_path(config.get_source_place_type())
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct, //todo maybe remove
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        vec![
            crate::dev::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: vec![vec![(
                    crate::common::source_and_code_occurence::Source {
                        source: self.get_source_as_string(),
                        uuid: uuid::Uuid::new_v4(),
                    },
                    vec![],
                )]],
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
    return Err(Box::new(FiveOneOriginError {
        source: String::from("five_one error"),
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
            git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
            time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                String::from(file!()),
                line!(),
                column!(),
            ),
        }
    }));
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
        self.code_occurence
            .get_code_path(config.get_source_place_type())
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let mut sources_for_tracing: Vec<
            Vec<(
                crate::common::source_and_code_occurence::Source,
                Vec<crate::common::source_and_code_occurence::Key>,
            )>,
        > = Vec::new();
        let mut vec = self.source.iter().fold(
            Vec::with_capacity(self.source.len() + 1),
            |mut acc, vec_element| {
                vec_element
                    .get_inner_source_and_code_occurence_as_string(config)
                    .into_iter()
                    .for_each(|mut e| {
                        e.source.iter().for_each(|f| {
                            sources_for_tracing.push(f.clone());
                        });
                        e.add_one();
                        acc.push(e);
                    });
                acc
            },
        );
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
        config.log(format!(
            "{}{}{}",
            self.get_source_as_string(config),
            config.get_log_type().symbol(),
            self.get_code_occurence_as_string(config)
        ));
    }
}

pub fn six(should_trace: bool) -> Result<(), Box<SixWrapperError>> {
    match (seven(false), eight(false)) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            return Err(Box::new(SixWrapperError {
                source: vec![SixWrapperErrorEnum::SevenWrapper(*seven_error), SixWrapperErrorEnum::EightWrapper(*eight_error)],
                code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                    git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                        String::from(file!()),
                        line!(),
                        column!(),
                    ),
                }
            }));
        }
    }
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
        self.code_occurence
            .get_code_path(config.get_source_place_type())
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct, //todo maybe remove
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        vec![
            crate::dev::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: vec![vec![(
                    crate::common::source_and_code_occurence::Source {
                        source: self.get_source_as_string(),
                        uuid: uuid::Uuid::new_v4(),
                    },
                    vec![],
                )]],
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        ]
    }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        config.log(format!(
            "{}{}{}",
            self.get_source_as_string(),
            config.get_log_type().symbol(),
            self.get_code_occurence_as_string(config)
        ));
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
//todo - impl Display
impl EightOriginError {
    pub fn get_source_as_string(&self) -> String {
        format!("{}", self.source)
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        self.code_occurence
            .get_code_path(config.get_source_place_type())
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct, //todo maybe remove
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        vec![
            crate::dev::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: vec![vec![(
                    crate::common::source_and_code_occurence::Source {
                        source: self.get_source_as_string(),
                        uuid: uuid::Uuid::new_v4(),
                    },
                    vec![],
                )]],
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        ]
    }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        config.log(format!(
            "{}{}{}",
            self.get_source_as_string(),
            config.get_log_type().symbol(),
            self.get_code_occurence_as_string(config)
        ));
    }
}

pub fn eight(should_trace: bool) -> Result<(), Box<EightOriginError>> {
    return Err(Box::new(EightOriginError {
        source: String::from("error_eight"),
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
            git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
            time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                String::from(file!()),
                line!(),
                column!(),
            ),
        }
    }));
}
