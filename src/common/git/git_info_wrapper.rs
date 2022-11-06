//for some reason cannot get GitInformation type directly from lazy_static value. temp solution - put in struct wrapper
pub struct GitInformationWrapper {
    pub data: crate::common::git::git_info::GitInformation,
}

impl GitInformationWrapper {
    pub fn init(repo_git_path: &str, repo_name: &str) -> Self {
        GitInformationWrapper {
            data: crate::common::git::git_info::GitInformation::get_git_commit_info(
                repo_git_path,
                repo_name,
            ),
        }
    }
}
