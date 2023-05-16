#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourcePlaceType {
    Source,
    Github,
}

impl<'a> SourcePlaceType {
    pub fn get_code_path(
        &self,
        code_occurence: &(impl crate::traits::get_file::GetFile
              + crate::traits::get_line::GetLine
              + crate::traits::get_column::GetColumn
              + crate::traits::get_git_source_file_link::GetGitSourceFileLink<'a>),
    ) -> String {
        match self {
            SourcePlaceType::Source => {
                use crate::traits::error_logs_logic::form_error_path::FormErrorPathDirectory;
                code_occurence.form_error_path_directory()
            }
            SourcePlaceType::Github => {
                use crate::traits::error_logs_logic::form_error_path::FormErrorPathGithub;
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
