use crate::traits::git_info::GetCommitId;
use crate::traits::git_info::GetRepoLink;

pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> String;
}

impl<T> GetGitCommitLink for T
where
    T: GetCommitId + GetRepoLink,
{
    fn get_git_commit_link(&self) -> String {
        format!("{}/tree/{}/", self.get_repo_link(), self.get_commit_id())
    }
}
