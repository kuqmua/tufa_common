
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
use std::fmt::{self, Display};
use crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceAndCodeOccurence;
use crate::traits::new_error_with_git_info_file_line_column::SomethingTest;

#[derive(ImplGetSourceFromCrate)]
pub struct ThreeWrapperError {
    source: ThreeWrapperErrorEnum,
    code_occurence: crate::common::code_occurence::CodeOccurence,
}

impl crate::traits::get_source_value::GetSourceValue<ThreeWrapperErrorEnum> for ThreeWrapperError {
    fn get_source_value(&self) -> &ThreeWrapperErrorEnum {
        &self.source
    }
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
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
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
    code_occurence: crate::common::code_occurence::CodeOccurence,
}

impl crate::traits::get_source_value::GetSourceValue<HashMap<String, FourWrapperErrorEnum>> for FourOriginError {
    fn get_source_value(&self) -> &HashMap<String, FourWrapperErrorEnum> {
        &self.source
    }
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
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
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
use crate::traits::code_occurence_methods::CodeOccurenceNew;
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
    match (five(false), six(false)) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            return Err(Box::new(FourOriginError::new_error_with_one_addition(
         HashMap::from([
                    (
                        String::from("five_hashmap_key"),
                        FourWrapperErrorEnum::FiveWrapper(*f)
                    ),
                    (
                        String::from("six_hashmap_key"),
                        FourWrapperErrorEnum::SixWrapper(*s)
                    )
                ]),
                once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG), 
                once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES), 
                String::from(file!()), 
                line!(), 
                column!(), 
                should_trace
            )));
        }
    }
    Ok(())
}
//
#[derive(ImplGetSourceFromCrate)]
pub struct FiveOriginError {
    source: String,
    code_occurence: crate::common::code_occurence::CodeOccurence,
}

impl crate::traits::get_source_value::GetSourceValue<String> for FiveOriginError {
    fn get_source_value(&self) -> &String {
        &self.source
    }
}

impl crate::traits::init_error::InitError<String> for FiveOriginError {
    fn init_error(source: String, code_occurence: crate::common::code_occurence::CodeOccurence) -> Self {
        Self {
            source,
            code_occurence,
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurence for FiveOriginError {
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
        &self.code_occurence
    }
}
//
pub fn five(should_trace: bool) -> Result<(), Box<FiveOriginError>> {

    // let f = FiveOriginError::new_error_with_git_info_file_line_column(
    //     34,
    //         crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES
    //         .clone(), 
    //         String::from(file!()), 
    //         line!(), 
    //         column!(), 
    // );
    return Err(Box::new(FiveOriginError::something_test(
                String::from("error_five"),
        once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG), 
        once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES), 
        String::from(file!()),
        line!(), 
        column!(), 
        false
    )));
}
//
#[derive(ImplGetSourceFromCrate)]
pub struct SixOriginError {
    source: String,
    code_occurence: crate::common::code_occurence::CodeOccurence,
}

impl crate::traits::get_source_value::GetSourceValue<String> for SixOriginError {
    fn get_source_value(&self) -> &String {
        &self.source
    }
}

impl crate::traits::init_error::InitError<String> for SixOriginError {
    fn init_error(source: String, code_occurence: crate::common::code_occurence::CodeOccurence) -> Self {
        Self {
            source,
            code_occurence,
        }
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurence for SixOriginError {
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
        &self.code_occurence
    }
}
use crate::traits::get_source_and_code_occurence::GetSourceAndCodeOccurence;
pub fn six(should_trace: bool) -> Result<(), Box<SixOriginError>> {
    return Err(Box::new(SixOriginError::something_test(
        String::from("error_six"),
        once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG),
        once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES),
        String::from(file!()),
        line!(),
        column!(),
        false,
    ) ));
}

// impl crate::traits::get_source::GetSource for HashMap<std::string::String, FourWrapperErrorEnum> {
//     fn get_source(&self) -> String {
//         String::from("todo this impl")
//     }
// }

