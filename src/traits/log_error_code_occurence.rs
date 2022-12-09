use crate::traits::code_occurence_methods::CodeOccurenceMethods;
use crate::traits::code_occurence_methods::CodeOccurenceNewWithAddition;

pub trait LogErrorCodeOccurence<ConfigGeneric, ErrorColorBoldGeneric> {
    fn log_error_code_occurence(&self, config: ConfigGeneric);
}
impl<SelfGeneric, ConfigGeneric, ErrorColorBoldGeneric>
    LogErrorCodeOccurence<ConfigGeneric, ErrorColorBoldGeneric> for SelfGeneric
where
    SelfGeneric:
        crate::traits::get_code_occurence::GetCodeOccurence + crate::traits::get_source::GetSource,
    ConfigGeneric: crate::config_mods::traits::fields::GetSourcePlaceType
        + crate::config_mods::traits::fields::GetLogType
        + crate::traits::get_color::ErrorColorBold<ErrorColorBoldGeneric>,
{
    fn log_error_code_occurence(&self, config: ConfigGeneric) {
        self.get_code_occurence().log_code_occurence(
            self.get_source(),
            config.get_source_place_type(),
            config.get_log_type(),
            config.get_error_color_bold(),
        );
    }
}
