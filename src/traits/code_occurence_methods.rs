pub trait CodeOccurenceMethods {
    fn new(
        git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self;
    fn add(self, another_code_occurence: &Self) -> Self;
    fn log_code_occurence(
        &self,
        source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
        log_type: &crate::config_mods::log_type::LogType,
        source: String,
        style: ansi_term::Style,
    );
}
