use crate::traits::code_occurence_methods::CodeOccurenceMethods;
use crate::traits::get_code_occurence::GetCodeOccurence;
use crate::traits::get_source::GetSource;

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
    T: GetCodeOccurence + GetSource,
{
    fn log_error_code_occurence(
        &self,
        source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
        log_type: &crate::config_mods::log_type::LogType,
        style: ansi_term::Style,
    ) {
        self.get_code_occurence().log_code_occurence(
            source_place_type,
            log_type,
            self.get_source(),
            style,
        );
    }
}

//
// use crate::traits::code_occurence_methods::CodeOccurenceMethods;
// use crate::traits::get_code_occurence::GetCodeOccurence;
// use crate::traits::get_color::ErrorColorBold;
// use crate::traits::get_source::GetSource;

// pub trait LogErrorCodeOccurence<T, D> {
//     fn log_error_code_occurence(
//         &self,
//         source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
//         log_type: crate::config_mods::log_type::LogType,
//         impl_error_color_bold: &impl ErrorColorBold<D>,
//     );
// }

// impl<T, D> LogErrorCodeOccurence<T, D> for T
// where
//     T: GetCodeOccurence + GetSource,
// {
//     fn log_error_code_occurence(
//         &self,
//         source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
//         log_type: crate::config_mods::log_type::LogType,
//         impl_error_color_bold: &impl ErrorColorBold<D>,
//     ) {
//         self.get_code_occurence().log_code_occurence(
//             source_place_type,
//             log_type,
//             self.get_source(),
//             impl_error_color_bold.get_error_color_bold(),
//         );
//     }
// }
