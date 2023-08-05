pub trait SourceToStringWithConfig<'a> {
    fn source_to_string_with_config<
        ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
            + crate::common::config::config_fields::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> String;
}
