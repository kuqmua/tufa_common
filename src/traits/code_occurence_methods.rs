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

pub trait CodeOccurenceLog<ConfigGeneric, ErrorColorBoldGeneric, SourceGeneric> {
    fn log(&self, source: &SourceGeneric, config_generic: ConfigGeneric);
}
