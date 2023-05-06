pub trait GetGitInfo<'a> {
    fn get_git_info(&self) -> &'a crate::common::git::git_info::GitInformation;
}
