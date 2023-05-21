pub trait ToStringWithConfig<'a, ConfigGeneric> {
    fn to_string_with_config(
        &self,
        config: &ConfigGeneric,
    ) -> String;
}

impl<'a, SelfGeneric, ConfigGeneric>
    ToStringWithConfig<'a, ConfigGeneric> for SelfGeneric
where
    SelfGeneric:
        crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
                'a,
                ConfigGeneric,
            > + crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>,
    ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
        + crate::common::config::config_fields::GetTimezone,
{
    fn to_string_with_config(
        &self,
        config: &ConfigGeneric,
    ) -> String {
        use crate::traits::error_logs_logic::code_occurence_prepare_for_log::CodeOccurencePrepareForLogWithConfig;
        crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_with_config(config),
            self.get_code_occurence()
                .code_occurence_prepare_for_log_with_config(config),
        )
    }
}
