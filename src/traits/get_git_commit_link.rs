pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> String;
}

impl<T> GetGitCommitLink for T
where
    T: crate::traits::fields::GetGitCommitId + crate::traits::fields::GetGitRepoLink,
{
    fn get_git_commit_link(&self) -> String {
        format!(
            "{}/tree/{}/",
            self.get_git_repo_link(),
            self.get_git_commit_id()
        )
    }
}
