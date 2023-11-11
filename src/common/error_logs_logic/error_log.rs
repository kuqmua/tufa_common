pub trait ErrorLog {
    fn error_log<
        ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
            + crate::common::config::config_fields::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    );
}

impl<'a, SelfGeneric> crate::common::error_logs_logic::error_log::ErrorLog for SelfGeneric
where
    SelfGeneric: crate::common::error_logs_logic::to_string_with_config::ToStringWithConfig<'a>,
{
    fn error_log<
        ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
            + crate::common::config::config_fields::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) {
        eprintln!("{}", self.to_string_with_config(config));
    }
}
