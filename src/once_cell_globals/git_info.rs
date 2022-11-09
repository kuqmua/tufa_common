// use crate::project_constants::PROJECT_NAME;
use crate::common::git::git_info::GitInformation;
use once_cell::sync::Lazy;

pub static GIT_INFO: Lazy<GitInformation> =
    Lazy::new(|| GitInformation::get_git_commit_info("../", "tufa_common"));
