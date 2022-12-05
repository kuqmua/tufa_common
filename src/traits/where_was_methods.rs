pub trait WhereWasMethods {
    fn readable_time(&self, timezone: i32) -> String;
    fn file_line_column(&self) -> String;
    fn github_file_line_column(
        &self,
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
    ) -> String;
}
