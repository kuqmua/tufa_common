pub trait ErrorLog<ConfigGeneric> {
    fn error_log(&self, config: &ConfigGeneric);
}
