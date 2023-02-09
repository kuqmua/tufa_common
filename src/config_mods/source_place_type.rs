#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourcePlaceType {
    Source,
    Github,
    None,
}

impl SourcePlaceType {
    pub fn get_code_path(
        &self,
        code_occurence: &(impl crate::traits::fields::GetFile
              + crate::traits::fields::GetLine
              + crate::traits::fields::GetColumn
              + crate::traits::get_git_info::GetClonedGitInfo),
    ) -> String {
        match self {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                use crate::traits::error_logs_logic::form_error_path::FormErrorPathDirectory;
                code_occurence.form_error_path_directory()
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                use crate::traits::error_logs_logic::form_error_path::FormErrorPathGithub;
                code_occurence.form_error_path_github()
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => String::from(""), //todo maybe incorrect?
        }
    }
}

impl Default for SourcePlaceType {
    fn default() -> Self {
        Self::None
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
            "none" => Ok(SourcePlaceType::None),
            _ => Err(ParseSourcePlaceTypeError {
                _incorrect_str: e.to_string(),
            }),
        }
    }
}
