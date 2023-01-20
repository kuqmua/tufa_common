pub trait GetSource {
    fn get_source(&self) -> String;
}

pub trait GetSourceAsString<ConfigGeneric> {
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String;
}
