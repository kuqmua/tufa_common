use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct GitInformation<'a> {
    pub git_commit_id: &'a str,
    pub git_repo_link: &'a str,
    pub git_author: &'a str,
    pub git_author_email: &'a str,
    pub git_commit_unix_time: &'a str,
    pub git_timezone: &'a str,
    pub git_message: &'a str,
}

impl crate::traits::fields::GetGitCommitId for GitInformation<'_> {
    fn get_git_commit_id(&self) -> String {
        String::from(self.git_commit_id)
    }
}

impl crate::traits::fields::GetGitRepoLink for GitInformation<'_> {
    fn get_git_repo_link(&self) -> String {
        String::from(self.git_repo_link)
    }
}

impl crate::traits::fields::GetGitAuthor for GitInformation<'_> {
    fn get_git_author(&self) -> String {
        String::from(self.git_author)
    }
}

impl crate::traits::fields::GetGitAuthorEmail for GitInformation<'_> {
    fn get_git_author_email(&self) -> String {
        String::from(self.git_author_email)
    }
}

impl crate::traits::fields::GetGitCommitUnixTime for GitInformation<'_> {
    fn get_git_commit_unix_time(&self) -> String {
        String::from(self.git_commit_unix_time)
    }
}

impl crate::traits::fields::GetGitTimezone for GitInformation<'_> {
    fn get_git_timezone(&self) -> String {
        String::from(self.git_timezone)
    }
}

impl crate::traits::fields::GetGitMessage for GitInformation<'_> {
    fn get_git_message(&self) -> String {
        String::from(self.git_message)
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

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct GitInformationWithoutLifetimes {
    pub git_commit_id: String,
    pub git_repo_link: String,
    pub git_author: String,
    pub git_author_email: String,
    pub git_commit_unix_time: String,
    pub git_timezone: String,
    pub git_message: String,
}

impl crate::traits::fields::GetGitCommitId for GitInformationWithoutLifetimes {
    fn get_git_commit_id(&self) -> String {
        self.git_commit_id.clone()
    }
}

impl crate::traits::fields::GetGitRepoLink for GitInformationWithoutLifetimes {
    fn get_git_repo_link(&self) -> String {
        self.git_repo_link.clone()
    }
}

impl crate::traits::fields::GetGitAuthor for GitInformationWithoutLifetimes {
    fn get_git_author(&self) -> String {
        self.git_author.clone()
    }
}

impl crate::traits::fields::GetGitAuthorEmail for GitInformationWithoutLifetimes {
    fn get_git_author_email(&self) -> String {
        self.git_author_email.clone()
    }
}

impl crate::traits::fields::GetGitCommitUnixTime for GitInformationWithoutLifetimes {
    fn get_git_commit_unix_time(&self) -> String {
        self.git_commit_unix_time.clone()
    }
}

impl crate::traits::fields::GetGitTimezone for GitInformationWithoutLifetimes {
    fn get_git_timezone(&self) -> String {
        self.git_timezone.clone()
    }
}

impl crate::traits::fields::GetGitMessage for GitInformationWithoutLifetimes {
    fn get_git_message(&self) -> String {
        self.git_message.clone()
    }
}
