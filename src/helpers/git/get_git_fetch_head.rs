use crate::helpers::git::git_info::GitInformation;

impl GitInformation {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn get_git_fetch_head(&self) -> String {
        format!(
            "{}                branch '{}' of {}",
            self.commit_id, self.branch, self.repo_link
        )
    }
}
