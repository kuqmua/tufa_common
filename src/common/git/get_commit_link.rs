use crate::common::git::git_info::GitInformation;

impl GitInformation<'_> {
    pub fn get_commit_link(&self) -> String {
        format!("{}/tree/{}/", self.repo_link, self.commit_id)
    }
}
