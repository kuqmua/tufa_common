use crate::common::git::git_info::GitInformation;
use crate::common::git::git_info::GitInformationWithoutLifetimes;
use crate::common::where_was::WhereWas;
use crate::config_mods::log_type::LogType;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::code_occurence_methods;
use crate::traits::code_path::CodePath;
use crate::traits::console::Console;
use crate::traits::get_code_occurence::GetCodeOccurence;
use crate::traits::get_git_source_file_link::GetGitSourceFileLink;
use crate::traits::new_error_test::NewErrorTest;
use crate::traits::new_error_with_one_addition::NewErrorWithOneAddition;
use crate::traits::readable_time::ReadableTime;
use crate::traits::readable_time_string::ReadableTimeString;
use crate::traits::separator_symbol::SeparatorSymbol;
use ansi_term::Colour::RGB;
use chrono::prelude::DateTime;
use chrono::Utc;
use impl_get_source::ImplGetSourceFromCrate;
use std::collections::HashMap;
use std::fmt::{self, Display};
use crate::global_variables::runtime::config::CONFIG;

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

impl crate::traits::new_error_test::NewErrorTestTestTest<ThreeWrapperErrorEnum>
    for ThreeWrapperError
{
    fn new(
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
    source: FourWrapperErrorEnum,//HashMap<String, FourWrapperErrorEnum>,
    code_occurence: CodeOccurence,
}
//HashMap<String, FourWrapperErrorEnum>
impl crate::traits::new_error_test::NewErrorTestTestTest<FourWrapperErrorEnum> for FourOriginError {
    fn new(
        source: FourWrapperErrorEnum,//HashMap<String, FourWrapperErrorEnum>, 
        code_occurence: crate::common::code_occurence::CodeOccurence) -> Self {
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
            FourWrapperErrorEnum::SixWrapper(e) => e.get_code_occurence()
        }
    }
}
//
pub fn four(should_trace: bool) -> Result<(), Box<FourOriginError>> {
    if let Err(e) = six(false) {
        
                         return Err(Box::new(FourOriginError::new_error_with_one_addition(
            FourWrapperErrorEnum::SixWrapper(*e),
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG), 
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES), 
            String::from(file!()), 
            line!(), 
            column!(), 
            should_trace
    )));
    }
    // match (five(false), six(false)) {
    //     (Ok(_), Ok(_)) => todo!(),
    //     (Ok(_), Err(_)) => todo!(),
    //     (Err(_), Ok(_)) => todo!(),
    //     (Err(f), Err(s)) => {
    //              return Err(Box::new(FourOriginError::new_error_with_one_addition(
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
    //     },
    // }
    Ok(())
}
//
#[derive(ImplGetSourceFromCrate)]
pub struct FiveOriginError {
    source: u32,
    code_occurence: CodeOccurence,
}



impl crate::traits::new_error_test::NewErrorTestTestTest<u32> for FiveOriginError {
    fn new(source: u32, code_occurence: crate::common::code_occurence::CodeOccurence) -> Self {
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
    return Err(Box::new(FiveOriginError::new_with_git_info_file_line_column(
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

impl crate::traits::new_error_test::NewErrorTestTestTest<bool> for SixOriginError {
    fn new(source: bool, code_occurence: crate::common::code_occurence::CodeOccurence) -> Self {
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
    return Err(Box::new(SixOriginError::new_with_git_info_file_line_column(
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
    pub occurences: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
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
                vec_existing_value_elements.push(TimeFileLineColumnIncrement {
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
                vec![TimeFileLineColumnIncrement {
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
                vec![TimeFileLineColumnIncrement::new(file, line, column)],
            )]),
        }
    }
}

//
pub trait FromFewCodeOccurencesHashMap<KeyGeneric, ValueGeneric> {
    fn from_few_code_occurences_hashmap(&self) -> CodeOccurence;
}

impl<KeyGeneric, ValueGeneric> FromFewCodeOccurencesHashMap<KeyGeneric, ValueGeneric> for HashMap<KeyGeneric, ValueGeneric> 
where 
    ValueGeneric: crate::traits::get_code_occurence::GetCodeOccurence
{
    fn from_few_code_occurences_hashmap(&self) -> CodeOccurence {
        let mut parallel_counter = 0;//todo - add some field to code occurence?
        let mut formatted = self
        // .iter()
        .values()
        // .map(|(code_occurence)| code_occurence)
        // .collect::<Vec<String>>()
        // .iter()
        .fold(HashMap::<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>::new(), |mut acc, elem| {
            let current_code_occurence = elem.get_code_occurence();
            // acc.push_str(elem);
            acc
        })
        ;
        // if !formatted.is_empty() {
        //     formatted.pop();
        // }
        // formatted
        CodeOccurence {
            occurences: HashMap::new(),
        }
    }
}
//

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
    pub increment: u64, //potential overflow?
    pub concurrent_or_parallel_execution_index: Option<u64>,//for information about parallel error result like inside join_all!() or join!()
    pub time_file_line_column: TimeFileLineColumn,
}

impl TimeFileLineColumnIncrement {
    pub fn new(
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            increment: 0, //potential overflow?
            concurrent_or_parallel_execution_index: None,
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
    pub time: std::time::Duration,
    pub file_line_column: FileLineColumn,
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
