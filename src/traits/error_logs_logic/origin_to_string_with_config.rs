use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfig;

pub trait OriginToStringWithConfig<ConfigGeneric> {
    fn origin_to_string_with_config(&self, config: &ConfigGeneric) -> String;
}

impl<SelfGeneric, ConfigGeneric> OriginToStringWithConfig<ConfigGeneric> for SelfGeneric
where
    SelfGeneric:
        crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig
            + crate::traits::get_code_occurence::GetCodeOccurence,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn origin_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_without_config(),
            self.get_code_occurence().to_string_with_config(config),
        )
    }
}
