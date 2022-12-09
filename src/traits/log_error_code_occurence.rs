// use crate::traits::code_occurence_methods::CodeOccurenceMethods;
// use crate::traits::get_code_occurence::GetCodeOccurence;

pub trait LogErrorCodeOccurence {
    fn log_error_code_occurence(
        &self,
        source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
        log_type: &crate::config_mods::log_type::LogType,
        style: ansi_term::Style,
    );
}

impl<T> LogErrorCodeOccurence for T
where
    T: crate::traits::get_code_occurence::GetCodeOccurence + crate::traits::get_source::GetSource,
{
    fn log_error_code_occurence(
        &self,
        source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
        log_type: &crate::config_mods::log_type::LogType,
        style: ansi_term::Style,
    ) {
        self.get_code_occurence().log_code_occurence(
            self.get_source(),
            source_place_type,
            log_type,
            style,
        );
    }
}

use crate::traits::code_occurence_methods::CodeOccurenceMethods;

pub trait LogErrorCodeOccurenceTest<ConfigGeneric, ErrorColorBoldGeneric> {
    fn log_error_code_occurence_test(&self, config: ConfigGeneric);
}
//SourcePlaceTypeGeneric, GetLogTypeGeneric, ErrorColorBoldGeneric
impl<SelfGeneric, ConfigGeneric, ErrorColorBoldGeneric>
    LogErrorCodeOccurenceTest<ConfigGeneric, ErrorColorBoldGeneric> for SelfGeneric
where
    SelfGeneric:
        crate::traits::get_code_occurence::GetCodeOccurence + crate::traits::get_source::GetSource,
    ConfigGeneric: crate::config_mods::traits::fields::GetSourcePlaceType
        + crate::config_mods::traits::fields::GetLogType
        + crate::traits::get_color::ErrorColorBold<ErrorColorBoldGeneric>,
{
    fn log_error_code_occurence_test(
        &self,
        config: ConfigGeneric,
        // source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
        // log_type: &crate::config_mods::log_type::LogType,
        // style: ansi_term::Style,
    ) {
        self.get_code_occurence().log_code_occurence(
            self.get_source(),
            config.get_source_place_type(),
            config.get_log_type(),
            config.get_error_color_bold(),
            // source_place_type,
            // log_type,
            // style,
        );
    }
}
