use crate::traits::code_occurence_methods::CodeOccurenceMethods;
use crate::traits::get_code_occurence::GetCodeOccurence;
use crate::traits::get_source::GetSource;

pub trait LogErrorCodeOccurence {
    fn log_error_code_occurence(
        &self,
        source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
        log_type: crate::config_mods::log_type::LogType,
        style: ansi_term::Style,
    );
}

impl<T> LogErrorCodeOccurence for T
where
    T: GetCodeOccurence + GetSource,
{
    fn log_error_code_occurence(
        &self,
        source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
        log_type: crate::config_mods::log_type::LogType,
        style: ansi_term::Style,
    ) {
        self.get_code_occurence().log_code_occurence(
            source_place_type,
            log_type,
            self.get_source(),
            style,
        )
    }
}
