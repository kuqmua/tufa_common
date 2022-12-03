use crate::common::git::git_info::GitInformation;

impl GitInformation<'_> {
    pub fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        format!("{}/blob/{}/{file}#L{line}", self.repo_link, self.commit_id)
    }
}

use crate::common::git::git_info::GitInformationWithoutLifetimes;
use crate::common::where_was::GitInfoForWhereWas;

impl GitInfoForWhereWas {
    pub fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        format!("{}/blob/{}/{file}#L{line}", self.repo_link, self.commit_id)
    }
}

impl GitInformationWithoutLifetimes {
    pub fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        format!("{}/blob/{}/{file}#L{line}", self.repo_link, self.commit_id)
    }
}
