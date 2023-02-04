pub trait ToStringWithConfig<ConfigGeneric> {
    fn to_string_with_config(&self, config: &ConfigGeneric) -> String;
}

impl<SelfGeneric, ConfigGeneric> ToStringWithConfig<ConfigGeneric> for SelfGeneric
where
    SelfGeneric:
        crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
                ConfigGeneric,
            > + crate::traits::get_code_occurence::GetCodeOccurence,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
        crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_with_config(config),
            self.get_code_occurence().to_string_with_config(config),
        )
    }
}
