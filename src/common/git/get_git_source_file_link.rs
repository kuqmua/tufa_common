use crate::common::git::git_info::GitInformation;

impl GitInformation<'_> {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        format!("{}/blob/{}/{file}#L{line}", self.repo_link, self.commit_id)
    }
}

use crate::common::where_was::GitInfoForWhereWas;

impl GitInfoForWhereWas {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        format!("{}/blob/{}/{file}#L{line}", self.repo_link, self.commit_id)
    }
}
