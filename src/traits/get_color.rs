use crate::config_mods::traits::fields::GetCleaningBlue;
use crate::config_mods::traits::fields::GetCleaningGreen;
use crate::config_mods::traits::fields::GetCleaningRed;
use crate::config_mods::traits::fields::GetErrorBlue;
use crate::config_mods::traits::fields::GetErrorGreen;
use crate::config_mods::traits::fields::GetErrorRed;
use crate::config_mods::traits::fields::GetInfoBlue;
use crate::config_mods::traits::fields::GetInfoGreen;
use crate::config_mods::traits::fields::GetInfoRed;
use crate::config_mods::traits::fields::GetPartialSuccessBlue;
use crate::config_mods::traits::fields::GetPartialSuccessGreen;
use crate::config_mods::traits::fields::GetPartialSuccessRed;
use crate::config_mods::traits::fields::GetSuccessBlue;
use crate::config_mods::traits::fields::GetSuccessGreen;
use crate::config_mods::traits::fields::GetSuccessRed;
use crate::config_mods::traits::fields::GetTimeMeasurementBlue;
use crate::config_mods::traits::fields::GetTimeMeasurementGreen;
use crate::config_mods::traits::fields::GetTimeMeasurementRed;
use crate::config_mods::traits::fields::GetWarningHighBlue;
use crate::config_mods::traits::fields::GetWarningHighGreen;
use crate::config_mods::traits::fields::GetWarningHighRed;
use crate::config_mods::traits::fields::GetWarningLowBlue;
use crate::config_mods::traits::fields::GetWarningLowGreen;
use crate::config_mods::traits::fields::GetWarningLowRed;
use ansi_term::Colour::RGB;

pub trait ErrorColor<T> {
    fn get_error_color(&self) -> ansi_term::Style;
}

impl<T> ErrorColor<Self> for T
where
    Self: GetErrorBlue + GetErrorGreen + GetErrorRed,
{
    fn get_error_color(&self) -> ansi_term::Style {
        RGB(
            *self.get_error_red(),
            *self.get_error_green(),
            *self.get_error_blue(),
        )
        .bold()
    }
}

pub trait WarningHighColor<T> {
    fn get_warning_high_color(&self) -> ansi_term::Style;
}

impl<T> WarningHighColor<Self> for T
where
    Self: GetWarningHighBlue + GetWarningHighGreen + GetWarningHighRed,
{
    fn get_warning_high_color(&self) -> ansi_term::Style {
        RGB(
            *self.get_warning_high_red(),
            *self.get_warning_high_green(),
            *self.get_warning_high_blue(),
        )
        .bold()
    }
}

pub trait WarningLowColor<T> {
    fn get_warning_low_color(&self) -> ansi_term::Style;
}

impl<T> WarningLowColor<Self> for T
where
    Self: GetWarningLowBlue + GetWarningLowGreen + GetWarningLowRed,
{
    fn get_warning_low_color(&self) -> ansi_term::Style {
        RGB(
            *self.get_warning_low_red(),
            *self.get_warning_low_green(),
            *self.get_warning_low_blue(),
        )
        .bold()
    }
}

pub trait SuccessColor<T> {
    fn get_success_color(&self) -> ansi_term::Style;
}

impl<T> SuccessColor<Self> for T
where
    Self: GetSuccessBlue + GetSuccessGreen + GetSuccessRed,
{
    fn get_success_color(&self) -> ansi_term::Style {
        RGB(
            *self.get_success_red(),
            *self.get_success_green(),
            *self.get_success_blue(),
        )
        .bold()
    }
}

pub trait PartialSuccessColor<T> {
    fn get_partial_success_color(&self) -> ansi_term::Style;
}

impl<T> PartialSuccessColor<Self> for T
where
    Self: GetPartialSuccessBlue + GetPartialSuccessGreen + GetPartialSuccessRed,
{
    fn get_partial_success_color(&self) -> ansi_term::Style {
        RGB(
            *self.get_partial_success_red(),
            *self.get_partial_success_green(),
            *self.get_partial_success_blue(),
        )
        .bold()
    }
}

pub trait CleaningColor<T> {
    fn get_cleaning_color(&self) -> ansi_term::Style;
}

impl<T> CleaningColor<Self> for T
where
    Self: GetCleaningBlue + GetCleaningGreen + GetCleaningRed,
{
    fn get_cleaning_color(&self) -> ansi_term::Style {
        RGB(
            *self.get_cleaning_red(),
            *self.get_cleaning_green(),
            *self.get_cleaning_blue(),
        )
        .bold()
    }
}

pub trait TimeMeasurementColor<T> {
    fn get_time_measurement_color(&self) -> ansi_term::Style;
}

impl<T> TimeMeasurementColor<Self> for T
where
    Self: GetTimeMeasurementBlue + GetTimeMeasurementGreen + GetTimeMeasurementRed,
{
    fn get_time_measurement_color(&self) -> ansi_term::Style {
        RGB(
            *self.get_time_measurement_red(),
            *self.get_time_measurement_green(),
            *self.get_time_measurement_blue(),
        )
        .bold()
    }
}

pub trait InfoColor<T> {
    fn get_info_color(&self) -> ansi_term::Style;
}

impl<T> InfoColor<Self> for T
where
    Self: GetInfoBlue + GetInfoGreen + GetInfoRed,
{
    fn get_info_color(&self) -> ansi_term::Style {
        RGB(
            *self.get_info_red(),
            *self.get_info_green(),
            *self.get_info_blue(),
        )
        .bold()
    }
}
