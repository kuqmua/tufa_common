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
