use crate::common::git::git_info::GitInformation;

impl GitInformation<'_> {
    pub fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        format!("{}/blob/{}/{file}#L{line}", self.repo_link, self.commit_id)
    }
}
//todo make one implementation with traits support
use crate::common::git::git_info::GitInformationWithoutLifetimes;

impl GitInformationWithoutLifetimes {
    pub fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        format!("{}/blob/{}/{file}#L{line}", self.repo_link, self.commit_id)
    }
}
