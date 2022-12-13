use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::get_git_source_file_link::GetGitSourceFileLink;

pub trait CodePath {
    fn get_code_path(
        &self,
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
        source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    ) -> String;
    fn get_project_code_path(&self) -> String;
    fn get_github_code_path(
        &self,
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
    ) -> String; //theoretically can make it const fn
}

impl<SelfGeneric> CodePath for SelfGeneric
where
    SelfGeneric: crate::traits::get_file::GetFile
        + crate::traits::get_line::GetLine
        + crate::traits::get_column::GetColumn,
{
    fn get_code_path(
        &self,
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
        source_place_type: &SourcePlaceType,
    ) -> String {
        match source_place_type {
            SourcePlaceType::Source => self.get_project_code_path(),
            SourcePlaceType::Github => self.get_github_code_path(git_info),
            SourcePlaceType::None => String::from(""), //todo maybe incorrect?
        }
    }
    fn get_project_code_path(&self) -> String {
        format!(
            "{}:{}:{}",
            self.get_file(),
            self.get_line(),
            self.get_column()
        )
    }
    //todo make it const fn
    fn get_github_code_path(
        &self,
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
    ) -> String {
        let file = self.get_file().clone();
        let backslash = "/";
        match file.find(backslash) {
            Some(index) => git_info
                .get_git_source_file_link(&file[index + backslash.len()..], *self.get_line()),
            None => String::from("cant find backslash symbol in file path of location"),
        }
    }
}
