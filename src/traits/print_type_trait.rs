use crate::config_mods::config_struct::ConfigStruct;
use crate::config_mods::print_type::PrintType;
use crate::traits::get_color::CleaningColor;
use crate::traits::get_color::ErrorColor;
use crate::traits::get_color::InfoColor;
use crate::traits::get_color::PartialSuccessColor;
use crate::traits::get_color::SuccessColor;
use crate::traits::get_color::TimeMeasurementColor;
use crate::traits::get_color::WarningHighColor;
use crate::traits::get_color::WarningLowColor;
use crate::traits::is_enabled_prints::GetIsCleaningWarningLogsDirectoryEnabledWrapper;
use crate::traits::is_enabled_prints::GetIsInfoPrintsEnabledWrapper;
use crate::traits::is_enabled_prints::GetIsPartialSuccessPrintsEnabledWrapper;
use crate::traits::is_enabled_prints::GetIsSuccessPrintsEnabledWrapper;
use crate::traits::is_enabled_prints::GetIsTimeMeasurementPrintsEnabledWrapper;
use crate::traits::is_enabled_prints::GetIsWarningHighPrintsEnabledWrapper;
use crate::traits::is_enabled_prints::GetIsWarningLowPrintsEnabledWrapper;
use ansi_term::Colour;

pub trait PrintTypeTrait {
    fn is_prints_enabled(&self, print_type: &PrintType) -> bool;
    fn get_color(&self, print_type: &PrintType) -> Colour;
}

impl PrintTypeTrait for ConfigStruct {
    fn is_prints_enabled(&self, print_type: &PrintType) -> bool {
        match print_type {
            PrintType::WarningHigh => self.get_is_warning_high_prints_enabled_wrapper(),
            PrintType::WarningLow => self.get_is_warning_low_prints_enabled_wrapper(),
            PrintType::Success => self.get_is_success_prints_enabled_wrapper(),
            PrintType::PartialSuccess => self.get_is_partial_success_prints_enabled_wrapper(),
            PrintType::Cleaning => self.get_is_cleaning_warning_logs_directory_enabled_wrapper(),
            PrintType::TimeMeasurement => self.get_is_time_measurement_prints_enabled_wrapper(),
            PrintType::Info => self.get_is_info_prints_enabled_wrapper(),
        }
    }
    fn get_color(&self, print_type: &PrintType) -> ansi_term::Colour {
        match print_type {
            PrintType::WarningHigh => self.get_warning_high_color(),
            PrintType::WarningLow => self.get_warning_low_color(),
            PrintType::Success => self.get_success_color(),
            PrintType::PartialSuccess => self.get_partial_success_color(),
            PrintType::Cleaning => self.get_cleaning_color(),
            PrintType::TimeMeasurement => self.get_time_measurement_color(),
            PrintType::Info => self.get_info_color(),
        }
    }
}
