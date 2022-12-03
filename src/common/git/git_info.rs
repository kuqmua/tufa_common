use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct GitInformation<'a> {
    pub commit_id: &'a str,
    pub repo_link: &'a str,
    pub author: &'a str,
    pub author_email: &'a str,
    pub commit_unix_time: &'a str,
    pub timezone: &'a str,
    pub message: &'a str,
}

impl GitInformation<'static> {
    pub fn get_runtime_version(&self) -> GitInformationWithoutLifetimes {
        GitInformationWithoutLifetimes {
            commit_id: String::from(self.commit_id),
            repo_link: String::from(self.repo_link),
            author: String::from(self.author),
            author_email: String::from(self.author_email),
            commit_unix_time: String::from(self.commit_unix_time),
            timezone: String::from(self.timezone),
            message: String::from(self.message),
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct GitInformationWithoutLifetimes {
    pub commit_id: String,
    pub repo_link: String,
    pub author: String,
    pub author_email: String,
    pub commit_unix_time: String,
    pub timezone: String,
    pub message: String,
}
