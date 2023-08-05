pub trait ToStringWithConfig<'a, ConfigGeneric> {
    fn to_string_with_config(&self, config: &ConfigGeneric) -> String;
}

impl<'a, SelfGeneric, ConfigGeneric> ToStringWithConfig<'a, ConfigGeneric> for SelfGeneric
where
    SelfGeneric:
        crate::common::error_logs_logic::source_to_string_with_config_second::SourceToStringWithConfigSecond<'a> + crate::common::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>,
    ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
        + crate::common::config::config_fields::GetTimezone,
{
    fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
        crate::common::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_with_config_second(config),
            crate::common::error_logs_logic::code_occurence_prepare_for_log::CodeOccurencePrepareForLogWithConfig::code_occurence_prepare_for_log_with_config(
                self.get_code_occurence(),
                config
            )
        )
    }
}
