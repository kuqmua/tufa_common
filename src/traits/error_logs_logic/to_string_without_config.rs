pub trait ToStringWithoutConfigLifetime<'a> {
    fn to_string_without_config_lifetime(&self) -> String;
}

impl<'a, SelfGeneric> ToStringWithoutConfigLifetime<'a> for SelfGeneric
where
    SelfGeneric:
        crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a>
            + crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceLifetime<'a>,
{
    fn to_string_without_config_lifetime(&self) -> String {
        crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_without_config_lifetime(),
            self.get_code_occurence_lifetime().to_string(),
        )
    }
}
// //implemented coz you cant deserialize field into &'a GitInformation(not implememnted in serde)
pub trait ToStringWithoutConfigLifetimeWithDeserialize<'a> {
    fn to_string_without_config_lifetime_with_deserialize(&self) -> String;
}

impl<'a, SelfGeneric> ToStringWithoutConfigLifetimeWithDeserialize<'a> for SelfGeneric
where
    SelfGeneric:
        crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a>
            + crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceLifetimeWithDeserialize<'a>,
{
    fn to_string_without_config_lifetime_with_deserialize(&self) -> String {
        crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_without_config_lifetime(),
            self.get_code_occurence_lifetime_with_deserialize().to_string(),//todo - do .to_string() inside inner
        )
    }
}
