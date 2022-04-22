use crate::helpers::git::git_info::GitInformation;

impl GitInformation {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        //todo - sync with tufa_project. it working correctly with tufa_server but not working correctly with tufa_project
        //
        //was "https://{}/blob/{}/{file}#L{line}",
        format!("{}/blob/{}/{file}#L{line}", self.repo_link, self.commit_id)
    }
}
