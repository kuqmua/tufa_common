pub trait ErrorLog<ConfigGeneric> {
    fn error_log(&self, config: &ConfigGeneric);
}

impl<SelfGeneric, ConfigGeneric> crate::traits::error_log::ErrorLog<ConfigGeneric> for SelfGeneric
where
    ConfigGeneric: crate::traits::get_color::ErrorColorBold
        + crate::traits::fields::GetServerPort
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::fields::GetServerIp,
    SelfGeneric: crate::traits::error_display::ToStringHandle<ConfigGeneric>,
{
    fn error_log(&self, config: &ConfigGeneric) {
        eprintln!(
            "{}",
            config
                .get_error_color_bold()
                .paint(self.to_string_handle(config))
        );
    }
}
