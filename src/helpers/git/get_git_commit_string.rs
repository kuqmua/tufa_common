use crate::helpers::git::git_info::GitInformation;

impl GitInformation {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn get_git_commit_string(&self) -> String {
        format!(
            "{} branch'{}' {} {}",
            self.repo_link, self.branch, self.commit_id, self.commit_message,
        )
    }
}
