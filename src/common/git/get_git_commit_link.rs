pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> std::string::String;
}

impl<T> GetGitCommitLink for T
where
    T: error_occurence_lib::git_fields::GetGitCommitId
        + error_occurence_lib::git_fields::GetGitRepoLink,
{
    fn get_git_commit_link(&self) -> std::string::String {
        format!(
            "{}/tree/{}",
            self.get_git_repo_link(),
            self.get_git_commit_id()
        )
    }
}
