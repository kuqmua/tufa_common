use crate::common::git::git_info::GitInformation;

pub trait GetGitInfo {
    fn get_git_info(&self) -> &'static GitInformation;
}

pub trait GetGitInfoWithoutLifetimes {
    fn get_git_info_without_lifetimes(
        &self,
    ) -> &crate::common::git::git_info::GitInformationWithoutLifetimes;
}
