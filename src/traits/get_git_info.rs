pub trait GetGitInfo<'a> {
    fn get_git_info(&self) -> &'a crate::common::git::git_info::GitInformation;
}

pub trait GetGitInfoWithoutLifetimes {
    fn get_git_info_without_lifetimes(
        &self,
    ) -> &crate::common::git::git_info::GitInformationWithoutLifetimes;
}
