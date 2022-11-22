use crate::common::git::git_info::GitInformation;

impl GitInformation<'_> {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        println!("self.repo_link {}", self.repo_link);
        format!("{}/blob/{}/{file}#L{line}", self.repo_link, self.commit_id)
    }
}