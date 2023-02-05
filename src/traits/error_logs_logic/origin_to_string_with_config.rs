// use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfig;

// pub trait OriginToStringWithConfig<ConfigGeneric> {
//     fn origin_to_string_with_config(&self, config: &ConfigGeneric) -> String;
// }

// impl<SelfGeneric, ConfigGeneric> OriginToStringWithConfig<ConfigGeneric> for SelfGeneric
// where
//     SelfGeneric:
//         crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig
//             + crate::traits::get_code_occurence::GetCodeOccurence,
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn origin_to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
//             self.source_to_string_without_config(),
//             self.get_code_occurence().to_string_with_config(config),
//         )
//     }
// }
//
use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime;

pub trait OriginToStringWithConfigLifetime<'a, ConfigGeneric> {
    fn origin_to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String;
}

impl<'a, SelfGeneric, ConfigGeneric> OriginToStringWithConfigLifetime<'a, ConfigGeneric> for SelfGeneric
where
    SelfGeneric:
        crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a>
            + crate::traits::get_code_occurence::GetCodeOccurenceLifetime<'a>,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn origin_to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_without_config_lifetime(),
            self.get_code_occurence_lifetime().to_string_with_config_lifetime(config),
        )
    }
}
