pub trait GetGitSourceFileLink<'a> {
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String;
}

impl<'a, SelfGeneric> GetGitSourceFileLink<'a> for SelfGeneric
where
    Self: crate::traits::fields::GetGitRepoLinkLifetime<'a>
        + crate::traits::fields::GetGitCommitIdLifetime<'a>,
{
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        format!(
            "{}/blob/{}/{file}#L{line}",
            self.get_git_repo_link_lifetime(),
            self.get_git_commit_id_lifetime()
        )
    }
}
