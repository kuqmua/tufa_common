use crate::common::file_line_column::FileLineColumn;
use crate::common::git::git_info::GitInformation;
use crate::common::git::git_info::GitInformationWithoutLifetimes;
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
use crate::traits::init_error::InitError;
use crate::traits::new_error_with_git_info_file_line_column::NewErrorWithGitInfoFileLineColumn;
use crate::traits::new_error_with_one_addition::NewErrorWithOneAddition;
use crate::traits::readable_time::ReadableTime;
use crate::traits::separator_symbol::SeparatorSymbol;
use ansi_term::Colour::RGB;
use chrono::prelude::DateTime;
use chrono::Utc;
use impl_get_source::ImplGetSourceFromCrate;
use std::collections::HashMap;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct CodeOccurence {
    pub occurences: HashMap<
        GitInformationWithoutLifetimes,
        Vec<crate::common::increment_time_file_line_column::IncrementTimeFileLineColumn>,
    >,
}

impl<SourceGeneric>
    crate::traits::code_occurence_methods::CodeOccurenceNewErrorWithOneAddition<SourceGeneric>
    for CodeOccurence
where
    SourceGeneric: crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn new_error_with_one_addition(
        git_info: &GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
        source_generic: &SourceGeneric,
    ) -> Self {
        let mut occurences =
            HashMap::with_capacity(source_generic.get_code_occurence().occurences.keys().len() + 1);
        let mut new_last_increment = {
            let mut increment_handle = 0;
            source_generic
                .get_code_occurence()
                .occurences
                .values()
                .for_each(|v| {
                    v.iter().for_each(|e| {
                        if e.increment > increment_handle {
                            increment_handle = e.increment;
                        }
                    });
                });
            increment_handle
        } + 1;
        occurences = source_generic.get_code_occurence().occurences.clone();
        occurences
            .entry(git_info.clone())
            .and_modify(|vec_existing_value_elements| {
                vec_existing_value_elements.push(
                    crate::common::increment_time_file_line_column::IncrementTimeFileLineColumn {
                        increment: new_last_increment,
                        concurrent_or_parallel_execution_index: None,
                        time_file_line_column: TimeFileLineColumn::new(FileLineColumn {
                            file: file.clone(),
                            line,
                            column,
                        }), //todo how to rewrite it without clone() ?
                    },
                );
            })
            .or_insert_with(|| {
                vec![
                    crate::common::increment_time_file_line_column::IncrementTimeFileLineColumn {
                        increment: new_last_increment,
                        concurrent_or_parallel_execution_index: None,
                        time_file_line_column: TimeFileLineColumn::new(FileLineColumn {
                            file: file.clone(),
                            line,
                            column,
                        }),
                    },
                ]
            });
        Self { occurences }
    }
}

impl crate::traits::code_occurence_methods::CodeOccurenceNew for CodeOccurence {
    fn new(
        git_info: GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            occurences: HashMap::from([(
                git_info,
                vec![crate::common::increment_time_file_line_column::IncrementTimeFileLineColumn::new(file, line, column)],
            )]),
        }
    }
}

use crate::traits::readable_time_string::ReadableTimeString;
#[derive(Debug, Clone)]
pub struct OccurenceFilter {
    pub increment: u64, //potential overflow?
    pub time: std::time::Duration,
    pub occurence: String,
}

impl crate::traits::get_time::GetTime for OccurenceFilter {
    fn get_time(&self) -> std::time::Duration {
        self.time
    }
}
