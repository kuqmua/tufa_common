use crate::helpers::git::get_git_commit_info::get_git_commit_info;
use crate::helpers::git::get_git_commit_info::GitCommitInfo;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GIT_INFO: GitCommitInfo = get_git_commit_info();
}
