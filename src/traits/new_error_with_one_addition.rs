use crate::common::code_occurence;
use crate::traits::code_occurence_methods::CodeOccurenceLog;
use crate::traits::code_occurence_methods::CodeOccurenceNew;
use crate::traits::code_occurence_methods::CodeOccurenceNewErrorWithOneAddition;
use crate::traits::code_occurence_methods::CodeOccurenceToString;
use crate::traits::console::Console;
use crate::traits::separator_symbol::SeparatorSymbol;

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

// impl<SourceGeneric, ConfigGeneric, ReturnSelfGeneric>
//     NewErrorWithOneAddition<SourceGeneric, ConfigGeneric, ReturnSelfGeneric> for ReturnSelfGeneric
// where
//     SourceGeneric:
//         crate::traits::get_source::GetSource + crate::traits::get_code_occurence::GetCodeOccurence,
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
//         let code_occurence =
//             crate::common::code_occurence::CodeOccurence::new_error_with_one_addition(
//                 git_info, file, line, column, &source,
//             );
//         let error = ReturnSelfGeneric::init_error(source, code_occurence);
//         if let true = should_trace {
//             error.log_error_code_occurence(config);
//         }
//         error
//     }
// }
// //
impl<SourceGeneric, ConfigGeneric, ReturnSelfGeneric>
    NewErrorWithOneAddition<SourceGeneric, ConfigGeneric, ReturnSelfGeneric> for ReturnSelfGeneric
where
    SourceGeneric:
        crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceAndCodeOccurence<
            ConfigGeneric,
        >,
    ReturnSelfGeneric: crate::traits::init_error::InitError<SourceGeneric>
        + crate::traits::log_error_code_occurence::LogErrorCodeOccurence<ConfigGeneric>
        + crate::traits::get_source_value::GetSourceValue<SourceGeneric>
        + crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceInnerAndCodeOccurence<ConfigGeneric, SourceGeneric>,
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
            crate::common::code_occurence::CodeOccurence::new(git_info.clone(), file, line, column);
        let string_code_occurence = code_occurence.code_occurence_to_string(&config);
        let log_type = config.get_log_type().symbol();
        // let f = sour
        // occurences.push(format!("{}{}", self.get_source_value().prepare_log_source_and_code_occurence(), log_type.symbol()));
        let g = ReturnSelfGeneric::init_error(source, code_occurence);
        if let true = should_trace {
            // config.get_log_type().console(
            //     config.get_error_color_bold(),
            //     format!(
            //         "{}{}{}",
            //         source.prepare_log_source_and_code_occurence(&config),
            //         log_type,
            //         string_code_occurence
            //     ),
            // );

            config.get_log_type().console(
                config.get_error_color_bold(),
                format!(
                    "{}",
                    // source.prepare_log_source_inner_and_code_occurence(&config),
                    g.prepare_log_source_inner_and_code_occurence(&config),
                    // log_type,
                    // string_code_occurence
                ),
            );
            //prepare_log_source_inner_and_code_occurence
        }

        // let j = g.get_source_value(); // not actuaally this .prepare_log_source_and_code_occurence(&config);
        g
    }
}
