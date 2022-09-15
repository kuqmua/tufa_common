//for some reason cannot get GitInformation type directly from lazy_static value. temp solution - put in struct wrapper
pub struct GitInformationWrapper {
    pub data: GitInformation,
}

impl GitInformationWrapper {
    pub fn init(repo_git_path: &str, repo_name: &str) -> Self {
        GitInformationWrapper {
            data: GitInformation::get_git_commit_info(repo_git_path, repo_name),
        }
    }
}

pub struct GitInformation {
    pub commit_id: String,
    pub repo_link: String,
    pub author: String,
    pub author_email: String,
    pub commit_unix_time: String,
    pub timezone: String,
    pub message: String,
}
