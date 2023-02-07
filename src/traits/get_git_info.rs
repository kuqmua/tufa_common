use crate::common::git::git_info::GitInformation;

pub trait GetGitInfo<'a> {
    fn get_git_info(&self) -> &'a GitInformation;
}

pub trait GetGitInfoWithoutLifetimes {
    fn get_git_info_without_lifetimes(
        &self,
    ) -> &crate::common::git::git_info::GitInformationWithoutLifetimes;
}

pub trait GetClonedGitInfo {
    fn get_cloned_git_info(&self) -> GitInformation;
}
