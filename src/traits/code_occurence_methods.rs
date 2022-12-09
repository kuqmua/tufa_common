pub trait CodeOccurenceMethods {
    fn new(
        git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self;
    fn new_with_addition(
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
        another_code_occurence: &Self,
    ) -> Self;
    fn log_code_occurence(
        &self,
        source: String,
        source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
        log_type: &crate::config_mods::log_type::LogType,
        style: ansi_term::Style,
    );
}
