pub trait ErrorLogLifetime<'a, ConfigGeneric> {
    fn error_log_lifetime(&self, config: &ConfigGeneric);
}

impl<'a, SelfGeneric, ConfigGeneric>
    crate::traits::error_logs_logic::error_log::ErrorLogLifetime<'a, ConfigGeneric> for SelfGeneric
where
    ConfigGeneric: crate::traits::get_color::ErrorColorBold
        + crate::traits::fields::GetServerPort
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::fields::GetServerIp,
    SelfGeneric: crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    >,
{
    fn error_log_lifetime(&self, config: &ConfigGeneric) {
        eprintln!(
            "{}",
            config
                .get_error_color_bold()
                .paint(self.to_string_with_config_lifetime(config))
        );
    }
}

pub trait ErrorLogLifetimeWithDeserialize<'a, ConfigGeneric> {
    fn error_log_lifetime_with_deserialize(&self, config: &ConfigGeneric);
}

impl<'a, SelfGeneric, ConfigGeneric>
    crate::traits::error_logs_logic::error_log::ErrorLogLifetimeWithDeserialize<'a, ConfigGeneric> for SelfGeneric
where
    ConfigGeneric: crate::traits::get_color::ErrorColorBold
        + crate::traits::fields::GetServerPort
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::fields::GetServerIp,
    SelfGeneric: crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetimeWithDeserialize<
        'a,
        ConfigGeneric,
    >,
{
    fn error_log_lifetime_with_deserialize(&self, config: &ConfigGeneric) {
        eprintln!(
            "{}",
            config
                .get_error_color_bold()
                .paint(self.to_string_with_config_lifetime_with_deserialize(config))
        );
    }
}
