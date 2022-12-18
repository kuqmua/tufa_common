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
// use crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceAndCodeOccurence;
// use crate::traits::new_error_with_git_info_file_line_column::SomethingTest;

#[derive(ImplGetSourceFromCrate)]
pub struct ThreeWrapperError {
    source: ThreeWrapperErrorEnum,
    code_occurence: crate::common::code_occurence::CodeOccurence,
}
// хранение данные в code_occurence для сериализации и вывод данных в консоль - 2 разные вещи
// impl sssssss for SixOriginError {
//     fn prepare_log_source_inner_and_code_occurence(
//         &self,
//         config_generic: &ConfigGeneric,
//     ) -> String {
// format!("{}{}", self.source.get_sourceand_code_occurence_as_string(), code_occurence из инпута параметра, не self. для self.code_occurence возможно будет тзасунуто больше чем нужно для вывода в консоль)
//     }
// }

impl crate::traits::get_source_value::GetSourceValue<ThreeWrapperErrorEnum> for ThreeWrapperError {
    fn get_source_value(&self) -> &ThreeWrapperErrorEnum {
        &self.source //.get_source_value() - get source value in string with code occurence and erroor
    }
}

// impl crate::traits::get_source::GetSource for ThreeWrapperError {
//     fn get_source_and_code_occurence_as_string(&self) -> String {
//         self.source.get_source()
//     }
// }

impl crate::traits::init_error::InitError<ThreeWrapperErrorEnum> for ThreeWrapperError {
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

        // return Err(Box::new(ThreeWrapperError::new_error_with_one_addition(
        //     ThreeWrapperErrorEnum::FourWrapper(*e),
        //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG),
        //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES),
        //     String::from(file!()),
        //     line!(),
        //     column!(),
        //     should_trace
        // )));

        return Err(Box::new(ThreeWrapperError {
            source: ThreeWrapperErrorEnum::FourWrapper(*e),
            code_occurence: crate::common::code_occurence::CodeOccurence {
                occurences: HashMap::new(),
            },
        }));
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

impl crate::traits::get_source_value::GetSourceValue<HashMap<String, FourWrapperErrorEnum>>
    for FourOriginError
{
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
            //     return Err(Box::new(FourOriginError::new_error_with_one_addition(
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

            return Err(Box::new(FourOriginError {
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
                code_occurence: crate::common::code_occurence::CodeOccurence {
                    occurences: HashMap::new(),
                },
            }));
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
    fn init_error(
        source: String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    ) -> Self {
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
    // return Err(Box::new(FiveOriginError::something_test(
    //             String::from("error_five"),
    //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG),
    //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES),
    //     String::from(file!()),
    //     line!(),
    //     column!(),
    //     false
    // )));
    //
    return Err(Box::new(FiveOriginError {
        source: String::from("error_five"),
        code_occurence: crate::common::code_occurence::CodeOccurence {
            occurences: HashMap::new(),
        },
    }));
}
//
use crate::traits::code_occurence_methods::CodeOccurenceToString;
use crate::traits::fields::GetLogType;
use crate::traits::fields::GetSourcePlaceType;
use crate::traits::get_color::ErrorColorBold;
use crate::traits::get_source_value::GetSourceValue;
use crate::traits::readable_time_string::ReadableTimeString;
// use crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceInnerAndCodeOccurence;

#[derive(ImplGetSourceFromCrate)]
pub struct SixOriginError {
    source: String,
    code_occurence: crate::common::code_occurence::CodeOccurence,
}

// impl SixOriginError {
//     fn new_error_with_one_addition_test_six(
//         source: String,
//         config: crate::config_mods::config_struct::ConfigStruct,
//         git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
//         file: String,
//         line: u32,
//         column: u32,
//         should_trace: bool,
//     ) -> Self {
//         let code_occurence =
//             crate::common::code_occurence::CodeOccurence::new(git_info.clone(), file, line, column);
//         let string_code_occurence = code_occurence.code_occurence_to_string(&config);
//         let log_type = config.get_log_type().symbol();
//         // let g = ReturnSelfGeneric::init_error(source, code_occurence);
//         let g = SixOriginError {
//             source,
//             code_occurence,
//         };
//         if let true = should_trace {
//             //
//             let prepared = {
//                 let capacity =
//                     g.get_code_occurence()
//                         .occurences
//                         .values()
//                         .fold(0, |mut acc, elem| {
//                             acc += elem.len();
//                             acc
//                         });
//                 let mut vec: Vec<crate::common::code_occurence::OccurenceFilter> =
//                     Vec::with_capacity(capacity);
//                 g.get_code_occurence()
//                     .occurences
//                     .iter()
//                     .for_each(|(git_info, v)| {
//                         v.iter().for_each(|e| {
//                             vec.push(crate::common::code_occurence::OccurenceFilter {
//                                 increment: e.increment,
//                                 time: e.time_file_line_column.time,
//                                 occurence: e
//                                     .time_file_line_column
//                                     .get_code_path(git_info, config.get_source_place_type()),
//                             })
//                         })
//                     });
//                 //vec.reverse();//todo check reserve or not
//                 vec.sort_by(|a, b| a.increment.cmp(&b.increment));
//                 let mut occurences = Vec::with_capacity(capacity + 1);
//                 let log_type = config.get_log_type();
//                 occurences.push(format!("////{}{}////", g.get_source(), log_type.symbol())); //here must be self.get_source_value().
//                                                                                              // occurences.push(format!("{}{}", self.get_source_value().prepare_log_source_and_code_occurence(), log_type.symbol()));
//                 vec.into_iter().for_each(|e| {
//                     occurences.push(format!(
//                         "{} {}{}",
//                         e.readable_time_string(),
//                         e.occurence,
//                         log_type.symbol()
//                     ));
//                 });
//                 let mut occurence = occurences.iter().fold(String::from(""), |mut acc, elem| {
//                     acc.push_str(elem);
//                     acc
//                 });
//                 log_type.pop_last(&mut occurence);
//                 occurence
//             };
//             //
//             config.get_log_type().console(
//                 config.get_error_color_bold(),
//                 format!(
//                     "{}",
//                     // source.prepare_log_source_inner_and_code_occurence(&config),
//                     prepared, // g.prepare_log_source_inner_and_code_occurence(&config),
//                               // log_type,
//                               // string_code_occurence
//                 ),
//             );
//         }
//         g
//     }
// }

impl crate::traits::get_source_value::GetSourceValue<String> for SixOriginError {
    fn get_source_value(&self) -> &String {
        &self.source
    }
}

impl crate::traits::init_error::InitError<String> for SixOriginError {
    fn init_error(
        source: String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    ) -> Self {
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
    // return Err(Box::new(SixOriginError::something_test(
    //     String::from("error_six"),
    //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG),
    //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES),
    //     String::from(file!()),
    //     line!(),
    //     column!(),
    //     false,
    // ) ));
    //
    return Err(Box::new(SixOriginError {
        source: String::from("error_six"),
        code_occurence: crate::common::code_occurence::CodeOccurence {
            occurences: HashMap::new(),
        },
    }));
}

// impl crate::traits::get_source::GetSource for HashMap<std::string::String, FourWrapperErrorEnum> {
//     fn get_source(&self) -> String {
//         String::from("todo this impl")
//     }
// }
