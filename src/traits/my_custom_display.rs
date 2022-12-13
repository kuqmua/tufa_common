use crate::traits::code_occurence_methods::CodeOccurenceToString;
use crate::traits::code_occurence_methods::CodeOccurenceWithSourceToString;
use crate::traits::get_code_occurence::GetCodeOccurence;
use crate::traits::get_source::GetSource;
use crate::traits::separator_symbol::SeparatorSymbol;

pub trait DisplayError<ConfigGeneric, ErrorColorBoldGeneric> {
    fn display_error(&self, config_generic: &ConfigGeneric) -> String;
}

impl<ConfigGeneric, ErrorColorBoldGeneric, ReturnSelfGeneric>
    DisplayError<ConfigGeneric, ErrorColorBoldGeneric> for ReturnSelfGeneric
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetLogType
        + crate::traits::get_color::ErrorColorBold<ErrorColorBoldGeneric>,
    ReturnSelfGeneric:
        crate::traits::get_code_occurence::GetCodeOccurence + crate::traits::get_source::GetSource,
{
    fn display_error(&self, config_generic: &ConfigGeneric) -> String {
        format!(
            "{}{}{}",
            self.get_source(),
            config_generic.get_log_type().symbol(),
            self.get_code_occurence()
                .code_occurence_to_string(config_generic)
        )
    }
}

pub trait ErrorCodeOccurenceToString<ConfigGeneric, ErrorColorBoldGeneric, SourceGeneric> {
    fn error_code_occurence_to_string(
        &self,
        source_generic: &SourceGeneric,
        config_generic: &ConfigGeneric,
    ) -> String;
}

impl<ConfigGeneric, ErrorColorBoldGeneric, SourceGeneric, ReturnSelfGeneric>
    ErrorCodeOccurenceToString<ConfigGeneric, ErrorColorBoldGeneric, SourceGeneric>
    for ReturnSelfGeneric
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetLogType
        + crate::traits::get_color::ErrorColorBold<ErrorColorBoldGeneric>,
    SourceGeneric: crate::traits::get_source::GetSource,
    ReturnSelfGeneric: crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn error_code_occurence_to_string(
        &self,
        source_generic: &SourceGeneric,
        config_generic: &ConfigGeneric,
    ) -> String {
        self.get_code_occurence()
            .code_occurence_with_source_to_string(source_generic, config_generic)
    }
}
