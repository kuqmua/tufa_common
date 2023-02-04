pub trait SourceToStringWithConfig<ConfigGeneric> {
    fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String;
}
