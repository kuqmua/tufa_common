pub trait GetGitSourceFileLink {
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String;
}

impl<SelfGeneric> GetGitSourceFileLink for SelfGeneric
where
    Self: crate::traits::git_info::GetRepoLink + crate::traits::git_info::GetCommitId,
{
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        format!(
            "{}/blob/{}/{file}#L{line}",
            self.get_repo_link(),
            self.get_commit_id()
        )
    }
}
