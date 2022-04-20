use crate::helpers::git::git_info::GitInfo;

impl GitInfo {
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
