use crate::traits::code_occurence_methods::CodeOccurenceMethods;
use crate::traits::get_code_occurence::GetCodeOccurence;

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
    T: GetCodeOccurence + crate::traits::get_source::GetSource,
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

// pub trait LogErrorCodeOccurenceTest<SelfGeneric> {
//     fn log_error_code_occurence_test(
//         &self,
//         config: SelfGeneric,
//     );
// }
// //SourcePlaceTypeGeneric, GetLogTypeGeneric, ErrorColorBoldGeneric
// impl<SelfGeneric, ErrorColorBoldGeneric> LogErrorCodeOccurenceTest<ErrorColorBoldGeneric>
//     for SelfGeneric
// where
//     SelfGeneric: GetCodeOccurence
//         + crate::traits::get_source::GetSource
//         + crate::config_mods::traits::fields::GetSourcePlaceType
//         + crate::config_mods::traits::fields::GetLogType
//         + crate::traits::get_color::ErrorColorBold<ErrorColorBoldGeneric>,
// {
//     fn log_error_code_occurence_test(
//         &self,
//         source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
//         log_type: &crate::config_mods::log_type::LogType,
//         style: ansi_term::Style,
//     ) {
//         self.get_code_occurence().log_code_occurence(
//             self.get_source(),
//             source_place_type,
//             log_type,
//             style,
//         );
//     }
// }
