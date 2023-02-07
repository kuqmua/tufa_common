use crate::traits::get_git_source_file_link::GetGitSourceFileLinkLifetime;
use std::str::FromStr;

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
            crate::config_mods::source_place_type::SourcePlaceType::Source => format!(
                "src/{}:{}:{}", //todo "src" - hardcode, for some reason vscode stops following just {}:{}:{} path(without prefix "src")
                code_occurence.get_file(),
                code_occurence.get_line(),
                code_occurence.get_column()
            ),
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                let backslash = "/";
                let file = code_occurence.get_file();
                match file.find(backslash) {
                    Some(index) => code_occurence
                        .get_cloned_git_info()
                        .get_git_source_file_link_lifetime(
                            &file[index + backslash.len()..],
                            *code_occurence.get_line(),
                        ),
                    None => String::from("cant find backslash symbol in file path of location"),
                }
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

impl FromStr for SourcePlaceType {
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
