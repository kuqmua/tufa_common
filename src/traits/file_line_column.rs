pub trait FileLineColumnTrait {
    fn file_line_column(&self) -> String;
    fn github_file_line_column(
        &self,
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
    ) -> String; //theoretically can make it const fn
}
