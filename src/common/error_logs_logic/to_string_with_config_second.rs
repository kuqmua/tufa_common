pub trait ToStringWithConfigSecond<'a> {
    fn to_string_with_config_second<ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
        + crate::common::config::config_fields::GetTimezone + ?Sized>(&self, config: &ConfigGeneric) -> String;
}

impl<'a, SelfGeneric> ToStringWithConfigSecond<'a> for SelfGeneric
where
    SelfGeneric:
        crate::common::error_logs_logic::source_to_string_with_config_second::SourceToStringWithConfigSecond<'a> 
        + crate::common::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>,
{
    fn to_string_with_config_second<ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
        + crate::common::config::config_fields::GetTimezone + ?Sized>(&self, config: &ConfigGeneric) -> String {
        crate::common::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_with_config_second(config),
            crate::common::error_logs_logic::code_occurence_prepare_for_log::CodeOccurencePrepareForLogWithConfigSecond::code_occurence_prepare_for_log_with_config_second(
                self.get_code_occurence(),
                config
            )
        )
    }
}