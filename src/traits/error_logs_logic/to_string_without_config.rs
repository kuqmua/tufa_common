pub trait ToStringWithoutConfigLifetime<'a> {
    fn to_string_without_config_lifetime(&self) -> String;
}

impl<'a, SelfGeneric> ToStringWithoutConfigLifetime<'a> for SelfGeneric
where
    SelfGeneric:
        crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<'a>
            + crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>,
{
    fn to_string_without_config_lifetime(&self) -> String {
        crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_without_config(),
            self.get_code_occurence().to_string(),
        )
    }
}
// //implemented coz you cant deserialize field into &'a GitInformation(not implememnted in serde)
pub trait ToStringWithoutConfigWithDeserialize<'a> {
    fn to_string_without_config_with_deserialize(&self) -> String;
}

impl<'a, SelfGeneric> ToStringWithoutConfigWithDeserialize<'a> for SelfGeneric
where
    SelfGeneric:
        crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<'a>
            + crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>,
{
    fn to_string_without_config_with_deserialize(&self) -> String {
        crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_without_config(),
            self.get_code_occurence_with_deserialize().to_string(),//todo - do .to_string() inside inner
        )
    }
}
