pub trait CodePath {
    fn get_code_path(
        &self,
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
        source_place_type: crate::config_mods::source_place_type::SourcePlaceType,
    ) -> String;
    fn get_project_code_path(&self) -> String;
    fn get_github_code_path(
        &self,
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
    ) -> String; //theoretically can make it const fn
}
