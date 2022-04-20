use crate::helpers::git::lazy_static_git_info::GIT_INFO;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn get_commit_link() -> String {
    format!(
        "{}/tree/{}/",
        GIT_INFO.repo_link, GIT_INFO.commit_id
    )
}
