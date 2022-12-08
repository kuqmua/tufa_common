use crate::traits::code_occurence_methods::CodeOccurenceMethods;

pub trait NewErrorTestTest<SourceGeneric, ConfigGeneric, ErrorColorBoldGeneric, ReturnSelfGeneric> {
    fn new_error_test_test(
        source: SourceGeneric,
        config: ConfigGeneric,
        git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String,
        line: u32,
        column: u32,
        should_trace: bool,
    ) -> ReturnSelfGeneric;
}

impl<SourceGeneric, ConfigGeneric, ErrorColorBoldGeneric, ReturnSelfGeneric>
    NewErrorTestTest<SourceGeneric, ConfigGeneric, ErrorColorBoldGeneric, ReturnSelfGeneric>
    for ReturnSelfGeneric
where
    SourceGeneric:
        crate::traits::get_source::GetSource + crate::traits::get_code_occurence::GetCodeOccurence,
    ReturnSelfGeneric: crate::traits::new_error_test::NewErrorTest<SourceGeneric>
        + crate::traits::log_error_code_occurence::LogErrorCodeOccurence,
    ConfigGeneric: crate::config_mods::traits::fields::GetSourcePlaceType
        + crate::config_mods::traits::fields::GetLogType
        + crate::traits::get_color::ErrorColorBold<ErrorColorBoldGeneric>,
{
    fn new_error_test_test(
        source: SourceGeneric,
        config: ConfigGeneric,
        git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String,
        line: u32,
        column: u32,
        should_trace: bool,
    ) -> ReturnSelfGeneric {
        let code_occurence =
            crate::common::code_occurence::CodeOccurence::new(git_info, file, line, column)
                .add(source.get_code_occurence());
        let error = ReturnSelfGeneric::new_error_test(source, code_occurence);
        if let true = should_trace {
            error.log_error_code_occurence(
                config.get_source_place_type(),
                config.get_log_type(),
                config.get_error_color_bold(),
            );
        }
        error
    }
}
