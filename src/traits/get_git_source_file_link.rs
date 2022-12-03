use crate::traits::get_git_commit_link::GetGitCommitLink;
use crate::traits::git_info::GetAuthor;
use crate::traits::git_info::GetAuthorEmail;
use crate::traits::git_info::GetCommitId;
use crate::traits::git_info::GetCommitUnixTime;
use crate::traits::git_info::GetMessage;
use crate::traits::git_info::GetRepoLink;
use crate::traits::git_info::GetTimezone;

pub trait GetGitSourceFileLink {
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String;
}

impl<T> GetGitSourceFileLink for T
where
    T: GetCommitId + GetRepoLink,
{
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        format!(
            "{}/blob/{}/{file}#L{line}",
            self.get_repo_link(),
            self.get_commit_id()
        )
    }
}
