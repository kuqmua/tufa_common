#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone, Eq, Hash, PartialEq, Default)]
pub struct GitInformation<'a> {
    pub git_commit_id: &'a str,
    pub git_repo_link: &'a str,
    pub git_author: &'a str,
    pub git_author_email: &'a str,
    pub git_commit_unix_time: &'a str,
    pub git_timezone: &'a str,
    pub git_message: &'a str,
}

impl<'a> crate::traits::fields::GetGitCommitIdLifetime<'a> for GitInformation<'a> {
    fn get_git_commit_id_lifetime(&self) -> &'a str {
        self.git_commit_id
    }
}

impl<'a> crate::traits::fields::GetGitRepoLinkLifetime<'a> for GitInformation<'a> {
    fn get_git_repo_link_lifetime(&self) -> &'a str {
        self.git_repo_link
    }
}

impl<'a> crate::traits::fields::GetGitAuthorLifetime<'a> for GitInformation<'a> {
    fn get_git_author_lifetime(&self) -> &'a str {
        self.git_author
    }
}

impl<'a> crate::traits::fields::GetGitAuthorEmailLifetime<'a> for GitInformation<'a> {
    fn get_git_author_email_lifetime(&self) -> &'a str {
        self.git_author_email
    }
}

impl<'a> crate::traits::fields::GetGitCommitUnixTimeLifetime<'a> for GitInformation<'a> {
    fn get_git_commit_unix_time_lifetime(&self) -> &'a str {
        self.git_commit_unix_time
    }
}

impl<'a> crate::traits::fields::GetGitTimezoneLifetime<'a> for GitInformation<'a> {
    fn get_git_timezone_lifetime(&self) -> &'a str {
        self.git_timezone
    }
}

impl<'a> crate::traits::fields::GetGitMessageLifetime<'a> for GitInformation<'a> {
    fn get_git_message_lifetime(&self) -> &'a str {
        self.git_message
    }
}

impl GitInformation<'static> {
    pub fn get_runtime_version(&self) -> GitInformationWithoutLifetimes {
        GitInformationWithoutLifetimes {
            git_commit_id: String::from(self.git_commit_id),
            git_repo_link: String::from(self.git_repo_link),
            git_author: String::from(self.git_author),
            git_author_email: String::from(self.git_author_email),
            git_commit_unix_time: String::from(self.git_commit_unix_time),
            git_timezone: String::from(self.git_timezone),
            git_message: String::from(self.git_message),
        }
    }
}

#[derive(
    Debug,
    Clone,
    Eq,
    Hash,
    PartialEq,
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFieldsFromCrate,
    serde_derive::Serialize,
    serde_derive::Deserialize,
)]
pub struct GitInformationWithoutLifetimes {
    pub git_commit_id: String,
    pub git_repo_link: String,
    pub git_author: String,
    pub git_author_email: String,
    pub git_commit_unix_time: String,
    pub git_timezone: String,
    pub git_message: String,
}

// pub trait GetGitCommitId {
//     fn get_git_commit_id(&self) -> &String {
//         self.git_commit_id
//     }
// }

// pub trait GetGitRepoLink {
//     fn get_git_repo_link(&self) -> &String {
//         self.git_repo_link
//     }
// }

// pub trait GetGitAuthor {
//     fn get_git_author(&self) -> &String {
//         self.git_author
//     }
// }

// pub trait GetGitAuthorEmail {
//     fn get_git_author_email(&self) -> &String {
//         self.git_author_email
//     }
// }

// pub trait GetGitCommitUnixTime {
//     fn get_git_commit_unix_time(&self) -> &String {
//         self.git_commit_unix_time
//     }
// }

// pub trait GetGitTimezone {
//     fn get_git_timezone(&self) -> &String {
//         self.git_timezone
//     }
// }

// pub trait GetGitMessage {
//     fn get_git_message(&self) -> &String {
//         self.git_message
//     }
// }
