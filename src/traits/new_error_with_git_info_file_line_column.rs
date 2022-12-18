// use crate::traits::code_occurence_methods::CodeOccurenceNew;
// use crate::traits::init_error::InitError;

// pub trait NewErrorWithGitInfoFileLineColumn<SourceGeneric> {
//     fn new_error_with_git_info_file_line_column(
//         source: SourceGeneric,
//         git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
//         file: String, //&'a str
//         line: u32,
//         column: u32,
//     ) -> Self;
// }

// impl<SourceGeneric, ReturnSelfGeneric> NewErrorWithGitInfoFileLineColumn<SourceGeneric>
//     for ReturnSelfGeneric
// where
//     ReturnSelfGeneric: InitError<SourceGeneric>,
// {
//     fn new_error_with_git_info_file_line_column(
//         source: SourceGeneric,
//         git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
//         file: String, //&'a str
//         line: u32,
//         column: u32,
//     ) -> ReturnSelfGeneric {
//         ReturnSelfGeneric::init_error(
//             source,
//             crate::common::code_occurence::CodeOccurence::new(git_info, file, line, column),
//         )
//     }
// }

//
// use crate::traits::console::Console;
// pub trait SomethingTest<SourceGeneric, ConfigGeneric, ReturnSelfGeneric> {
//     fn something_test(
//         source: SourceGeneric,
//         config: ConfigGeneric,
//         git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
//         file: String,
//         line: u32,
//         column: u32,
//         should_trace: bool,
//     ) -> ReturnSelfGeneric;
// }

// impl<SourceGeneric, ConfigGeneric, ReturnSelfGeneric>
//     SomethingTest<SourceGeneric, ConfigGeneric, ReturnSelfGeneric> for ReturnSelfGeneric
// where
//     // SourceGeneric:
//     // crate::traits::get_source::GetSource,
//     // crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceAndCodeOccurence<
//     //     ConfigGeneric,
//     // >,
//     ReturnSelfGeneric:
//         crate::traits::init_error::InitError<SourceGeneric>
//             + crate::traits::log_error_code_occurence::LogErrorCodeOccurence<ConfigGeneric>
//             + crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceAndCodeOccurence<
//                 ConfigGeneric,
//             >,
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetLogType
//         + crate::traits::get_color::ErrorColorBold,
// {
//     fn something_test(
//         source: SourceGeneric,
//         config: ConfigGeneric,
//         git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
//         file: String,
//         line: u32,
//         column: u32,
//         should_trace: bool,
//     ) -> ReturnSelfGeneric {
//         let code_occurence =
//             crate::common::code_occurence::CodeOccurence::new(git_info.clone(), file, line, column);
//         let error = ReturnSelfGeneric::init_error(source, code_occurence);
//         if let true = should_trace {
//             config.get_log_type().console(
//                 config.get_error_color_bold(),
//                 error.prepare_log_source_and_code_occurence(&config),
//             );
//         }
//         error
//     }
// }
