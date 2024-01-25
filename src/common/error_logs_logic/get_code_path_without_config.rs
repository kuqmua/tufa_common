pub trait GetCodePathWithoutConfig {
    fn get_code_path_without_config(&self) -> String;
}

impl<SelfGeneric> GetCodePathWithoutConfig for SelfGeneric
where
    SelfGeneric: error_occurence_lib::get_file::GetFile
        + error_occurence_lib::get_line::GetLine
        + error_occurence_lib::get_column::GetColumn,
{
    fn get_code_path_without_config(&self) -> String {
        format!(
            "src/{}:{}:{}", //todo "src" - hardcode, for some reason vscode stops following just {}:{}:{} path(without prefix "src")
            self.get_file(),
            self.get_line(),
            self.get_column()
        )
    }
}
