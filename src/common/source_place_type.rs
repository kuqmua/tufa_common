#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    strum_macros::Display,
    serde::Serialize,
    serde::Deserialize,
    from_str::FromStr,
)]
pub enum SourcePlaceType {
    Source,
    Github,
}

impl<'a> SourcePlaceType {
    pub fn get_code_path(
        &self,
        code_occurence: &(impl error_occurence_lib::get_file::GetFile
              + error_occurence_lib::get_line::GetLine
              + error_occurence_lib::get_column::GetColumn
              + crate::common::git::get_git_source_file_link::GetGitSourceFileLink<'a>),
    ) -> String {
        match self {
            SourcePlaceType::Source => crate::common::error_logs_logic::form_error_path::FormErrorPathDirectory::form_error_path_directory(code_occurence),
            SourcePlaceType::Github => crate::common::error_logs_logic::form_error_path::FormErrorPathGithub::form_error_path_github(code_occurence)
        }
    }
}

impl Default for SourcePlaceType {
    fn default() -> Self {
        Self::Source
    }
}
