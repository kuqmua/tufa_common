use crate::helpers::git::git_info::GitInfo;

impl GitInfo {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        //was "https://{}/blob/{}/{file}#L{line}",
        format!("{}/blob/{}/{file}#L{line}", self.repo_link, self.commit_id)
    }
}
