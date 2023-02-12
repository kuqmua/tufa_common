pub trait SourceToStringWithConfigLifetime<'a, ConfigGeneric> {
    fn source_to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String;
}

pub trait SourceToStringWithConfigLifetimeWithDeserialize<'a, ConfigGeneric> {
    fn source_to_string_with_config_lifetime_with_deserialize(
        &self,
        config: &ConfigGeneric,
    ) -> String;
}
