pub trait PrepareOriginErrorDisplay {
    fn prepare_origin_error_display(&self) -> String;
}

impl<SelfGeneric> PrepareOriginErrorDisplay for SelfGeneric
where
    SelfGeneric: crate::traits::to_string_without_config::ToStringWithoutConfig
        + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn prepare_origin_error_display(&self) -> String {
        format!(
            "{}{}",
            self.to_string_without_config(),
            self.get_code_occurence()
        )
    }
}
