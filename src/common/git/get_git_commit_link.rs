pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> std::string::String;
}

impl<T> GetGitCommitLink for T
where
    T: crate::common::git::git_fields::GetGitCommitId
        + crate::common::git::git_fields::GetGitRepoLink,
{
    fn get_git_commit_link(&self) -> std::string::String {
        format!(
            "{}/tree/{}",
            self.get_git_repo_link(),
            self.get_git_commit_id()
        )
    }
}
