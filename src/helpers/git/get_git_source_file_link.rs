use crate::helpers::git::lazy_static_git_info::GIT_INFO;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn get_git_source_file_link(file: &str, line: u32) -> String {
    format!(
        "https://{}/blob/{}/{file}#L{line}",
        GIT_INFO.repo_link, GIT_INFO.commit_id
    )
}
