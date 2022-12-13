pub trait ErrorColor {
    fn get_error_color(&self) -> ansi_term::Colour;
}

impl<SelfGeneric> ErrorColor for SelfGeneric
where
    Self: crate::traits::fields::GetErrorBlue
        + crate::traits::fields::GetErrorGreen
        + crate::traits::fields::GetErrorRed,
{
    fn get_error_color(&self) -> ansi_term::Colour {
        ansi_term::Colour::RGB(
            *self.get_error_red(),
            *self.get_error_green(),
            *self.get_error_blue(),
        )
    }
}

pub trait ErrorColorBold {
    fn get_error_color_bold(&self) -> ansi_term::Style;
}

impl<SelfGeneric> ErrorColorBold for SelfGeneric
where
    Self: ErrorColor,
{
    fn get_error_color_bold(&self) -> ansi_term::Style {
        self.get_error_color().bold()
    }
}

pub trait WarningHighColor {
    fn get_warning_high_color(&self) -> ansi_term::Colour;
}

impl<SelfGeneric> WarningHighColor for SelfGeneric
where
    Self: crate::traits::fields::GetWarningHighBlue
        + crate::traits::fields::GetWarningHighGreen
        + crate::traits::fields::GetWarningHighRed,
{
    fn get_warning_high_color(&self) -> ansi_term::Colour {
        ansi_term::Colour::RGB(
            *self.get_warning_high_red(),
            *self.get_warning_high_green(),
            *self.get_warning_high_blue(),
        )
    }
}

pub trait WarningHighColorBold {
    fn get_warning_high_color_bold(&self) -> ansi_term::Style;
}

impl<SelfGeneric> WarningHighColorBold for SelfGeneric
where
    Self: WarningHighColor,
{
    fn get_warning_high_color_bold(&self) -> ansi_term::Style {
        self.get_warning_high_color().bold()
    }
}

pub trait WarningLowColor {
    fn get_warning_low_color(&self) -> ansi_term::Colour;
}

impl<SelfGeneric> WarningLowColor for SelfGeneric
where
    Self: crate::traits::fields::GetWarningLowBlue
        + crate::traits::fields::GetWarningLowGreen
        + crate::traits::fields::GetWarningLowRed,
{
    fn get_warning_low_color(&self) -> ansi_term::Colour {
        ansi_term::Colour::RGB(
            *self.get_warning_low_red(),
            *self.get_warning_low_green(),
            *self.get_warning_low_blue(),
        )
    }
}

pub trait WarningLowColorBold {
    fn get_warning_low_color_bold(&self) -> ansi_term::Style;
}

impl<SelfGeneric> WarningLowColorBold for SelfGeneric
where
    Self: WarningLowColor,
{
    fn get_warning_low_color_bold(&self) -> ansi_term::Style {
        self.get_warning_low_color().bold()
    }
}

pub trait SuccessColor {
    fn get_success_color(&self) -> ansi_term::Colour;
}

impl<SelfGeneric> SuccessColor for SelfGeneric
where
    Self: crate::traits::fields::GetSuccessBlue
        + crate::traits::fields::GetSuccessGreen
        + crate::traits::fields::GetSuccessRed,
{
    fn get_success_color(&self) -> ansi_term::Colour {
        ansi_term::Colour::RGB(
            *self.get_success_red(),
            *self.get_success_green(),
            *self.get_success_blue(),
        )
    }
}

pub trait SuccessColorBold {
    fn get_success_color_bold(&self) -> ansi_term::Style;
}

impl<SelfGeneric> SuccessColorBold for SelfGeneric
where
    Self: SuccessColor,
{
    fn get_success_color_bold(&self) -> ansi_term::Style {
        self.get_success_color().bold()
    }
}

pub trait PartialSuccessColor {
    fn get_partial_success_color(&self) -> ansi_term::Colour;
}

impl<SelfGeneric> PartialSuccessColor for SelfGeneric
where
    Self: crate::traits::fields::GetPartialSuccessBlue
        + crate::traits::fields::GetPartialSuccessGreen
        + crate::traits::fields::GetPartialSuccessRed,
{
    fn get_partial_success_color(&self) -> ansi_term::Colour {
        ansi_term::Colour::RGB(
            *self.get_partial_success_red(),
            *self.get_partial_success_green(),
            *self.get_partial_success_blue(),
        )
    }
}

pub trait PartialSuccessColorBold {
    fn get_partial_success_color_bold(&self) -> ansi_term::Style;
}

impl<SelfGeneric> PartialSuccessColorBold for SelfGeneric
where
    Self: PartialSuccessColor,
{
    fn get_partial_success_color_bold(&self) -> ansi_term::Style {
        self.get_partial_success_color().bold()
    }
}

pub trait CleaningColor {
    fn get_cleaning_color(&self) -> ansi_term::Colour;
}

impl<SelfGeneric> CleaningColor for SelfGeneric
where
    Self: crate::traits::fields::GetCleaningBlue
        + crate::traits::fields::GetCleaningGreen
        + crate::traits::fields::GetCleaningRed,
{
    fn get_cleaning_color(&self) -> ansi_term::Colour {
        ansi_term::Colour::RGB(
            *self.get_cleaning_red(),
            *self.get_cleaning_green(),
            *self.get_cleaning_blue(),
        )
    }
}

pub trait CleaningColorBold {
    fn get_cleaning_color_bold(&self) -> ansi_term::Style;
}

impl<SelfGeneric> CleaningColorBold for SelfGeneric
where
    Self: CleaningColor,
{
    fn get_cleaning_color_bold(&self) -> ansi_term::Style {
        self.get_cleaning_color().bold()
    }
}

pub trait TimeMeasurementColor {
    fn get_time_measurement_color(&self) -> ansi_term::Colour;
}

impl<SelfGeneric> TimeMeasurementColor for SelfGeneric
where
    Self: crate::traits::fields::GetTimeMeasurementBlue
        + crate::traits::fields::GetTimeMeasurementGreen
        + crate::traits::fields::GetTimeMeasurementRed,
{
    fn get_time_measurement_color(&self) -> ansi_term::Colour {
        ansi_term::Colour::RGB(
            *self.get_time_measurement_red(),
            *self.get_time_measurement_green(),
            *self.get_time_measurement_blue(),
        )
    }
}

pub trait TimeMeasurementColorBold {
    fn get_time_measurement_color_bold(&self) -> ansi_term::Style;
}

impl<SelfGeneric> TimeMeasurementColorBold for SelfGeneric
where
    Self: TimeMeasurementColor,
{
    fn get_time_measurement_color_bold(&self) -> ansi_term::Style {
        self.get_time_measurement_color().bold()
    }
}

pub trait InfoColor {
    fn get_info_color(&self) -> ansi_term::Colour;
}

impl<SelfGeneric> InfoColor for SelfGeneric
where
    Self: crate::traits::fields::GetInfoBlue
        + crate::traits::fields::GetInfoGreen
        + crate::traits::fields::GetInfoRed,
{
    fn get_info_color(&self) -> ansi_term::Colour {
        ansi_term::Colour::RGB(
            *self.get_info_red(),
            *self.get_info_green(),
            *self.get_info_blue(),
        )
    }
}

pub trait InfoColorBold {
    fn get_info_color_bold(&self) -> ansi_term::Style;
}

impl<SelfGeneric> InfoColorBold for SelfGeneric
where
    Self: InfoColor,
{
    fn get_info_color_bold(&self) -> ansi_term::Style {
        self.get_info_color().bold()
    }
}
