pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> String;
}

impl<'a, T> GetGitCommitLink for T
where
    T: crate::common::git::git_fields::GetGitCommitId<'a> + crate::common::git::git_fields::GetGitRepoLink<'a>,
{
    fn get_git_commit_link(&self) -> String {
        format!(
            "{}/tree/{}/",
            self.get_git_repo_link(),
            self.get_git_commit_id()
        )
    }
}