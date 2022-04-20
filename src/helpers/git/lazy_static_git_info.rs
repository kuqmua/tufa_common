use crate::helpers::git::git_info::GitInformation;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GIT_INFO: GitInformation = GitInformation::get_git_commit_info("../");
}
