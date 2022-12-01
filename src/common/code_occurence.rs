use crate::common::git::git_info::GitInformation;
use crate::common::where_was::GitInfoForWhereWas;
use crate::common::where_was::WhereWas;
use crate::config_mods::log_type::LogType;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::code_occurence::CodeOccurenceTrait;
use crate::traits::file_line_column::FileLineColumnTrait;
use crate::traits::readable_time::ReadableTimeTrait;
use crate::traits::separator_symbol_trait::SeparatorSymbolTrait;
use ansi_term::Colour::RGB;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::time::UNIX_EPOCH;

// #[derive(Debug, Clone)]
// pub struct CodeOccurence {
//     where_was_hashmap: HashMap<GitInfoForWhereWas, Vec<TimeFileLineColumnIncrement>>,
// }

impl CodeOccurenceTrait for HashMap<GitInfoForWhereWas, Vec<TimeFileLineColumnIncrement>> {
    //     time: std::time::Duration,
    //     file: String, //&'a str
    //     line: u32,
    //     column: u32,
    // fn new_with_increment(key: GitInfoForWhereWas, value_element: TimeFileLineColumn) -> Self {
    //     HashMap::from([(
    //         key,
    //         vec![TimeFileLineColumnIncrement {
    //             increment: 0,
    //             value: value_element,
    //         }],
    //     )])
    // }
    fn insert_with_key_check(
        &mut self,
        key: GitInfoForWhereWas,
        value_element: TimeFileLineColumn,
    ) {
        let last_increment = {
            let mut increment_handle = 0;
            self.values().for_each(|v| {
                v.iter().for_each(|e| {
                    if e.increment > increment_handle {
                        increment_handle = e.increment;
                    }
                });
            });
            increment_handle
        };
        self.entry(key)
            .and_modify(|vec_existing_value_elements| {
                vec_existing_value_elements.push(TimeFileLineColumnIncrement {
                    increment: last_increment,
                    value: value_element.clone(), //todo how to rewrite it without clone() ?
                });
            })
            .or_insert_with(|| {
                vec![TimeFileLineColumnIncrement {
                    increment: last_increment,
                    value: value_element,
                }]
            });
    }
    fn add(&mut self, hashmap: HashMap<GitInfoForWhereWas, Vec<TimeFileLineColumnIncrement>>) {
        let mut last_increment = {
            let mut increment_handle = 0;
            hashmap.values().for_each(|v| {
                v.iter().for_each(|e| {
                    if e.increment > increment_handle {
                        increment_handle = e.increment;
                    }
                });
            });
            increment_handle
        };
        hashmap.iter().for_each(|(key, value)| {
            value.iter().for_each(|value_element| {
                last_increment += 1;
                self.entry(key.clone())
                    .and_modify(|vec_existing_value_elements| {
                        vec_existing_value_elements.push(TimeFileLineColumnIncrement {
                            increment: last_increment,
                            value: value_element.value.clone(), //todo how to rewrite it without clone() ?
                        });
                    })
                    .or_insert_with(|| {
                        vec![TimeFileLineColumnIncrement {
                            increment: last_increment,
                            value: value_element.value.clone(),
                        }]
                    });
            });
        });
    }
    fn log(
        &self,
        source_place_type: &SourcePlaceType,
        log_type: LogType,
        source: String,
        error_red: u8,
        error_green: u8,
        error_blue: u8,
    ) {
        match source_place_type {
            SourcePlaceType::Source => {
                let len = self.values().fold(0, |mut acc, elem| {
                    acc += elem.len();
                    acc
                });
                let mut vec: Vec<OccurenceFilter> = Vec::with_capacity(len);
                self.values().for_each(|v| {
                    v.iter().for_each(|e| {
                        vec.push(OccurenceFilter {
                            increment: e.increment,
                            time: e.value.time,
                            occurence: e.value.file_line_column(),
                        })
                    })
                });
                //todo check reserve or not
                vec.sort_by(|a, b| a.increment.cmp(&b.increment));
                let mut occurences = Vec::with_capacity(len + 1);
                occurences.push(format!("{}{}", source, log_type.symbol()));
                vec.into_iter().for_each(|e| {
                    occurences.push(format!("{:?} {}{}", e.time, e.occurence, log_type.symbol()));
                });
                let mut occurence = occurences.iter().fold(String::from(""), |mut acc, elem| {
                    acc.push_str(elem);
                    acc
                });
                log_type.pop_last(&mut occurence);
                match log_type {
                    LogType::Tracing => {
                        tracing::error!(error = occurence);
                    }
                    LogType::Stack => {
                        eprintln!(
                            "{}",
                            RGB(error_red, error_green, error_blue)
                                .bold()
                                .paint(occurence)
                        );
                    }
                    LogType::None => (),
                }
            }
            SourcePlaceType::Github => {
                let len = self.values().fold(0, |mut acc, elem| {
                    acc += elem.len();
                    acc
                });
                let mut vec: Vec<OccurenceFilter> = Vec::with_capacity(len);
                self.iter().for_each(|(k, v)| {
                    v.iter().for_each(|e| {
                        vec.push(OccurenceFilter {
                            increment: e.increment,
                            time: e.value.time,
                            occurence: e.value.github_file_line_column(k),
                        })
                    })
                });
                //todo check reserve or not
                vec.sort_by(|a, b| a.increment.cmp(&b.increment));
                let mut occurences = Vec::with_capacity(len + 1);
                occurences.push(format!("{}{}", source, log_type.symbol()));
                vec.into_iter().for_each(|e| {
                    let d = UNIX_EPOCH + e.time;
                    let datetime = DateTime::<Utc>::from(d);
                    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
                    occurences.push(format!(
                        "{} {}{}",
                        timestamp_str,
                        e.occurence,
                        log_type.symbol()
                    ));
                });
                let mut occurence = occurences.iter().fold(String::from(""), |mut acc, elem| {
                    acc.push_str(elem);
                    acc
                });
                log_type.pop_last(&mut occurence);
                match log_type {
                    LogType::Tracing => {
                        tracing::error!(error = occurence);
                    }
                    LogType::Stack => {
                        eprintln!(
                            "{}",
                            RGB(error_red, error_green, error_blue)
                                .bold()
                                .paint(occurence)
                        );
                    }
                    LogType::None => (),
                }
            }
            SourcePlaceType::None => (),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OccurenceFilter {
    pub increment: u64, //potential overflow?
    pub time: std::time::Duration,
    pub occurence: String,
}

#[derive(Debug, Clone)]
pub struct TimeFileLineColumnIncrement {
    pub increment: u64, //potential overflow?
    pub value: TimeFileLineColumn,
}

#[derive(Debug, Clone)]
pub struct TimeFileLineColumn {
    pub time: std::time::Duration,
    pub file_line_column: FileLineColumn,
}

impl ReadableTimeTrait for TimeFileLineColumn {
    fn readable_time(&self, timezone: i32) -> String {
        chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.time)
            .with_timezone(&chrono::FixedOffset::east(timezone))
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}

impl FileLineColumnTrait for TimeFileLineColumn {
    fn file_line_column(&self) -> String {
        self.file_line_column.file_line_column()
    }
    //todo make it const fn
    fn github_file_line_column(
        &self,
        git_info: &crate::common::where_was::GitInfoForWhereWas,
    ) -> String {
        self.file_line_column.github_file_line_column(git_info)
    }
}

#[derive(Debug, Clone)]
pub struct FileLineColumn {
    pub file: String, //&'a str
    pub line: u32,
    pub column: u32,
}

impl FileLineColumnTrait for FileLineColumn {
    fn file_line_column(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.column)
    }
    //todo make it const fn
    fn github_file_line_column(
        &self,
        git_info: &crate::common::where_was::GitInfoForWhereWas,
    ) -> String {
        let file = self.file.clone();
        let backslash = "/";
        match file.find(backslash) {
            Some(index) => {
                git_info.get_git_source_file_link(&file[index + backslash.len()..], self.line)
            }
            None => String::from("cant find backslash symbol in file path of location"),
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct FileLineColumn<'a> {
//     pub file: &'a str
//     pub line: u32,
//     pub column: u32,
// }

// impl FileLineColumn {
//     const fn file_line_column<'a>(&self) -> &'a str {
//         //todo how to convert u32 to &'a str on compiletime
//         format!("{}:{}:{}", self.file, self.line, self.column)
//     }
// }
