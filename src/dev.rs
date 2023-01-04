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
use crate::traits::code_path::CodePath;
use crate::traits::console::Console;
use crate::traits::get_code_occurence::GetCodeOccurence;
use crate::traits::get_git_source_file_link::GetGitSourceFileLink;
use crate::traits::get_source_and_code_occurence;
use crate::traits::init_error::InitError;
// use crate::traits::new_error_with_git_info_file_line_column::NewErrorWithGitInfoFileLineColumn;
use crate::traits::new_error_with_one_addition::NewErrorWithOneAddition;
use crate::traits::readable_time::ReadableTime;
use crate::traits::separator_symbol::SeparatorSymbol;
use ansi_term::Colour::RGB;
use chrono::prelude::DateTime;
use chrono::Utc;
use impl_get_source::ImplGetSourceFromCrate;
use std::collections::HashMap;
use std::fmt::format;
use std::fmt::{self, Display};
use std::vec;
// use crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceAndCodeOccurence;
// use crate::traits::new_error_with_git_info_file_line_column::SomethingTest;
use crate::traits::get_source_and_code_occurence::GetSourceAndCodeOccurence;
use itertools::Itertools;

// #[derive(ImplGetSourceFromCrate)]
#[derive(Debug)]
pub struct ThreeWrapperError {
    source: ThreeWrapperErrorEnum,
    // code_occurence: crate::common::code_occurence::CodeOccurence,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl ThreeWrapperError {
    pub fn get_source_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String {
        //todo if origin - without config, if wrapper - with config
        // format!(
        //     "{}",
        //     self.source.get_source_and_code_occurence_as_string(config)
        // )
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
        let mut sources_for_tracing = vec![];
        let mut keys_for_tracing = vec![];
        let mut vec = self
            .source
            .get_inner_source_and_code_occurence_as_string(config);
        vec.iter_mut().for_each(|n| {
            n.increment += 1;
                match &n.source {
                    source_and_code_occurence::SourceEnum::SourceWithKeys(source_with_keys) => {
                        source_with_keys.keys.iter().for_each(|k| {
                            keys_for_tracing.push(k.clone());
                        });
                    }
                    source_and_code_occurence::SourceEnum::Source(s) => {
                        sources_for_tracing.push(s.clone());
                    }
                    source_and_code_occurence::SourceEnum::SourcesForTracing(sources) => {
                        sources.iter().for_each(|s| {
                            sources_for_tracing.push(s.clone());
                        });
                    }
                    source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(
                        sources_and_keys_for_tracing,
                    ) => {
                        sources_and_keys_for_tracing.sources.iter().for_each(|s| {
                            sources_for_tracing.push(s.clone());
                        });
                        sources_and_keys_for_tracing.keys.iter().for_each(|k| {
                            keys_for_tracing.push(k.clone());
                        });
                    }
                }
        });
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        keys_for_tracing = keys_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        let source_handle = match keys_for_tracing.is_empty() {
            true => source_and_code_occurence::SourceEnum::SourcesForTracing(
                sources_for_tracing,
            ),
            false => source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(
                    source_and_code_occurence::SourcesAndKeysForTracing {
                        sources: sources_for_tracing,
                        keys: keys_for_tracing,
                    },
                ),
        };
        vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: source_handle,
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        vec
    }
    // pub fn get_source_and_code_occurence_as_string(
    //     &self,
    //     config: &crate::config_mods::config_struct::ConfigStruct,
    // ) -> String {
    //     format!(
    //         "{}{}{}",
    //         self.get_source_as_string(config),
    //         config.get_log_type().symbol(),
    //         self.get_code_occurence_as_string(config)
    //     )
    // }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        log_type.console(
            &config.get_error_color_bold(),
            // self.get_source_and_code_occurence_as_string(config),
            format!(
                "{}{}{}",
                self.get_source_as_string(config),
                config.get_log_type().symbol(),
                self.get_code_occurence_as_string(config)
            ),
        )
    }
}

// хранение данные в code_occurence для сериализации и вывод данных в консоль - 2 разные вещи
// impl sssssss for SixWrapperError {
//     fn prepare_log_source_inner_and_code_occurence(
//         &self,
//         config_generic: &ConfigGeneric,
//     ) -> String {
// format!("{}{}", self.source.get_sourceand_code_occurence_as_string(), code_occurence из инпута параметра, не self. для self.code_occurence возможно будет тзасунуто больше чем нужно для вывода в консоль)
//     }
// }

// impl crate::traits::get_source_value::GetSourceValue<ThreeWrapperErrorEnum> for ThreeWrapperError {
//     fn get_source_value(&self) -> &ThreeWrapperErrorEnum {
//         &self.source //.get_source_value() - get source value in string with code occurence and erroor
//     }
// }

// impl crate::traits::get_source::GetSource for ThreeWrapperError {
//     fn get_source_and_code_occurence_as_string(&self) -> String {
//         self.source.get_source()
//     }
// }

// impl crate::traits::init_error::InitError<ThreeWrapperErrorEnum> for ThreeWrapperError {
//     fn init_error(
//         source: ThreeWrapperErrorEnum,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     ) -> Self {
//         Self {
//             source,
//             code_occurence,
//         }
//     }
// }

// impl crate::traits::get_code_occurence::GetCodeOccurence for ThreeWrapperError {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
//         &self.code_occurence
//     }
// }
use crate::traits::my_custom_display::DisplayError;
pub fn three(should_trace: bool) -> Result<(), Box<ThreeWrapperError>> {
    if let Err(e) = four(false) {
        // println!("{}", <FourWrapperError as DisplayError<SourceGeneric, ConfigStruct, ConfigStruct>>::display_error(&*e, once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG)));

        // return Err(Box::new(ThreeWrapperError::new_error_with_one_addition(
        //     ThreeWrapperErrorEnum::FourWrapper(*e),
        //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG),
        //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES),
        //     String::from(file!()),
        //     line!(),
        //     column!(),
        //     should_trace
        // )));
        let f = ThreeWrapperError {
            source: ThreeWrapperErrorEnum::FourWrapper(*e),
            // code_occurence: crate::common::code_occurence::CodeOccurence {
            //     occurences: HashMap::new(),
            // },
            code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                    String::from(file!()),
                    line!(),
                    column!(),
                ),
            }
        };
        // println!("three f {}", std::mem::size_of_val(&f));
        // println!("three source {}", std::mem::size_of_val(&f.source));
        // println!("three source {}", std::mem::size_of_val(&f.code_occurence));
        // println!("three-----");
        // f.log(once_cell::sync::Lazy::force(
        //     &crate::global_variables::runtime::config::CONFIG,
        // ));
        // println!("three-----");
        return Err(Box::new(f));
    };
    Ok(())
}
//
// #[derive(ImplGetSourceFromCrate)]
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
    // //does it need to be implemented here?
    // pub fn get_source_and_code_occurence_as_string(
    //     &self,
    //     config: &crate::config_mods::config_struct::ConfigStruct,
    // ) -> String {
    //     match self {
    //         ThreeWrapperErrorEnum::FourWrapper(i) => {
    //             i.get_source_and_code_occurence_as_string(config)
    //         }
    //     }
    // }
}

// impl crate::traits::get_code_occurence::GetCodeOccurence for ThreeWrapperErrorEnum {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
//         match self {
//             ThreeWrapperErrorEnum::FourWrapper(e) => e.get_code_occurence(),
//         }
//     }
// }
//
// #[derive(ImplGetSourceFromCrate)]
#[derive(Debug)]
pub struct FourWrapperError {
    source: HashMap<String, FourWrapperErrorEnum>,
    // code_occurence: crate::common::code_occurence::CodeOccurence,
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
                    //todo maybe space symbol
                    acc.push_str(&format!(
                        "[key: {}]{} {}{} {}{}",
                        key, symbol, source_as_string, symbol, get_code_occurence_as_string, symbol
                    ));
                    acc
                });
        log_type.pop_last(&mut source_as_string);
        // source_as_string = format!("@{}@", source_as_string);
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
        let mut sources_for_tracing = vec![];
        let mut keys_for_tracing = vec![];
        let mut vec = self
            .source
            .iter()
            .fold(Vec::with_capacity(len), |mut acc, (key, value)| {
                // println!("iter {}{:#?}", key, value);
                // let mut temp_keys_vec = vec![];
                //todo - must find highest increment value and put key there, for others  None - is it correct?     or maybe for one where source !== None ? if its more than one - spaces logic
                value
                    .get_inner_source_and_code_occurence_as_string(config)
                    .into_iter()
                    .for_each(|e| {
                        match e.source {
                            source_and_code_occurence::SourceEnum::SourceWithKeys(source_with_keys) => {
                                let mut key_vec = source_with_keys.keys;
                                key_vec.iter().for_each(|k|{
                                    // temp_keys_vec.push(k.clone())
                                    keys_for_tracing.push(k.clone());
                                });
                                key_vec.push(key.clone());
                                sources_for_tracing.push(source_with_keys.source.clone());
                                acc.push(
                                    crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                                        source: crate::common::source_and_code_occurence::SourceEnum::SourceWithKeys(
                                            crate::common::source_and_code_occurence::SourceWithKeys {
                                                keys: key_vec,
                                                source: source_with_keys.source.clone(),
                                            }
                                        ),
                                        code_occurence: e.code_occurence.clone(),
                                        increment: e.increment + 1,//change it to add_one()?
                                    }
                                );
                            },
                            source_and_code_occurence::SourceEnum::Source(source) => {
                                sources_for_tracing.push(source.clone());
                                acc.push(
                                    crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                                        source: crate::common::source_and_code_occurence::SourceEnum::SourceWithKeys(
                                            crate::common::source_and_code_occurence::SourceWithKeys {
                                                keys: vec![key.clone()],
                                                source: source.clone(),
                                            }
                                        ),
                                        code_occurence: e.code_occurence.clone(),
                                        increment: e.increment + 1,//change it to add_one()?
                                    }
                                );
                            },
                            source_and_code_occurence::SourceEnum::SourcesForTracing(sources) => {
                                sources.iter().for_each(|s|{
                                    sources_for_tracing.push(s.clone())
                                });
                                acc.push(
                                    crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                                        source: crate::common::source_and_code_occurence::SourceEnum::SourcesForTracing(
                                            sources.clone()
                                        ),
                                        code_occurence: e.code_occurence.clone(),
                                        increment: e.increment + 1,//change it to add_one()?
                                    }
                                );   
                            }
                            source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing) => {
                                sources_and_keys_for_tracing.sources.iter().for_each(|s|{
                                    sources_for_tracing.push(s.clone())
                                });
                                let mut key_vec = sources_and_keys_for_tracing.keys.clone();
                                key_vec.iter().for_each(|k|{
                                    keys_for_tracing.push(k.clone())
                                });
                                key_vec.push(key.clone());
                                acc.push(
                                    crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                                        source: crate::common::source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(
                                            crate::dev::source_and_code_occurence::SourcesAndKeysForTracing {
                                                sources: sources_and_keys_for_tracing.sources.clone(),
                                                keys: key_vec.clone(),
                                            }
                                        ),
                                        code_occurence: e.code_occurence.clone(),
                                        increment: e.increment + 1,//change it to add_one()?
                                    }
                                );
                            }
                        }
                    });
                // temp_keys_vec.push(key.clone());
                keys_for_tracing.push(key.clone());
                acc
            });
        // println!("keys_for_tracing{:#?}keys_for_tracing", keys_for_tracing);
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        keys_for_tracing = keys_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        let source_handle = match keys_for_tracing.is_empty() {
            true => source_and_code_occurence::SourceEnum::SourcesForTracing(
                sources_for_tracing,
            ),
            false => source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(
                    source_and_code_occurence::SourcesAndKeysForTracing {
                        sources: sources_for_tracing,
                        keys: keys_for_tracing,
                    },
                ),
        };
        vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: source_handle,
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
//
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
    // if let Err(e) = six(false) {
    //     return Err(Box::new(FourWrapperError::new_error_with_one_addition(
    //         FourWrapperErrorEnum::SixWrapper(*e),
    //         once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG),
    //         once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES),
    //         String::from(file!()),
    //         line!(),
    //         column!(),
    //         should_trace
    //     )));
    // }
    match (five(false), six(false)) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            //     return Err(Box::new(FourWrapperError::new_error_with_one_addition(
            //  HashMap::from([
            //             (
            //                 String::from("five_hashmap_key"),
            //                 FourWrapperErrorEnum::FiveWrapper(*f)
            //             ),
            //             (
            //                 String::from("six_hashmap_key"),
            //                 FourWrapperErrorEnum::SixWrapper(*s)
            //             )
            //         ]),
            //         once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG),
            //         once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES),
            //         String::from(file!()),
            //         line!(),
            //         column!(),
            //         should_trace
            //     )));
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
                // code_occurence: crate::common::code_occurence::CodeOccurence {
                //     occurences: HashMap::new(),
                // },
                code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                    git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                        String::from(file!()),
                        line!(),
                        column!(),
                    ),
                }
            };
            // println!("four f {}", std::mem::size_of_val(&f));
            // println!("four source {}", std::mem::size_of_val(&f.source));
            // println!("four source {}", std::mem::size_of_val(&f.code_occurence));
            // println!("four-----");
            // f.log(once_cell::sync::Lazy::force(
            //     &crate::global_variables::runtime::config::CONFIG,
            // ));
            // println!("four-----");
            return Err(Box::new(f));
        }
    }
    Ok(())
}
//
//
//
#[derive(Debug)]
pub struct FiveWrapperError {
    source: HashMap<String, FiveWrapperErrorEnum>,
    // code_occurence: crate::common::code_occurence::CodeOccurence,
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
        // source_as_string = format!("@{}@", source_as_string);
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
        let mut sources_for_tracing = vec![];
        let mut keys_for_tracing = vec![];
        let mut vec = self
            .source
            .iter()
            .fold(Vec::with_capacity(len), |mut acc, (key, value)| {
                // println!("iter {}{:#?}", key, value);
                // let mut temp_keys_vec = vec![];
                //todo - must find highest increment value and put key there, for others  None - is it correct?     or maybe for one where source !== None ? if its more than one - spaces logic
                value
                    .get_inner_source_and_code_occurence_as_string(config)
                    .into_iter()
                    .for_each(|e| {
                        match e.source {
                            source_and_code_occurence::SourceEnum::SourceWithKeys(source_with_keys) => {
                                let mut key_vec = source_with_keys.keys;
                                key_vec.iter().for_each(|k|{
                                    // temp_keys_vec.push(k.clone())
                                    keys_for_tracing.push(k.clone());
                                });
                                key_vec.push(key.clone());
                                sources_for_tracing.push(source_with_keys.source.clone());
                                acc.push(
                                    crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                                        source: crate::common::source_and_code_occurence::SourceEnum::SourceWithKeys(
                                            crate::common::source_and_code_occurence::SourceWithKeys {
                                                keys: key_vec,
                                                source: source_with_keys.source.clone(),
                                            }
                                        ),
                                        code_occurence: e.code_occurence.clone(),
                                        increment: e.increment + 1,//change it to add_one()?
                                    }
                                );
                            },
                            source_and_code_occurence::SourceEnum::Source(source) => {
                                sources_for_tracing.push(source.clone());
                                acc.push(
                                    crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                                        source: crate::common::source_and_code_occurence::SourceEnum::SourceWithKeys(
                                            crate::common::source_and_code_occurence::SourceWithKeys {
                                                keys: vec![key.clone()],
                                                source: source.clone(),
                                            }
                                        ),
                                        code_occurence: e.code_occurence.clone(),
                                        increment: e.increment + 1,//change it to add_one()?
                                    }
                                );
                            },
                            source_and_code_occurence::SourceEnum::SourcesForTracing(sources) => {
                                sources.iter().for_each(|s|{
                                    sources_for_tracing.push(s.clone())
                                });
                                acc.push(
                                    crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                                        source: crate::common::source_and_code_occurence::SourceEnum::SourcesForTracing(
                                            sources.clone()
                                        ),
                                        code_occurence: e.code_occurence.clone(),
                                        increment: e.increment + 1,//change it to add_one()?
                                    }
                                );   
                            }
                            source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing) => {
                                sources_and_keys_for_tracing.sources.iter().for_each(|s|{
                                    sources_for_tracing.push(s.clone())
                                });
                                let mut key_vec = sources_and_keys_for_tracing.keys.clone();
                                key_vec.iter().for_each(|k|{
                                    keys_for_tracing.push(k.clone())
                                });
                                key_vec.push(key.clone());
                                acc.push(
                                    crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                                        source: crate::common::source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(
                                            crate::dev::source_and_code_occurence::SourcesAndKeysForTracing {
                                                sources: sources_and_keys_for_tracing.sources.clone(),
                                                keys: key_vec.clone(),
                                            }
                                        ),
                                        code_occurence: e.code_occurence.clone(),
                                        increment: e.increment + 1,//change it to add_one()?
                                    }
                                );
                            }
                        }
                    });
                // temp_keys_vec.push(key.clone());
                keys_for_tracing.push(key.clone());
                acc
            });
        // println!("keys_for_tracing{:#?}keys_for_tracing", keys_for_tracing);
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        keys_for_tracing = keys_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        let source_handle = match keys_for_tracing.is_empty() {
            true => source_and_code_occurence::SourceEnum::SourcesForTracing(
                sources_for_tracing,
            ),
            false => source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(
                    source_and_code_occurence::SourcesAndKeysForTracing {
                        sources: sources_for_tracing,
                        keys: keys_for_tracing,
                    },
                ),
        };
        vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: source_handle,
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
//
#[derive(Debug)]
pub enum FiveWrapperErrorEnum {
    FiveOneOrigin(FiveOneOriginError)
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
//

pub fn five(should_trace: bool) -> Result<(), Box<FiveWrapperError>> {
    let five_one_result = five_one(false);
    if let Err(e) = five_one_result {
        let mut hm = HashMap::new();
        hm.insert(String::from("five_one_hashmap key"), FiveWrapperErrorEnum::FiveOneOrigin(*e));
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
    // println!("five f {}", std::mem::size_of_val(&f));
    // println!("five source {}", std::mem::size_of_val(&f.source));
    // println!("five source {}", std::mem::size_of_val(&f.code_occurence));
    // println!("five-----");
    // f.log(once_cell::sync::Lazy::force(
    //     &crate::global_variables::runtime::config::CONFIG,
    // ));
    // println!("five-----");
    return Err(Box::new(f));
    }
    Ok(())
}

//
// #[derive(ImplGetSourceFromCrate)]
#[derive(Debug)]
pub struct FiveOneOriginError {
    source: String,
    // code_occurence: crate::common::code_occurence::CodeOccurence,
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
                source: 
                    crate::common::source_and_code_occurence::SourceEnum::Source(
                        self.get_source_as_string(),
                    ),
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        ]
    }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        log_type.console(
            &config.get_error_color_bold(),
            // self.get_source_and_code_occurence_as_string(config),
            format!(
                "{}{}{}",
                self.get_source_as_string(),
                config.get_log_type().symbol(),
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
    // println!("five f {}", std::mem::size_of_val(&f));
    // println!("five source {}", std::mem::size_of_val(&f.source));
    // println!("five source {}", std::mem::size_of_val(&f.code_occurence));
    // println!("five-----");
    // f.log(once_cell::sync::Lazy::force(
    //     &crate::global_variables::runtime::config::CONFIG,
    // ));
    // println!("five-----");
    return Err(Box::new(f));
}

use crate::traits::code_occurence_methods::CodeOccurenceToString;
use crate::traits::fields::GetLogType;
use crate::traits::fields::GetSourcePlaceType;
use crate::traits::get_color::ErrorColorBold;
use crate::traits::get_source_value::GetSourceValue;
use crate::traits::readable_time_string::ReadableTimeString;
// use crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceInnerAndCodeOccurence;

// #[derive(ImplGetSourceFromCrate)]
#[derive(Debug)]
pub struct SixWrapperError {
    source: Vec<SixWrapperErrorEnum>,
    // code_occurence: crate::common::code_occurence::CodeOccurence,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}
// 00[key: six_hashmap_key] ==error_seven
// https://github.com/kuqmua/tufa_common/blob/031d2a364a21941c2742c95076f0d4fd5f2f5acf/src/dev.rs#L809==
// ==error_eight
// https://github.com/kuqmua/tufa_common/blob/031d2a364a21941c2742c95076f0d4fd5f2f5acf/src/dev.rs#L870==11
//   https://github.com/kuqmua/tufa_common/blob/031d2a364a21941c2742c95076f0d4fd5f2f5acf/src/dev.rs#L69722

// [key: six_hashmap_key]
//  error_seven
//   https://github.com/kuqmua/tufa_common/blob/031d2a364a21941c2742c95076f0d4fd5f2f5acf/src/dev.rs#L809==
//  error_eight
//   https://github.com/kuqmua/tufa_common/blob/031d2a364a21941c2742c95076f0d4fd5f2f5acf/src/dev.rs#L870==11
// https://github.com/kuqmua/tufa_common/blob/031d2a364a21941c2742c95076f0d4fd5f2f5acf/src/dev.rs#L697
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
                    // let source_and_code_occurence_as_string =
                    // vec_element.get_source_and_code_occurence_as_string(config);

                    // self.get_code_occurence_as_string(config)
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
        // source_as_string = format!("!{}!", source_as_string);
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
        let mut sources_for_tracing = vec![];
        let mut keys_for_tracing = vec![];
        let mut vec = self
            .source
            .iter()
            .fold(Vec::with_capacity(len), |mut acc, vec_element| {
                vec_element
                    .get_inner_source_and_code_occurence_as_string(config)
                    .into_iter()
                    .for_each(|e| {
                        match &e.source {
                            source_and_code_occurence::SourceEnum::SourceWithKeys(
                                source_with_keys,
                            ) => {
                                source_with_keys.keys.iter().for_each(|k| {
                                    keys_for_tracing.push(k.clone());
                                });
                            }
                            source_and_code_occurence::SourceEnum::Source(string_handle) => {
                                sources_for_tracing.push(string_handle.clone());
                            }
                            source_and_code_occurence::SourceEnum::SourcesForTracing(
                                sources,
                            ) => sources.iter().for_each(|s| {
                                sources_for_tracing.push(s.clone());
                            }),
                            source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(
                                sources_and_keys_for_tracing,
                            ) => {
                                sources_and_keys_for_tracing.keys.iter().for_each(|k| {
                                    keys_for_tracing.push(k.clone());
                                });
                                sources_and_keys_for_tracing.keys.iter().for_each(|s| {
                                    sources_for_tracing.push(s.clone());
                                });
                            }
                        }
                        acc.push(e.add_one());
                    });
                acc
            });
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        keys_for_tracing = keys_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        let source_handle = match keys_for_tracing.is_empty() {
            true => source_and_code_occurence::SourceEnum::SourcesForTracing(
                sources_for_tracing,
            ),
            false => source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(
                    source_and_code_occurence::SourcesAndKeysForTracing {
                        sources: sources_for_tracing,
                        keys: keys_for_tracing,
                    },
                ),
        };
        vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: source_handle,
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        vec
    }
    // pub fn get_source_and_code_occurence_as_string(
    //     &self,
    //     config: &crate::config_mods::config_struct::ConfigStruct,
    // ) -> String {
    //     format!(
    //         "{}{}{}",
    //         self.get_source_as_string(config),
    //         config.get_log_type().symbol(),
    //         self.get_code_occurence_as_string(config)
    //     )
    // }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        log_type.console(
            &config.get_error_color_bold(),
            // self.get_source_and_code_occurence_as_string(config),
            format!(
                "{}{}{}",
                self.get_source_as_string(config),
                config.get_log_type().symbol(),
                self.get_code_occurence_as_string(config)
            ),
        )
    }
}
//

// //

//     let mut vec: Vec<crate::common::code_occurence::OccurenceFilter> =
//         Vec::with_capacity(capacity);
//     self.occurences.iter().for_each(|(git_info, v)| {
//         v.iter().for_each(|e| {
//             vec.push(crate::common::code_occurence::OccurenceFilter {
//                 increment: e.increment,
//                 time: e.time_file_line_column.time,
//                 occurence: e
//                     .time_file_line_column
//                     .get_code_path(git_info, config_generic.get_source_place_type()),
//             })
//         })
//     });
//     //vec.reverse();//todo check reserve or not
//     vec.sort_by(|a, b| a.increment.cmp(&b.increment));
//     let mut occurences = Vec::with_capacity(capacity + 1);
//     let log_type = config_generic.get_log_type();
//     occurences.push(format!(
//         "{}{}",
//         source_generic.get_source(),
//         log_type.symbol()
//     ));
//     vec.into_iter().for_each(|e| {
//         occurences.push(format!(
//             "{} {}{}",
//             e.readable_time_string(),
//             e.occurence,
//             log_type.symbol()
//         ));
//     });
//     let mut occurence = occurences.iter().fold(String::from(""), |mut acc, elem| {
//         acc.push_str(elem);
//         acc
//     });
//     log_type.pop_last(&mut occurence);
//     occurence
// //
//

// impl crate::traits::get_source_value::GetSourceValue<String> for SixWrapperError {
//     fn get_source_value(&self) -> &String {
//         &self.source
//     }
// }

// impl crate::traits::init_error::InitError<String> for SixWrapperError {
//     fn init_error(
//         source: String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     ) -> Self {
//         Self {
//             source,
//             code_occurence,
//         }
//     }
// }

// impl crate::traits::get_code_occurence::GetCodeOccurence for SixWrapperError {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
//         &self.code_occurence
//     }
// }

pub fn six(should_trace: bool) -> Result<(), Box<SixWrapperError>> {
    // let arc_usage = crate::common::code_occurence::CodeOccurenceWithArcUsage::new(
    //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES_UNDER_ARC).clone(),
    //     String::from(file!()),
    //     line!(),
    //     column!(),
    // );
    // let typical = crate::common::code_occurence::CodeOccurence::new(
    //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
    //     String::from(file!()),
    //     line!(),
    //     column!(),
    // );
    // // println!("arc usage {}", std::mem::size_of_val(&arc_usage.occurences));
    // println!("typical {}", std::mem::size_of_val(&typical.occurences));
    // return Err(Box::new(SixWrapperError::something_test(
    //     String::from("error_six"),
    //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG),
    //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES),
    //     String::from(file!()),
    //     line!(),
    //     column!(),
    //     false,
    // ) ));
    //
    match (seven(false), eight(false)) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let f = SixWrapperError {
                source: vec![SixWrapperErrorEnum::SevenWrapper(*seven_error), SixWrapperErrorEnum::EightWrapper(*eight_error)],
                // code_occurence: typical,
                code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                    git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                        String::from(file!()),
                        line!(),
                        column!(),
                    ),
                }
            };
            // println!("six f {}", std::mem::size_of_val(&f));
            // println!("six source {}", std::mem::size_of_val(&f.source));
            // println!("six source {}", std::mem::size_of_val(&f.code_occurence));
            // println!("six-----");
            // f.log(once_cell::sync::Lazy::force(
            //     &crate::global_variables::runtime::config::CONFIG,
            // ));
            // println!("six-----");
            return Err(Box::new(f));
        }
    }
    Ok(())
}

// impl crate::traits::get_source::GetSource for HashMap<std::string::String, FourWrapperErrorEnum> {
//     fn get_source(&self) -> String {
//         String::from("todo this impl")
//     }
// }

// #[derive(ImplGetSourceFromCrate)]
#[derive(Debug)]
pub enum SixWrapperErrorEnum {
    SevenWrapper(SevenOriginError),
    EightWrapper(EightOriginError),
}
//
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
    //does it need to be implemented here?
    // pub fn get_source_and_code_occurence_as_string(
    //     &self,
    //     config: &crate::config_mods::config_struct::ConfigStruct,
    // ) -> String {
    //     match self {
    //         SixWrapperErrorEnum::SevenWrapper(i) => {
    //             i.get_source_and_code_occurence_as_string(config)
    //         }
    //         SixWrapperErrorEnum::EightWrapper(i) => {
    //             i.get_source_and_code_occurence_as_string(config)
    //         }
    //     }
    // }
}
////////////////

// #[derive(ImplGetSourceFromCrate)]
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
                source: 
                    crate::common::source_and_code_occurence::SourceEnum::Source(
                        self.get_source_as_string(),
                    ),
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        ]
    }
    // pub fn get_source_and_code_occurence_as_string(
    //     &self,
    //     config: &crate::config_mods::config_struct::ConfigStruct,
    // ) -> String {
    //     format!(
    //         "{}{} {}",
    //         self.get_source_as_string(),
    //         config.get_log_type().symbol(),
    //         self.get_code_occurence_as_string(config)
    //     )
    // }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        log_type.console(
            &config.get_error_color_bold(),
            // self.get_source_and_code_occurence_as_string(config),
            format!(
                "{}{} {}",
                self.get_source_as_string(),
                config.get_log_type().symbol(),
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
    // println!("seven f {}", std::mem::size_of_val(&f));
    // println!("seven source {}", std::mem::size_of_val(&f.source));
    // println!("seven source {}", std::mem::size_of_val(&f.code_occurence));
    // println!("seven-----");
    // f.log(once_cell::sync::Lazy::force(
    //     &crate::global_variables::runtime::config::CONFIG,
    // ));
    // println!("seven-----");
    return Err(Box::new(f));
}

//
// #[derive(ImplGetSourceFromCrate)]
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
                source: 
                    crate::common::source_and_code_occurence::SourceEnum::Source(
                        self.get_source_as_string(),
                    ),
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        ]
    }
    // pub fn get_source_and_code_occurence_as_string(
    //     &self,
    //     config: &crate::config_mods::config_struct::ConfigStruct,
    // ) -> String {
    //     format!(
    //         "{}{}({}",
    //         self.get_source_as_string(),
    //         config.get_log_type().symbol(),
    //         self.get_code_occurence_as_string(config)
    //     )
    // }
    pub fn log(&self, config: &crate::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        log_type.console(
            &config.get_error_color_bold(),
            // self.get_source_and_code_occurence_as_string(config),
            format!(
                "{}{}({}",
                self.get_source_as_string(),
                config.get_log_type().symbol(),
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
    // println!("eight f {}", std::mem::size_of_val(&f));
    // println!("eight source {}", std::mem::size_of_val(&f.source));
    // println!("eight source {}", std::mem::size_of_val(&f.code_occurence));
    // println!("eight-----");
    // f.log(once_cell::sync::Lazy::force(
    //     &crate::global_variables::runtime::config::CONFIG,
    // ));
    // println!("eight-----");
    return Err(Box::new(f));
}
