pub trait MyCustomDisplay<SourceGeneric, ConfigGeneric, ErrorColorBoldGeneric, ReturnSelfGeneric> {
    fn display_source(&self) -> String;
    fn display_code_occurence(&self) -> String;
    fn display_full_error(&self) -> String;
}

// impl<SourceGeneric, ConfigGeneric, ErrorColorBoldGeneric, ReturnSelfGeneric>
//     NewErrorWithOneAddition<SourceGeneric, ConfigGeneric, ErrorColorBoldGeneric, ReturnSelfGeneric>
//     for ReturnSelfGeneric
// where
//     SourceGeneric:
//         crate::traits::get_source::GetSource + crate::traits::get_code_occurence::GetCodeOccurence,
//     ReturnSelfGeneric: crate::traits::new_error_test::NewErrorTest<SourceGeneric>
//         + crate::traits::log_error_code_occurence::LogErrorCodeOccurence<
//             ConfigGeneric,
//             ErrorColorBoldGeneric,
//         >,
//     ConfigGeneric: crate::config_mods::traits::fields::GetSourcePlaceType
//         + crate::config_mods::traits::fields::GetLogType
//         + crate::traits::get_color::ErrorColorBold<ErrorColorBoldGeneric>,
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
//         let error = ReturnSelfGeneric::new_with_code_occurance(source, code_occurence);
//         if let true = should_trace {
//             error.log_error_code_occurence(config);
//         }
//         error
//     }
// }
