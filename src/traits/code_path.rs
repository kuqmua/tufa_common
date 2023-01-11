use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::get_git_source_file_link::GetGitSourceFileLink;

pub trait CodePath {
    fn get_code_path(
        &self,
        source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    ) -> String;
    fn get_project_code_path(&self) -> String;
    fn get_github_code_path(&self) -> String; //theoretically can make it const fn
}

impl<SelfGeneric> CodePath for SelfGeneric
where
    SelfGeneric: crate::traits::fields::GetFile
        + crate::traits::fields::GetLine
        + crate::traits::fields::GetColumn
        + crate::traits::get_git_info::GetGitInfoWithoutLifetimes,
{
    fn get_code_path(&self, source_place_type: &SourcePlaceType) -> String {
        match source_place_type {
            SourcePlaceType::Source => self.get_project_code_path(),
            SourcePlaceType::Github => self.get_github_code_path(),
            SourcePlaceType::None => String::from(""), //todo maybe incorrect?
        }
    }
    fn get_project_code_path(&self) -> String {
        format!(
            "src/{}:{}:{}", //todo "src" - hardcode, for some reason vscode stops following just {}:{}:{} path(without prefix "src")
            self.get_file(),
            self.get_line(),
            self.get_column()
        )
    }
    //todo make it const fn
    fn get_github_code_path(&self) -> String {
        let file = self.get_file().clone();
        let backslash = "/";
        match file.find(backslash) {
            Some(index) => self
                .get_git_info_without_lifetimes()
                .get_git_source_file_link(&file[index + backslash.len()..], *self.get_line()),
            None => String::from("cant find backslash symbol in file path of location"),
        }
    }
}
