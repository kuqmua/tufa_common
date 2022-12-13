use crate::traits::fields::GetCleaningBlue;
use crate::traits::fields::GetCleaningGreen;
use crate::traits::fields::GetCleaningRed;
use crate::traits::fields::GetErrorBlue;
use crate::traits::fields::GetErrorGreen;
use crate::traits::fields::GetErrorRed;
use crate::traits::fields::GetInfoBlue;
use crate::traits::fields::GetInfoGreen;
use crate::traits::fields::GetInfoRed;
use crate::traits::fields::GetPartialSuccessBlue;
use crate::traits::fields::GetPartialSuccessGreen;
use crate::traits::fields::GetPartialSuccessRed;
use crate::traits::fields::GetSuccessBlue;
use crate::traits::fields::GetSuccessGreen;
use crate::traits::fields::GetSuccessRed;
use crate::traits::fields::GetTimeMeasurementBlue;
use crate::traits::fields::GetTimeMeasurementGreen;
use crate::traits::fields::GetTimeMeasurementRed;
use crate::traits::fields::GetWarningHighBlue;
use crate::traits::fields::GetWarningHighGreen;
use crate::traits::fields::GetWarningHighRed;
use crate::traits::fields::GetWarningLowBlue;
use crate::traits::fields::GetWarningLowGreen;
use crate::traits::fields::GetWarningLowRed;
use ansi_term::Colour::RGB;

pub trait ErrorColor<T> {
    fn get_error_color(&self) -> ansi_term::Colour;
}

impl<T> ErrorColor<Self> for T
where
    Self: GetErrorBlue + GetErrorGreen + GetErrorRed,
{
    fn get_error_color(&self) -> ansi_term::Colour {
        RGB(
            *self.get_error_red(),
            *self.get_error_green(),
            *self.get_error_blue(),
        )
    }
}

pub trait ErrorColorBold<T> {
    fn get_error_color_bold(&self) -> ansi_term::Style;
}

impl<T> ErrorColorBold<Self> for T
where
    Self: ErrorColor<T>,
{
    fn get_error_color_bold(&self) -> ansi_term::Style {
        self.get_error_color().bold()
    }
}

pub trait WarningHighColor<T> {
    fn get_warning_high_color(&self) -> ansi_term::Colour;
}

impl<T> WarningHighColor<Self> for T
where
    Self: GetWarningHighBlue + GetWarningHighGreen + GetWarningHighRed,
{
    fn get_warning_high_color(&self) -> ansi_term::Colour {
        RGB(
            *self.get_warning_high_red(),
            *self.get_warning_high_green(),
            *self.get_warning_high_blue(),
        )
    }
}

pub trait WarningHighColorBold<T> {
    fn get_warning_high_color_bold(&self) -> ansi_term::Style;
}

impl<T> WarningHighColorBold<Self> for T
where
    Self: WarningHighColor<T>,
{
    fn get_warning_high_color_bold(&self) -> ansi_term::Style {
        self.get_warning_high_color().bold()
    }
}

pub trait WarningLowColor<T> {
    fn get_warning_low_color(&self) -> ansi_term::Colour;
}

impl<T> WarningLowColor<Self> for T
where
    Self: GetWarningLowBlue + GetWarningLowGreen + GetWarningLowRed,
{
    fn get_warning_low_color(&self) -> ansi_term::Colour {
        RGB(
            *self.get_warning_low_red(),
            *self.get_warning_low_green(),
            *self.get_warning_low_blue(),
        )
    }
}

pub trait WarningLowColorBold<T> {
    fn get_warning_low_color_bold(&self) -> ansi_term::Style;
}

impl<T> WarningLowColorBold<Self> for T
where
    Self: WarningLowColor<T>,
{
    fn get_warning_low_color_bold(&self) -> ansi_term::Style {
        self.get_warning_low_color().bold()
    }
}

pub trait SuccessColor<T> {
    fn get_success_color(&self) -> ansi_term::Colour;
}

impl<T> SuccessColor<Self> for T
where
    Self: GetSuccessBlue + GetSuccessGreen + GetSuccessRed,
{
    fn get_success_color(&self) -> ansi_term::Colour {
        RGB(
            *self.get_success_red(),
            *self.get_success_green(),
            *self.get_success_blue(),
        )
    }
}

pub trait SuccessColorBold<T> {
    fn get_success_color_bold(&self) -> ansi_term::Style;
}

impl<T> SuccessColorBold<Self> for T
where
    Self: SuccessColor<T>,
{
    fn get_success_color_bold(&self) -> ansi_term::Style {
        self.get_success_color().bold()
    }
}

pub trait PartialSuccessColor<T> {
    fn get_partial_success_color(&self) -> ansi_term::Colour;
}

impl<T> PartialSuccessColor<Self> for T
where
    Self: GetPartialSuccessBlue + GetPartialSuccessGreen + GetPartialSuccessRed,
{
    fn get_partial_success_color(&self) -> ansi_term::Colour {
        RGB(
            *self.get_partial_success_red(),
            *self.get_partial_success_green(),
            *self.get_partial_success_blue(),
        )
    }
}

pub trait PartialSuccessColorBold<T> {
    fn get_partial_success_color_bold(&self) -> ansi_term::Style;
}

impl<T> PartialSuccessColorBold<Self> for T
where
    Self: PartialSuccessColor<T>,
{
    fn get_partial_success_color_bold(&self) -> ansi_term::Style {
        self.get_partial_success_color().bold()
    }
}

pub trait CleaningColor<T> {
    fn get_cleaning_color(&self) -> ansi_term::Colour;
}

impl<T> CleaningColor<Self> for T
where
    Self: GetCleaningBlue + GetCleaningGreen + GetCleaningRed,
{
    fn get_cleaning_color(&self) -> ansi_term::Colour {
        RGB(
            *self.get_cleaning_red(),
            *self.get_cleaning_green(),
            *self.get_cleaning_blue(),
        )
    }
}

pub trait CleaningColorBold<T> {
    fn get_cleaning_color_bold(&self) -> ansi_term::Style;
}

impl<T> CleaningColorBold<Self> for T
where
    Self: CleaningColor<T>,
{
    fn get_cleaning_color_bold(&self) -> ansi_term::Style {
        self.get_cleaning_color().bold()
    }
}

pub trait TimeMeasurementColor<T> {
    fn get_time_measurement_color(&self) -> ansi_term::Colour;
}

impl<T> TimeMeasurementColor<Self> for T
where
    Self: GetTimeMeasurementBlue + GetTimeMeasurementGreen + GetTimeMeasurementRed,
{
    fn get_time_measurement_color(&self) -> ansi_term::Colour {
        RGB(
            *self.get_time_measurement_red(),
            *self.get_time_measurement_green(),
            *self.get_time_measurement_blue(),
        )
    }
}

pub trait TimeMeasurementColorBold<T> {
    fn get_time_measurement_color_bold(&self) -> ansi_term::Style;
}

impl<T> TimeMeasurementColorBold<Self> for T
where
    Self: TimeMeasurementColor<T>,
{
    fn get_time_measurement_color_bold(&self) -> ansi_term::Style {
        self.get_time_measurement_color().bold()
    }
}

pub trait InfoColor<T> {
    fn get_info_color(&self) -> ansi_term::Colour;
}

impl<T> InfoColor<Self> for T
where
    Self: GetInfoBlue + GetInfoGreen + GetInfoRed,
{
    fn get_info_color(&self) -> ansi_term::Colour {
        RGB(
            *self.get_info_red(),
            *self.get_info_green(),
            *self.get_info_blue(),
        )
    }
}

pub trait InfoColorBold<T> {
    fn get_info_color_bold(&self) -> ansi_term::Style;
}

impl<T> InfoColorBold<Self> for T
where
    Self: InfoColor<T>,
{
    fn get_info_color_bold(&self) -> ansi_term::Style {
        self.get_info_color().bold()
    }
}
