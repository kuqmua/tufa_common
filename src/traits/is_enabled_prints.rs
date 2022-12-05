use crate::config_mods::traits::fields::GetIsCleaningWarningLogsDirectoryEnabled;
use crate::config_mods::traits::fields::GetIsInfoPrintsEnabled;
use crate::config_mods::traits::fields::GetIsPartialSuccessPrintsEnabled;
use crate::config_mods::traits::fields::GetIsPrintsEnabled;
use crate::config_mods::traits::fields::GetIsSuccessPrintsEnabled;
use crate::config_mods::traits::fields::GetIsTimeMeasurementPrintsEnabled;
use crate::config_mods::traits::fields::GetIsWarningHighPrintsEnabled;
use crate::config_mods::traits::fields::GetIsWarningLowPrintsEnabled;

pub trait GetIsWarningHighPrintsEnabledWrapper<T> {
    fn get_is_warning_high_prints_enabled_wrapper(&self) -> bool;
}
impl<T> GetIsWarningHighPrintsEnabledWrapper<Self> for T
where
    Self: GetIsWarningHighPrintsEnabled + GetIsPrintsEnabled,
{
    fn get_is_warning_high_prints_enabled_wrapper(&self) -> bool {
        *self.get_is_prints_enabled() && *self.get_is_warning_high_prints_enabled()
    }
}
pub trait GetIsWarningLowPrintsEnabledWrapper<T> {
    fn get_is_warning_low_prints_enabled_wrapper(&self) -> bool;
}
impl<T> GetIsWarningLowPrintsEnabledWrapper<Self> for T
where
    Self: GetIsWarningLowPrintsEnabled + GetIsPrintsEnabled,
{
    fn get_is_warning_low_prints_enabled_wrapper(&self) -> bool {
        *self.get_is_prints_enabled() && *self.get_is_warning_low_prints_enabled()
    }
}
pub trait GetIsSuccessPrintsEnabledWrapper<T> {
    fn get_is_success_prints_enabled_wrapper(&self) -> bool;
}
impl<T> GetIsSuccessPrintsEnabledWrapper<Self> for T
where
    Self: GetIsSuccessPrintsEnabled + GetIsPrintsEnabled,
{
    fn get_is_success_prints_enabled_wrapper(&self) -> bool {
        *self.get_is_prints_enabled() && *self.get_is_success_prints_enabled()
    }
}
pub trait GetIsPartialSuccessPrintsEnabledWrapper<T> {
    fn get_is_partial_success_prints_enabled_wrapper(&self) -> bool;
}
impl<T> GetIsPartialSuccessPrintsEnabledWrapper<Self> for T
where
    Self: GetIsPartialSuccessPrintsEnabled + GetIsPrintsEnabled,
{
    fn get_is_partial_success_prints_enabled_wrapper(&self) -> bool {
        *self.get_is_prints_enabled() && *self.get_is_partial_success_prints_enabled()
    }
}
pub trait GetIsCleaningWarningLogsDirectoryEnabledWrapper<T> {
    fn get_is_cleaning_warning_logs_directory_enabled_wrapper(&self) -> bool;
}
impl<T> GetIsCleaningWarningLogsDirectoryEnabledWrapper<Self> for T
where
    Self: GetIsCleaningWarningLogsDirectoryEnabled + GetIsPrintsEnabled,
{
    fn get_is_cleaning_warning_logs_directory_enabled_wrapper(&self) -> bool {
        *self.get_is_prints_enabled() && *self.get_is_cleaning_warning_logs_directory_enabled()
    }
}
pub trait GetIsTimeMeasurementPrintsEnabledWrapper<T> {
    fn get_is_time_measurement_prints_enabled_wrapper(&self) -> bool;
}
impl<T> GetIsTimeMeasurementPrintsEnabledWrapper<Self> for T
where
    Self: GetIsTimeMeasurementPrintsEnabled + GetIsPrintsEnabled,
{
    fn get_is_time_measurement_prints_enabled_wrapper(&self) -> bool {
        *self.get_is_prints_enabled() && *self.get_is_time_measurement_prints_enabled()
    }
}
pub trait GetIsInfoPrintsEnabledWrapper<T> {
    fn get_is_info_prints_enabled_wrapper(&self) -> bool;
}
impl<T> GetIsInfoPrintsEnabledWrapper<Self> for T
where
    Self: GetIsInfoPrintsEnabled + GetIsPrintsEnabled,
{
    fn get_is_info_prints_enabled_wrapper(&self) -> bool {
        *self.get_is_prints_enabled() && *self.get_is_info_prints_enabled()
    }
}

// use tufa_common::config_mods::print_type::PrintType;
// use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
// use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfigTrait;
// use tufa_common::traits::print_type::PrintType;

// impl ProviderKind {
//     pub fn is_prints_for_print_type_enabled(&self, pt: &PrintType) -> bool {
//         match *pt {
//             tufa_common::config_mods::print_type::PrintType::WarningHigh => pt.is_prints_enabled() && self.is_error_prints_enabled(),
//             tufa_common::config_mods::print_type::PrintType::WarningHigh => {
//                 pt.is_prints_enabled() && self.is_warning_high_prints_enabled()
//             }
//             tufa_common::config_mods::print_type::PrintType::WarningLow => pt.is_prints_enabled() && self.is_warning_low_prints_enabled(),
//             tufa_common::config_mods::print_type::PrintType::Success => pt.is_prints_enabled() && self.is_success_prints_enabled(),
//             tufa_common::config_mods::print_type::PrintType::PartialSuccess => {
//                 pt.is_prints_enabled() && self.is_partial_success_prints_enabled()
//             }
//             tufa_common::config_mods::print_type::PrintType::TimeMeasurement => {
//                 pt.is_prints_enabled() && self.is_time_measurement_prints_enabled()
//             }
//             tufa_common::config_mods::print_type::PrintType::Info => pt.is_prints_enabled() && self.is_info_prints_enabled(),
//         }
//     }
// }
