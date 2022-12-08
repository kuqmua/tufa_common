use crate::common::git::git_info::GitInformation;
use crate::common::git::git_info::GitInformationWithoutLifetimes;
use crate::common::where_was::WhereWas;
use crate::config_mods::log_type::LogType;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::code_occurence_methods::CodeOccurenceMethods;
use crate::traits::code_path::CodePath;
use crate::traits::console::Console;
use crate::traits::get_code_occurence::GetCodeOccurence;
use crate::traits::get_git_source_file_link::GetGitSourceFileLink;
use crate::traits::new_error_test::NewErrorTest;
use crate::traits::readable_time::ReadableTime;
use crate::traits::readable_time_string::ReadableTimeString;
use crate::traits::separator_symbol::SeparatorSymbol;
use ansi_term::Colour::RGB;
use chrono::prelude::DateTime;
use chrono::Utc;
use impl_get_source::ImplGetSourceFromCrate;
use std::collections::HashMap;
use std::fmt::{self, Display};

#[derive(ImplGetSourceFromCrate, Clone)]
pub struct ThreeOriginError {
    source: u32,
    code_occurence: CodeOccurence,
}

impl crate::traits::new_error_test::NewErrorTestTestTest<u32> for ThreeOriginError {
    fn new(source: u32, code_occurence: crate::common::code_occurence::CodeOccurence) -> Self {
        Self {
            source,
            code_occurence,
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurence for ThreeOriginError {
    fn get_code_occurence(&self) -> &CodeOccurence {
        &self.code_occurence
    }
}

pub fn three() -> Result<(), Box<ThreeOriginError>> {
    return Err(Box::new(ThreeOriginError::new_with_git_info_file_line_column(
        34,
        crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES
            .clone(),
        String::from(file!()),
        line!(),
        column!(),
    )));
}

#[derive(Debug, Clone)]
pub struct CodeOccurence {
    pub occurences: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
}

impl CodeOccurenceMethods for CodeOccurence {
    fn new(
        git_info: GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            occurences: HashMap::from([(
                git_info,
                vec![TimeFileLineColumnIncrement::new(file, line, column)],
            )]),
        }
    }
    fn new_with_addition(
        git_info: &GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
        another_code_occurence: &Self,
    ) -> Self {
        let capacity = another_code_occurence
            .occurences
            .values()
            .fold(0, |mut acc, elem| {
                acc += elem.len();
                acc
            });
        let mut occurences = HashMap::with_capacity(capacity + 1);
        let mut new_last_increment = {
            let mut increment_handle = 0;
            another_code_occurence.occurences.values().for_each(|v| {
                v.iter().for_each(|e| {
                    if e.increment > increment_handle {
                        increment_handle = e.increment;
                    }
                });
            });
            increment_handle
        } + 1;
        occurences = another_code_occurence.occurences.clone();
        occurences
            .entry(git_info.clone())
            .and_modify(|vec_existing_value_elements| {
                vec_existing_value_elements.push(TimeFileLineColumnIncrement {
                    increment: new_last_increment,
                    time_file_line_column: TimeFileLineColumn::new(FileLineColumn {
                        file: file.clone(),
                        line,
                        column,
                    }), //todo how to rewrite it without clone() ?
                });
            })
            .or_insert_with(|| {
                vec![TimeFileLineColumnIncrement {
                    increment: new_last_increment,
                    time_file_line_column: TimeFileLineColumn::new(FileLineColumn {
                        file: file.clone(),
                        line,
                        column,
                    }),
                }]
            });
        Self { occurences }
    }
    fn log_code_occurence(
        &self,
        source_place_type: &SourcePlaceType,
        log_type: &LogType,
        source: String,
        style: ansi_term::Style,
    ) {
        let capacity = self.occurences.values().fold(0, |mut acc, elem| {
            acc += elem.len();
            acc
        });
        let mut vec: Vec<OccurenceFilter> = Vec::with_capacity(capacity);
        self.occurences.iter().for_each(|(git_info, v)| {
            v.iter().for_each(|e| {
                vec.push(OccurenceFilter {
                    increment: e.increment,
                    time: e.time_file_line_column.time,
                    occurence: e
                        .time_file_line_column
                        .get_code_path(git_info, source_place_type), // .get_project_code_path()
                })
            })
        });
        //vec.reverse();//todo check reserve or not
        vec.sort_by(|a, b| a.increment.cmp(&b.increment));
        let mut occurences = Vec::with_capacity(capacity + 1);
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
}

#[derive(Debug, Clone)]
pub struct OccurenceFilter {
    pub increment: u64, //potential overflow?
    pub time: std::time::Duration,
    pub occurence: String,
}

impl ReadableTimeString for OccurenceFilter {
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

impl ReadableTimeString for TimeFileLineColumnIncrement {
    fn readable_time_string(&self) -> String {
        self.time_file_line_column.readable_time_string()
    }
}

impl crate::traits::file_line_column::GetFile for TimeFileLineColumnIncrement {
    fn get_file(&self) -> &String {
        &self.time_file_line_column.get_file()
    }
}

impl crate::traits::file_line_column::GetLine for TimeFileLineColumnIncrement {
    fn get_line(&self) -> u32 {
        self.time_file_line_column.get_line()
    }
}

impl crate::traits::file_line_column::GetColumn for TimeFileLineColumnIncrement {
    fn get_column(&self) -> u32 {
        self.time_file_line_column.get_column()
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

impl ReadableTimeString for TimeFileLineColumn {
    fn readable_time_string(&self) -> String {
        DateTime::<Utc>::from(std::time::UNIX_EPOCH + self.time)
            .format("%Y-%m-%d %H:%M:%S.%f")
            .to_string()
    }
}

impl ReadableTime for TimeFileLineColumn {
    fn readable_time(&self, timezone: i32) -> String {
        chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.time)
            .with_timezone(&chrono::FixedOffset::east(timezone))
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}

impl crate::traits::file_line_column::GetFile for TimeFileLineColumn {
    fn get_file(&self) -> &String {
        &self.file_line_column.get_file()
    }
}

impl crate::traits::file_line_column::GetLine for TimeFileLineColumn {
    fn get_line(&self) -> u32 {
        self.file_line_column.get_line()
    }
}

impl crate::traits::file_line_column::GetColumn for TimeFileLineColumn {
    fn get_column(&self) -> u32 {
        self.file_line_column.get_column()
    }
}

#[derive(Debug, Clone)]
pub struct FileLineColumn {
    pub file: String, //&'a str
    pub line: u32,
    pub column: u32,
}

impl crate::traits::file_line_column::GetFile for FileLineColumn {
    fn get_file(&self) -> &String {
        &self.file
    }
}

impl crate::traits::file_line_column::GetLine for FileLineColumn {
    fn get_line(&self) -> u32 {
        self.line
    }
}

impl crate::traits::file_line_column::GetColumn for FileLineColumn {
    fn get_column(&self) -> u32 {
        self.column
    }
}
