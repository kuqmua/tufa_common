pub trait SourceToStringWithConfigSecond<'a> {
    fn source_to_string_with_config_second<
        ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
            + crate::common::config::config_fields::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> String;
}
