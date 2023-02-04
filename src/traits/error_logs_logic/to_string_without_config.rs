pub trait ToStringWithoutConfig {
    fn to_string_without_config(&self) -> String;
}

impl<SelfGeneric> ToStringWithoutConfig for SelfGeneric
where
    SelfGeneric:
        crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig
            + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn to_string_without_config(&self) -> String {
        crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_without_config(),
            self.get_code_occurence().to_string(),
        )
    }
}
