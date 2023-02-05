// pub trait ToStringWithoutConfig {
//     fn to_string_without_config(&self) -> String;
// }

// impl<SelfGeneric> ToStringWithoutConfig for SelfGeneric
// where
//     SelfGeneric:
//         crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig
//             + crate::traits::get_code_occurence::GetCodeOccurence,
// {
//     fn to_string_without_config(&self) -> String {
//         crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
//             self.source_to_string_without_config(),
//             self.get_code_occurence().to_string(),
//         )
//     }
// }
//
pub trait ToStringWithoutConfigLifetime<'a> {
    fn to_string_without_config_lifetime(&self) -> String;
}

impl<'a, SelfGeneric> ToStringWithoutConfigLifetime<'a> for SelfGeneric
where
    SelfGeneric:
        crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a>
            + crate::traits::get_code_occurence::GetCodeOccurenceLifetime<'a>,
{
    fn to_string_without_config_lifetime(&self) -> String {
        crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_without_config_lifetime(),
            self.get_code_occurence_lifetime().to_string(),
        )
    }
}
