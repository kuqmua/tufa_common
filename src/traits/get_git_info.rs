use crate::common::git::git_info::GitInformation;

pub trait GetGitInfo {
    fn get_git_info(&self) -> &'static GitInformation;
}
