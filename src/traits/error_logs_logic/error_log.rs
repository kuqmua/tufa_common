pub trait ErrorLog<'a, ConfigGeneric> {
    fn error_log(&self, config: &ConfigGeneric);
}

impl<'a, SelfGeneric, ConfigGeneric>
    crate::traits::error_logs_logic::error_log::ErrorLog<'a, ConfigGeneric> for SelfGeneric
where
    ConfigGeneric: crate::traits::get_color::ErrorColorBold
        + crate::traits::fields::GetServerPort
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::fields::GetServerIp,
    SelfGeneric: crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfig<
        'a,
        ConfigGeneric,
    >,
{
    fn error_log(&self, config: &ConfigGeneric) {
        eprintln!(
            "{}",
            config
                .get_error_color_bold()
                .paint(self.to_string_with_config(config))
        );
    }
}
