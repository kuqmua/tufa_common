use crate::common::code_occurence;
use crate::traits::code_occurence_methods::CodeOccurenceLog;
use crate::traits::code_occurence_methods::CodeOccurenceNewErrorWithOneAddition;

pub trait NewErrorWithOneAddition<SourceGeneric, ConfigGeneric, ReturnSelfGeneric> {
    fn new_error_with_one_addition(
        source: SourceGeneric,
        config: ConfigGeneric,
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String,
        line: u32,
        column: u32,
        should_trace: bool,
    ) -> ReturnSelfGeneric;
}

impl<SourceGeneric, ConfigGeneric, ReturnSelfGeneric>
    NewErrorWithOneAddition<SourceGeneric, ConfigGeneric, ReturnSelfGeneric> for ReturnSelfGeneric
where
    SourceGeneric:
        crate::traits::get_source::GetSource + crate::traits::get_code_occurence::GetCodeOccurence,
    ReturnSelfGeneric: crate::traits::init_error::InitError<SourceGeneric>
        + crate::traits::log_error_code_occurence::LogErrorCodeOccurence<ConfigGeneric>,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetLogType
        + crate::traits::get_color::ErrorColorBold,
{
    fn new_error_with_one_addition(
        source: SourceGeneric,
        config: ConfigGeneric,
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String,
        line: u32,
        column: u32,
        should_trace: bool,
    ) -> ReturnSelfGeneric {
        let code_occurence =
            crate::common::code_occurence::CodeOccurence::new_error_with_one_addition(
                git_info, file, line, column, &source,
            );
        let error = ReturnSelfGeneric::init_error(source, code_occurence);
        if let true = should_trace {
            error.log_error_code_occurence(config);
        }
        error
    }
}
// //
// impl<SourceGeneric, ConfigGeneric, ReturnSelfGeneric>
//     NewErrorWithOneAddition<SourceGeneric, ConfigGeneric, ReturnSelfGeneric> for ReturnSelfGeneric
// where
//     SourceGeneric:
//         crate::traits::get_source::GetSource
//             + crate::traits::get_code_occurence::GetCodeOccurence
//             + crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceAndCodeOccurence,
//     ReturnSelfGeneric: crate::traits::init_error::InitError<SourceGeneric>
//         + crate::traits::log_error_code_occurence::LogErrorCodeOccurence<ConfigGeneric>,
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetLogType
//         + crate::traits::get_color::ErrorColorBold,
// {
//     fn new_error_with_one_addition(
//         source: SourceGeneric,
//         config: ConfigGeneric,
//         git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
//         file: String,
//         line: u32,
//         column: u32,
//         should_trace: bool,
//     ) -> ReturnSelfGeneric {
//         // let code_occurence =
//         //     crate::common::code_occurence::CodeOccurence::new_error_with_one_addition(
//         //         git_info, file, line, column, &source,
//         //     );
//         let code_occurence =
//             crate::common::code_occurence::CodeOccurence::new(git_info, file, line, column);
//         let error = ReturnSelfGeneric::init_error(source, code_occurence);
//         if let true = should_trace {
//             // // error.log_error_code_occurence(config);
//             // self.get_code_occurence().log(
//             //     // self.get_source(),
//             //     self, config,
//             // );
//             // //
//             // let log_type = config_generic.get_log_type();
//             // let error_color_bold = config_generic.get_error_color_bold();
//             // log_type.console(
//             //     error_color_bold,
//             //     self.code_occurence_with_source_to_string(source_generic, &config_generic),
//             // )
//         }
//         error
//     }
// }
