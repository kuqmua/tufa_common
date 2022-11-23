use crate::common::git::git_info::GitInformation;

impl GitInformation<'_> {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn get_commit_link(&self) -> String {
        format!("{}/tree/{}/", self.repo_link, self.commit_id)
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
    pub fn get_commit_link(&self) -> String {
        format!("{}/tree/{}/", self.repo_link, self.commit_id)
    }
}
