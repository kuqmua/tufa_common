#[derive(
    Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone, Eq, Hash, PartialEq, Default,
)]
pub struct GitInfo<'a> {
    //todo - two GitInfo struct - one &'static, second for runtime deserialization, without static
    pub git_commit_id: &'a str,
    pub git_repo_link: &'a str,
    // pub git_author: &'a str,
    // pub git_author_email: &'a str,
    // pub git_commit_unix_time: &'a str,
    // pub git_timezone: &'a str,
    // pub git_message: &'a str,
}

impl<'a> GitInfo<'a> {
    pub fn to_git_info_without_lifetime(&self) -> GitInfoWithoutLifetime {
        GitInfoWithoutLifetime {
            git_commit_id: self.git_commit_id.to_owned(),
            git_repo_link: self.git_repo_link.to_owned(),
        }
    }
}

impl<'a> crate::common::git::git_fields::GetGitCommitId for GitInfo<'a> {
    fn get_git_commit_id(&self) -> std::string::String {
        self.git_commit_id.to_string()
    }
}

impl<'a> crate::common::git::git_fields::GetGitRepoLink for GitInfo<'a> {
    fn get_git_repo_link(&self) -> std::string::String {
        self.git_repo_link.to_string()
    }
}

// impl<'a> crate::common::git::git_fields::GetGitAuthor<'a> for GitInfo<'a> {
//     fn get_git_author(&self) -> &'a str {
//         self.git_author
//     }
// }

// impl<'a> crate::common::git::git_fields::GetGitAuthorEmail<'a> for GitInfo<'a> {
//     fn get_git_author_email(&self) -> &'a str {
//         self.git_author_email
//     }
// }

// impl<'a> crate::common::git::git_fields::GetGitCommitUnixTime<'a> for GitInfo<'a> {
//     fn get_git_commit_unix_time(&self) -> &'a str {
//         self.git_commit_unix_time
//     }
// }

// impl<'a> crate::common::git::git_fields::GetGitTimezone<'a> for GitInfo<'a> {
//     fn get_git_timezone(&self) -> &'a str {
//         self.git_timezone
//     }
// }

// impl<'a> crate::common::git::git_fields::GetGitMessage<'a> for GitInfo<'a> {
//     fn get_git_message(&self) -> &'a str {
//         self.git_message
//     }
// }

#[derive(
    Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone, Eq, Hash, PartialEq, Default, utoipa::ToSchema
)]
pub struct GitInfoWithoutLifetime {
    pub git_commit_id: std::string::String,
    pub git_repo_link: std::string::String,
}

impl crate::common::git::git_fields::GetGitCommitId for GitInfoWithoutLifetime {
    fn get_git_commit_id(&self) -> std::string::String {
        self.git_commit_id.to_string()
    }
}

impl crate::common::git::git_fields::GetGitRepoLink for GitInfoWithoutLifetime {
    fn get_git_repo_link(&self) -> std::string::String {
        self.git_repo_link.to_string()
    }
}

pub trait GetGitInfo<'a> {
    fn get_git_info(&self) -> &'a GitInfo<'a>;
}
