pub trait GetSource {
    fn get_source(&self) -> String;
}

pub trait GetErrorWrapperSourceAsSting<ConfigGeneric> {
    fn get_error_wrapper_source_as_string(&self, config: &ConfigGeneric) -> String;
}
