pub trait GetIsWarningHighPrintsEnabledWrapper<T> {
    fn get_is_warning_high_prints_enabled_wrapper(&self) -> bool;
}
impl<T> GetIsWarningHighPrintsEnabledWrapper<Self> for T
where
    Self: crate::traits::fields::GetIsWarningHighPrintsEnabled 
    + crate::traits::fields::GetIsPrintsEnabled,
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
    Self: crate::traits::fields::GetIsWarningLowPrintsEnabled 
    + crate::traits::fields::GetIsPrintsEnabled,
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
    Self: crate::traits::fields::GetIsSuccessPrintsEnabled 
    + crate::traits::fields::GetIsPrintsEnabled,
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
    Self: crate::traits::fields::GetIsPartialSuccessPrintsEnabled 
    + crate::traits::fields::GetIsPrintsEnabled,
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
    Self: crate::traits::fields::GetIsCleaningWarningLogsDirectoryEnabled 
    + crate::traits::fields::GetIsPrintsEnabled,
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
    Self: crate::traits::fields::GetIsTimeMeasurementPrintsEnabled 
    + crate::traits::fields::GetIsPrintsEnabled,
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
    Self: crate::traits::fields::GetIsInfoPrintsEnabled 
    + crate::traits::fields::GetIsPrintsEnabled,
{
    fn get_is_info_prints_enabled_wrapper(&self) -> bool {
        *self.get_is_prints_enabled() && *self.get_is_info_prints_enabled()
    }
}

// use crate::config_mods::print_type::PrintType;

// use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
// use crate::traits::print_type::PrintType;

// impl ProviderKind {
//     pub fn is_prints_for_print_type_enabled(&self, pt: &PrintType) -> bool {
//         match *pt {
//             crate::config_mods::print_type::PrintType::WarningHigh => pt.is_prints_enabled() && self.is_error_prints_enabled(),
//             crate::config_mods::print_type::PrintType::WarningHigh => {
//                 pt.is_prints_enabled() && self.is_warning_high_prints_enabled()
//             }
//             crate::config_mods::print_type::PrintType::WarningLow => pt.is_prints_enabled() && self.is_warning_low_prints_enabled(),
//             crate::config_mods::print_type::PrintType::Success => pt.is_prints_enabled() && self.is_success_prints_enabled(),
//             crate::config_mods::print_type::PrintType::PartialSuccess => {
//                 pt.is_prints_enabled() && self.is_partial_success_prints_enabled()
//             }
//             crate::config_mods::print_type::PrintType::TimeMeasurement => {
//                 pt.is_prints_enabled() && self.is_time_measurement_prints_enabled()
//             }
//             crate::config_mods::print_type::PrintType::Info => pt.is_prints_enabled() && self.is_info_prints_enabled(),
//             PrintType::WarningHigh => todo!(),
//             PrintType::WarningLow => todo!(),
//             PrintType::Success => todo!(),
//             PrintType::PartialSuccess => todo!(),
//             PrintType::Cleaning => todo!(),
//             PrintType::TimeMeasurement => todo!(),
//             PrintType::Info => todo!(),
//         }
//     }
// }
