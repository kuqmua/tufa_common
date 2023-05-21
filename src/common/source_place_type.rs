#[derive(
    Debug, 
    Clone, 
    PartialEq, 
    Eq,
    strum_macros::Display,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum SourcePlaceType {
    Source,
    Github,
}

impl<'a> SourcePlaceType {
    pub fn get_code_path(
        &self,
        code_occurence: &(impl crate::common::error_logs_logic::get_file::GetFile
              + crate::common::error_logs_logic::get_line::GetLine
              + crate::common::error_logs_logic::get_column::GetColumn
              + crate::common::git::get_git_source_file_link::GetGitSourceFileLink<'a>),
    ) -> String {
        match self {
            SourcePlaceType::Source => {
                use crate::common::error_logs_logic::form_error_path::FormErrorPathDirectory;
                code_occurence.form_error_path_directory()
            }
            SourcePlaceType::Github => {
                use crate::common::error_logs_logic::form_error_path::FormErrorPathGithub;
                code_occurence.form_error_path_github()
            }
        }
    }
}

impl Default for SourcePlaceType {
    fn default() -> Self {
        Self::Source
    }
}

pub struct ParseSourcePlaceTypeError {
    _incorrect_str: String,
}

impl std::str::FromStr for SourcePlaceType {
    type Err = ParseSourcePlaceTypeError;
    fn from_str(e: &str) -> Result<Self, ParseSourcePlaceTypeError> {
        match e {
            "source" => Ok(SourcePlaceType::Source),
            "github" => Ok(SourcePlaceType::Github),
            _ => Err(ParseSourcePlaceTypeError {
                _incorrect_str: e.to_string(),
            }),
        }
    }
}
