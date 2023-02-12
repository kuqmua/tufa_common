pub trait SourceToStringWithoutConfigLifetime<'a> {
    fn source_to_string_without_config_lifetime(&self) -> String;
}

pub trait SourceToStringWithoutConfigLifetimeWithDeserialize<'a> {
    fn source_to_string_without_config_lifetime_with_deserialize(&self) -> String;
}
