pub const PROJECT_COMMIT: &str = "project_commit";

#[derive(
    Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone, Eq, Hash, PartialEq, Default,
)]
pub struct ProjectGitInfo<'a> {
    pub project_commit: &'a str,
}

impl ProjectGitInfo<'_> {
    pub fn does_not_match_message(&self) -> &'static str {
        "please use special http request function from https://github.com/kuqmua/tufa_project for this API"
    }
    pub fn cannot_convert_project_commit_to_str_message(&self) -> &'static str {
        "cannot convert project commit to str, please check request headers"
    }
    pub fn no_project_commit_header_message(&self) -> &'static str {
        "no project_commit header provided, please add project_commit to request headers"
    }
}

impl<'a> crate::common::git::git_fields::GetGitCommitId for ProjectGitInfo<'a> {
    fn get_git_commit_id(&self) -> std::string::String {
        self.project_commit.to_string()
    }
}

impl<'a> crate::common::git::get_git_commit_link::GetGitCommitLink for ProjectGitInfo<'a> {
    fn get_git_commit_link(&self) -> std::string::String {
        format!(
            "https://github.com/kuqmua/tufa_project/tree/{}", //todo get git_author and git_name from .git directory
            self.project_commit
        )
    }
}

pub trait GetProjectGitCommitLink {
    fn get_project_git_commit_link(&self) -> std::string::String;
}

impl<'a> GetProjectGitCommitLink for ProjectGitInfo<'a> {
    fn get_project_git_commit_link(&self) -> std::string::String {
        crate::common::git::get_git_commit_link::GetGitCommitLink::get_git_commit_link(self)
    }
}
