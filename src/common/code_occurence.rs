use crate::common::git::git_info::GitInformation;
use crate::common::git::git_info::GitInformationWithoutLifetimes;
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
use crate::traits::new_error_with_one_addition::NewErrorWithOneAddition;
use crate::traits::readable_time::ReadableTime;
use crate::traits::separator_symbol::SeparatorSymbol;
use crate::traits::new_error_with_git_info_file_line_column::NewErrorWithGitInfoFileLineColumn;
use ansi_term::Colour::RGB;
use chrono::prelude::DateTime;
use chrono::Utc;
use impl_get_source::ImplGetSourceFromCrate;
use std::collections::HashMap;
use std::fmt::{self, Display};
use crate::common::file_line_column::FileLineColumn;
use crate::common::time_file_line_column::TimeFileLineColumn;

#[derive(ImplGetSourceFromCrate)]
pub struct ThreeWrapperError {
    source: ThreeWrapperErrorEnum,
    code_occurence: CodeOccurence,
}

// impl crate::traits::get_source::GetSource for ThreeWrapperError {
//     fn get_source(&self) -> String {
//         self.source.get_source()
//     }
// }

impl crate::traits::init_error::InitError<ThreeWrapperErrorEnum>
    for ThreeWrapperError
{
    fn init_error(
        source: ThreeWrapperErrorEnum,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    ) -> Self {
        Self {
            source,
            code_occurence,
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurence for ThreeWrapperError {
    fn get_code_occurence(&self) -> &CodeOccurence {
        &self.code_occurence
    }
}
use crate::traits::my_custom_display::DisplayError;
pub fn three(should_trace: bool) -> Result<(), Box<ThreeWrapperError>> {
    if let Err(e) = four(false) {
        // println!("{}", <FourOriginError as DisplayError<SourceGeneric, ConfigStruct, ConfigStruct>>::display_error(&*e, once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG)));
        return Err(Box::new(ThreeWrapperError::new_error_with_one_addition(
            ThreeWrapperErrorEnum::FourWrapper(*e),
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG), 
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES), 
            String::from(file!()), 
            line!(), 
            column!(), 
            should_trace
        )));
    };
    Ok(())
}
//
#[derive(ImplGetSourceFromCrate)]
pub enum ThreeWrapperErrorEnum {
    FourWrapper(FourOriginError),
}

impl crate::traits::get_code_occurence::GetCodeOccurence for ThreeWrapperErrorEnum {
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(e) => e.get_code_occurence(),
        }
    }
}
//
#[derive(ImplGetSourceFromCrate)]
pub struct FourOriginError {
    source: HashMap<String, FourWrapperErrorEnum>,
    code_occurence: CodeOccurence,
}
//
impl crate::traits::init_error::InitError<HashMap<String, FourWrapperErrorEnum>>
    for FourOriginError
{
    fn init_error(
        source: HashMap<String, FourWrapperErrorEnum>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    ) -> Self {
        Self {
            source,
            code_occurence,
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurence for FourOriginError {
    fn get_code_occurence(&self) -> &CodeOccurence {
        &self.code_occurence
    }
}

#[derive(ImplGetSourceFromCrate)]
pub enum FourWrapperErrorEnum {
    FiveWrapper(FiveOriginError),
    SixWrapper(SixOriginError),
}
use crate::traits::get_source::GetSource;
impl std::fmt::Display for FourWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FourWrapperErrorEnum::FiveWrapper(e) => write!(f, "{}", e.get_source()),
            FourWrapperErrorEnum::SixWrapper(e) => write!(f, "{}", e.get_source()),
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurence for FourWrapperErrorEnum {
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
        match self {
            FourWrapperErrorEnum::FiveWrapper(e) => e.get_code_occurence(),
            FourWrapperErrorEnum::SixWrapper(e) => e.get_code_occurence(),
        }
    }
}
//
pub fn four(should_trace: bool) -> Result<(), Box<FourOriginError>> {
    // if let Err(e) = six(false) {
    //     return Err(Box::new(FourOriginError::new_error_with_one_addition(
    //         FourWrapperErrorEnum::SixWrapper(*e),
    //         once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG),
    //         once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES),
    //         String::from(file!()),
    //         line!(),
    //         column!(),
    //         should_trace
    //     )));
    // }
    // match (five(false), six(false)) {
    //     (Ok(_), Ok(_)) => todo!(),
    //     (Ok(_), Err(_)) => todo!(),
    //     (Err(_), Ok(_)) => todo!(),
    //     (Err(f), Err(s)) => {
    //         return Err(Box::new(FourOriginError::new_error_with_one_addition(
    //         HashMap::from([
    //             (
    //                 String::from("one"),
    //                 FourWrapperErrorEnum::FiveWrapper(*f)
    //             ),
    //             (
    //                 String::from("two"),
    //                 FourWrapperErrorEnum::SixWrapper(*s)
    //             )
    //         ]),
    //         once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG), 
    //         once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES), 
    //         String::from(file!()), 
    //         line!(), 
    //         column!(), 
    //         should_trace
    // )));
    //     }
    // }
    Ok(())
}
//
#[derive(ImplGetSourceFromCrate)]
pub struct FiveOriginError {
    source: u32,
    code_occurence: CodeOccurence,
}

impl crate::traits::init_error::InitError<u32> for FiveOriginError {
    fn init_error(source: u32, code_occurence: crate::common::code_occurence::CodeOccurence) -> Self {
        Self {
            source,
            code_occurence,
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurence for FiveOriginError {
    fn get_code_occurence(&self) -> &CodeOccurence {
        &self.code_occurence
    }
}
//
pub fn five(should_trace: bool) -> Result<(), Box<FiveOriginError>> {
    return Err(Box::new(FiveOriginError::new_error_with_git_info_file_line_column(
        34,
            crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES
            .clone(), 
            String::from(file!()), 
            line!(), 
            column!(), 
    )));
}
//
#[derive(ImplGetSourceFromCrate)]
pub struct SixOriginError {
    source: bool,
    code_occurence: CodeOccurence,
}

impl crate::traits::init_error::InitError<bool> for SixOriginError {
    fn init_error(source: bool, code_occurence: crate::common::code_occurence::CodeOccurence) -> Self {
        Self {
            source,
            code_occurence,
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurence for SixOriginError {
    fn get_code_occurence(&self) -> &CodeOccurence {
        &self.code_occurence
    }
}
//
pub fn six(should_trace: bool) -> Result<(), Box<SixOriginError>> {
    return Err(Box::new(SixOriginError::new_error_with_git_info_file_line_column(
        true,
            crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES
            .clone(), 
            String::from(file!()), 
            line!(), 
            column!(), 
    )));
}
//

#[derive(Debug, Clone)]
pub struct CodeOccurence {
    pub occurences: HashMap<GitInformationWithoutLifetimes, Vec<crate::common::increment_time_file_line_column::IncrementTimeFileLineColumn>>,
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
                vec_existing_value_elements.push(crate::common::increment_time_file_line_column::IncrementTimeFileLineColumn {
                    increment: new_last_increment,
                    concurrent_or_parallel_execution_index: None,
                    time_file_line_column: TimeFileLineColumn::new(FileLineColumn {
                        file: file.clone(),
                        line,
                        column,
                    }), //todo how to rewrite it without clone() ?
                });
            })
            .or_insert_with(|| {
                vec![crate::common::increment_time_file_line_column::IncrementTimeFileLineColumn {
                    increment: new_last_increment,
                    concurrent_or_parallel_execution_index: None,
                    time_file_line_column: TimeFileLineColumn::new(FileLineColumn {
                        file: file.clone(),
                        line,
                        column,
                    }),
                }]
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

//


// impl crate::traits::get_code_occurence::GetCodeOccurence
//     for HashMap<std::string::String, FourWrapperErrorEnum>
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
//         //todo
//         self.values().fold(
//             &CodeOccurence {
//                 occurences: HashMap::new(),//::<GitInformationWithoutLifetimes, Vec<IncrementTimeFileLineColumn>>
//             },
//             |mut acc, elem| {
//                 let code_occurence = match elem {
//                     FourWrapperErrorEnum::FiveWrapper(e) => e.get_code_occurence(),
//                     FourWrapperErrorEnum::SixWrapper(e) => e.get_code_occurence(),
//                 };
//                 code_occurence.occurences.iter().for_each(|(key, value)| {
//                     acc.entry(key)
//                         .and_modify(|vec_existing_value_elements| {
//                             vec_existing_value_elements.push(IncrementTimeFileLineColumn {
//                                 increment: new_last_increment,
//                                 concurrent_or_parallel_execution_index: None,
//                                 time_file_line_column: TimeFileLineColumn::new(FileLineColumn {
//                                     file: file.clone(),
//                                     line,
//                                     column,
//                                 }), //todo how to rewrite it without clone() ?
//                             });
//                         })
//                         .or_insert_with(|| {
//                             vec![IncrementTimeFileLineColumn {
//                                 increment: new_last_increment,
//                                 concurrent_or_parallel_execution_index: None,
//                                 time_file_line_column: TimeFileLineColumn::new(FileLineColumn {
//                                     file: file.clone(),
//                                     line,
//                                     column,
//                                 }),
//                             }]
//                         });
//                 });
//                 // let current_code_occurence = elem.get_code_occurence();
//                 // acc.push_str(elem);
//                 acc
//             },
//         );
//         let f = self.get("one").unwrap();
//         match f {
//             FourWrapperErrorEnum::FiveWrapper(e) => e.get_code_occurence(),
//             FourWrapperErrorEnum::SixWrapper(e) => e.get_code_occurence(),
//         }
//     }
// }

impl crate::traits::get_source::GetSource for HashMap<std::string::String, FourWrapperErrorEnum> {
    fn get_source(&self) -> String {
        String::from("todo this impl")
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
