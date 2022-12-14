pub trait GetGitSourceFileLink {
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String;
}

impl<SelfGeneric> GetGitSourceFileLink for SelfGeneric
where
    Self: crate::traits::fields::GetGitRepoLink + crate::traits::fields::GetGitCommitId,
{
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        format!(
            "{}/blob/{}/{file}#L{line}",
            self.get_git_repo_link(),
            self.get_git_commit_id()
        )
    }
}
