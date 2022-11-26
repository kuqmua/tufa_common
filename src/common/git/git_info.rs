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
