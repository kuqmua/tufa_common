use crate::common::git::git_info::GitInformation;
use crate::common::git::git_info::GitInformationWithoutLifetimes;
use crate::common::where_was::WhereWas;
use crate::config_mods::log_type::LogType;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::code_occurence::CodeOccurence;
use crate::traits::code_path::CodePath;
use crate::traits::console::Console;
use crate::traits::get_git_source_file_link::GetGitSourceFileLink;
use crate::traits::readable_time::ReadableTimeTrait;
use crate::traits::readable_time_string::ReadableTimeStringTrait;
use crate::traits::separator_symbol_trait::SeparatorSymbolTrait;
use ansi_term::Colour::RGB;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::collections::HashMap;
use std::fmt::{self, Display};

use crate::global_variables::compile_time::git_info::GIT_INFO;
pub struct ThreeError {
    source: u32,
    pub code_occurence: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
}

pub fn three() -> Result<(), Box<ThreeError>> {
    return Err(Box::new(ThreeError {
        source: 34,
        code_occurence: HashMap::from([(
            crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
            vec![TimeFileLineColumnIncrement::new(
                String::from(file!()),
                line!(),
                column!(),
            )],
        )]),
    }));
}

impl CodeOccurence for HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>> {
    fn insert_with_key_check(
        &mut self,
        key: GitInformationWithoutLifetimes,
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
                    time_file_line_column: value_element.clone(), //todo how to rewrite it without clone() ?
                });
            })
            .or_insert_with(|| {
                vec![TimeFileLineColumnIncrement {
                    increment: last_increment,
                    time_file_line_column: value_element,
                }]
            });
    }
    fn add(
        &mut self,
        hashmap: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
    ) {
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
                            time_file_line_column: value_element.time_file_line_column.clone(), //todo how to rewrite it without clone() ?
                        });
                    })
                    .or_insert_with(|| {
                        vec![TimeFileLineColumnIncrement {
                            increment: last_increment,
                            time_file_line_column: value_element.time_file_line_column.clone(),
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
        style: ansi_term::Style,
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
                            time: e.time_file_line_column.time,
                            occurence: e.time_file_line_column.get_project_code_path(),
                        })
                    })
                });
                //todo check reserve or not
                vec.sort_by(|a, b| a.increment.cmp(&b.increment));
                let mut occurences = Vec::with_capacity(len + 1);
                occurences.push(format!("{}{}", source, log_type.symbol()));
                vec.into_iter().for_each(|e| {
                    occurences.push(format!(
                        "{:?} {}{}",
                        e.readable_time_string(),
                        e.occurence,
                        log_type.symbol()
                    ));
                });
                let mut occurence = occurences.iter().fold(String::from(""), |mut acc, elem| {
                    acc.push_str(elem);
                    acc
                });
                log_type.pop_last(&mut occurence);
                log_type.console(style, occurence);
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
                            time: e.time_file_line_column.time,
                            occurence: e.time_file_line_column.get_github_code_path(k),
                        })
                    })
                });
                //todo check reserve or not
                vec.sort_by(|a, b| a.increment.cmp(&b.increment));
                let mut occurences = Vec::with_capacity(len + 1);
                occurences.push(format!("{}{}", source, log_type.symbol()));
                vec.into_iter().for_each(|e| {
                    occurences.push(format!(
                        "{} {}{}",
                        e.readable_time_string(),
                        e.occurence,
                        log_type.symbol()
                    ));
                });
                let mut occurence = occurences.iter().fold(String::from(""), |mut acc, elem| {
                    acc.push_str(elem);
                    acc
                });
                log_type.pop_last(&mut occurence);
                log_type.console(style, occurence);
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

impl ReadableTimeStringTrait for OccurenceFilter {
    fn readable_time_string(&self) -> String {
        DateTime::<Utc>::from(std::time::UNIX_EPOCH + self.time)
            .format("%Y-%m-%d %H:%M:%S.%f")
            .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct TimeFileLineColumnIncrement {
    increment: u64, //potential overflow?
    time_file_line_column: TimeFileLineColumn,
}

impl TimeFileLineColumnIncrement {
    pub fn new(
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            increment: 0, //potential overflow?
            time_file_line_column: TimeFileLineColumn::new(FileLineColumn { file, line, column }),
        }
    }
}

impl ReadableTimeStringTrait for TimeFileLineColumnIncrement {
    fn readable_time_string(&self) -> String {
        self.time_file_line_column.readable_time_string()
    }
}

#[derive(Debug, Clone)]
pub struct TimeFileLineColumn {
    time: std::time::Duration,
    file_line_column: FileLineColumn,
}

impl TimeFileLineColumn {
    pub fn new(file_line_column: FileLineColumn) -> Self {
        Self {
            time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("cannot convert time to unix_epoch"),
            file_line_column,
        }
    }
}

impl ReadableTimeStringTrait for TimeFileLineColumn {
    fn readable_time_string(&self) -> String {
        DateTime::<Utc>::from(std::time::UNIX_EPOCH + self.time)
            .format("%Y-%m-%d %H:%M:%S.%f")
            .to_string()
    }
}

impl ReadableTimeTrait for TimeFileLineColumn {
    fn readable_time(&self, timezone: i32) -> String {
        chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.time)
            .with_timezone(&chrono::FixedOffset::east(timezone))
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}

impl CodePath for TimeFileLineColumn {
    fn get_project_code_path(&self) -> String {
        self.file_line_column.get_project_code_path()
    }
    //todo make it const fn
    fn get_github_code_path(&self, git_info: &GitInformationWithoutLifetimes) -> String {
        self.file_line_column.get_github_code_path(git_info)
    }
}

#[derive(Debug, Clone)]
pub struct FileLineColumn {
    pub file: String, //&'a str
    pub line: u32,
    pub column: u32,
}

impl CodePath for FileLineColumn {
    fn get_project_code_path(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.column)
    }
    //todo make it const fn
    fn get_github_code_path(&self, git_info: &GitInformationWithoutLifetimes) -> String {
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

// impl FileLineColumn {
//     const fn file_line_column<'a>(&self) -> &'a str {
//         //todo how to convert u32 to &'a str on compiletime
//         format!("{}:{}:{}", self.file, self.line, self.column)
//     }
// }
