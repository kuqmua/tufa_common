pub trait PrintTypeMethods {
    fn is_prints_enabled(&self, print_type: &crate::config_mods::print_type::PrintType) -> bool;
    fn get_color(&self, print_type: &crate::config_mods::print_type::PrintType) -> ansi_term::Colour;
}

impl PrintTypeMethods for crate::config_mods::config_struct::ConfigStruct {
    fn is_prints_enabled(&self, print_type: &crate::config_mods::print_type::PrintType) -> bool {
        match print_type {
            crate::config_mods::print_type::PrintType::WarningHigh => {
                use crate::traits::is_enabled_prints::GetIsWarningHighPrintsEnabledWrapper;
                self.get_is_warning_high_prints_enabled_wrapper()
            },
            crate::config_mods::print_type::PrintType::WarningLow => {
                use crate::traits::is_enabled_prints::GetIsWarningLowPrintsEnabledWrapper;
                self.get_is_warning_low_prints_enabled_wrapper()
            },
            crate::config_mods::print_type::PrintType::Success => {
                use crate::traits::is_enabled_prints::GetIsSuccessPrintsEnabledWrapper;
                self.get_is_success_prints_enabled_wrapper()
            },
            crate::config_mods::print_type::PrintType::PartialSuccess => {
                use crate::traits::is_enabled_prints::GetIsPartialSuccessPrintsEnabledWrapper;
                self.get_is_partial_success_prints_enabled_wrapper()
            },
            crate::config_mods::print_type::PrintType::Cleaning => {
                use crate::traits::is_enabled_prints::GetIsCleaningWarningLogsDirectoryEnabledWrapper;
                self.get_is_cleaning_warning_logs_directory_enabled_wrapper()
            },
            crate::config_mods::print_type::PrintType::TimeMeasurement => {
                use crate::traits::is_enabled_prints::GetIsTimeMeasurementPrintsEnabledWrapper;
                self.get_is_time_measurement_prints_enabled_wrapper()
            },
            crate::config_mods::print_type::PrintType::Info => {
                use crate::traits::is_enabled_prints::GetIsInfoPrintsEnabledWrapper;
                self.get_is_info_prints_enabled_wrapper()
            },
        }
    }
    fn get_color(&self, print_type: &crate::config_mods::print_type::PrintType) -> ansi_term::Colour {
        match print_type {
            crate::config_mods::print_type::PrintType::WarningHigh => {
                use crate::traits::get_color::WarningHighColor;
                self.get_warning_high_color()
            },
            crate::config_mods::print_type::PrintType::WarningLow => {
                use crate::traits::get_color::WarningLowColor;
                self.get_warning_low_color()
            },
            crate::config_mods::print_type::PrintType::Success => {
                use crate::traits::get_color::SuccessColor;
                self.get_success_color()
            },
            crate::config_mods::print_type::PrintType::PartialSuccess => {
                use crate::traits::get_color::PartialSuccessColor;
                self.get_partial_success_color()
            },
            crate::config_mods::print_type::PrintType::Cleaning => {
                use crate::traits::get_color::CleaningColor;
                self.get_cleaning_color()
            },
            crate::config_mods::print_type::PrintType::TimeMeasurement => {
                use crate::traits::get_color::TimeMeasurementColor;
                self.get_time_measurement_color()
            },
            crate::config_mods::print_type::PrintType::Info => {
                use crate::traits::get_color::InfoColor;
                self.get_info_color()
            },
        }
    }
}
