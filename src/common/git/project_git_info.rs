#[derive(
    Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone, Eq, Hash, PartialEq, Default,
)]
pub struct ProjectGitInfo<'a> {
    pub git_commit_id: &'a str,
}

impl<'a> crate::common::git::git_fields::GetGitCommitId<'a> for ProjectGitInfo<'a> {
    fn get_git_commit_id(&self) -> &'a str {
        self.git_commit_id
    }
}

impl<'a> crate::common::git::get_git_commit_link::GetGitCommitLink for ProjectGitInfo<'a> {
    fn get_git_commit_link(&self) -> String {
        format!(
            "https://github.com/kuqmua/tufa_project/commit/{}", //todo get git_author and git_name from .git directory
            self.git_commit_id
        )
    }
}
