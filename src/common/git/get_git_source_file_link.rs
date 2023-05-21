pub trait GetGitSourceFileLink<'a> {
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String;
}

impl<'a, SelfGeneric> GetGitSourceFileLink<'a> for SelfGeneric
where
    Self: crate::common::git::git_fields::GetGitRepoLink<'a>
        + crate::common::git::git_fields::GetGitCommitId<'a>,
{
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        format!(
            "{}/blob/{}/{file}#L{line}",
            self.get_git_repo_link(),
            self.get_git_commit_id()
        )
    }
}
