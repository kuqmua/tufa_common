use crate::traits::code_occurence_methods::CodeOccurenceLog;

pub trait LogErrorCodeOccurence<ConfigGeneric> {
    fn log_error_code_occurence(&self, config: ConfigGeneric);
}
impl<SelfGeneric, ConfigGeneric> LogErrorCodeOccurence<ConfigGeneric> for SelfGeneric
where
    SelfGeneric:
        crate::traits::get_code_occurence::GetCodeOccurence + crate::traits::get_source::GetSource,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetLogType
        + crate::traits::get_color::ErrorColorBold,
{
    fn log_error_code_occurence(&self, config: ConfigGeneric) {
        self.get_code_occurence().log(
            // self.get_source(),
            self, config,
        );
    }
}
