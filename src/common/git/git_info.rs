#[derive(
    Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone, Eq, Hash, PartialEq, Default,
)]
pub struct GitInformation<'a> {
    pub git_commit_id: &'a str,
    pub git_repo_link: &'a str,
    // pub git_author: &'a str,
    // pub git_author_email: &'a str,
    // pub git_commit_unix_time: &'a str,
    // pub git_timezone: &'a str,
    // pub git_message: &'a str,
}

impl<'a> crate::common::git::git_fields::GetGitCommitId<'a> for GitInformation<'a> {
    fn get_git_commit_id(&self) -> &'a str {
        self.git_commit_id
    }
}

impl<'a> crate::common::git::git_fields::GetGitRepoLink<'a> for GitInformation<'a> {
    fn get_git_repo_link(&self) -> &'a str {
        self.git_repo_link
    }
}

// impl<'a> crate::common::git::git_fields::GetGitAuthor<'a> for GitInformation<'a> {
//     fn get_git_author(&self) -> &'a str {
//         self.git_author
//     }
// }

// impl<'a> crate::common::git::git_fields::GetGitAuthorEmail<'a> for GitInformation<'a> {
//     fn get_git_author_email(&self) -> &'a str {
//         self.git_author_email
//     }
// }

// impl<'a> crate::common::git::git_fields::GetGitCommitUnixTime<'a> for GitInformation<'a> {
//     fn get_git_commit_unix_time(&self) -> &'a str {
//         self.git_commit_unix_time
//     }
// }

// impl<'a> crate::common::git::git_fields::GetGitTimezone<'a> for GitInformation<'a> {
//     fn get_git_timezone(&self) -> &'a str {
//         self.git_timezone
//     }
// }

// impl<'a> crate::common::git::git_fields::GetGitMessage<'a> for GitInformation<'a> {
//     fn get_git_message(&self) -> &'a str {
//         self.git_message
//     }
// }
