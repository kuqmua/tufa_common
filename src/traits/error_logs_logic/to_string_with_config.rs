pub trait ToStringWithConfigLifetime<'a, ConfigGeneric> {
    fn to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String;
}

impl<'a, SelfGeneric, ConfigGeneric> ToStringWithConfigLifetime<'a, ConfigGeneric> for SelfGeneric
where
    SelfGeneric:
        crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfigLifetime<
                'a,
                ConfigGeneric,
            > + crate::traits::get_code_occurence::GetCodeOccurenceLifetime<'a>,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_with_config_lifetime(config),
            self.get_code_occurence_lifetime().to_string_with_config_lifetime(config),
        )
    }
}
