pub trait SourceToStringWithConfig<'a, ConfigGeneric> {
    fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String;
}
