pub trait CodeOccurenceNewWithAddition<SourceGeneric> {
    fn new_with_addition(
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
        source_generic: &SourceGeneric,
    ) -> Self;
}

pub trait CodeOccurenceNew {
    fn new(
        git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self;
}

pub trait CodeOccurenceLog<ConfigGeneric, ErrorColorBoldGeneric> {
    fn log(&self, source: String, config_generic: ConfigGeneric);
}
