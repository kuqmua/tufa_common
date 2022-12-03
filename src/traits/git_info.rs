use crate::common::git::git_info::GitInformation;
use crate::common::git::git_info::GitInformationWithoutLifetimes;
pub trait GetCommitId {
    fn get_commit_id(&self) -> String;
}

impl GetCommitId for GitInformation<'_> {
    fn get_commit_id(&self) -> String {
        String::from(self.commit_id)
    }
}

impl GetCommitId for GitInformationWithoutLifetimes {
    fn get_commit_id(&self) -> String {
        self.commit_id.clone()
    }
}

pub trait GetRepoLink {
    fn get_repo_link(&self) -> String;
}

impl GetRepoLink for GitInformation<'_> {
    fn get_repo_link(&self) -> String {
        String::from(self.repo_link)
    }
}

impl GetRepoLink for GitInformationWithoutLifetimes {
    fn get_repo_link(&self) -> String {
        self.repo_link.clone()
    }
}

pub trait GetAuthor {
    fn get_author(&self) -> String;
}

impl GetAuthor for GitInformation<'_> {
    fn get_author(&self) -> String {
        String::from(self.author)
    }
}

impl GetAuthor for GitInformationWithoutLifetimes {
    fn get_author(&self) -> String {
        self.author.clone()
    }
}

pub trait GetAuthorEmail {
    fn get_author_email(&self) -> String;
}

impl GetAuthorEmail for GitInformation<'_> {
    fn get_author_email(&self) -> String {
        String::from(self.author_email)
    }
}

impl GetAuthorEmail for GitInformationWithoutLifetimes {
    fn get_author_email(&self) -> String {
        self.author_email.clone()
    }
}

pub trait GetCommitUnixTime {
    fn get_commit_unix_time(&self) -> String;
}

impl GetCommitUnixTime for GitInformation<'_> {
    fn get_commit_unix_time(&self) -> String {
        String::from(self.commit_unix_time)
    }
}

impl GetCommitUnixTime for GitInformationWithoutLifetimes {
    fn get_commit_unix_time(&self) -> String {
        self.commit_unix_time.clone()
    }
}

pub trait GetTimezone {
    fn get_timezone(&self) -> String;
}

impl GetTimezone for GitInformation<'_> {
    fn get_timezone(&self) -> String {
        String::from(self.timezone)
    }
}

impl GetTimezone for GitInformationWithoutLifetimes {
    fn get_timezone(&self) -> String {
        self.timezone.clone()
    }
}

pub trait GetMessage {
    fn get_message(&self) -> String;
}

impl GetMessage for GitInformation<'_> {
    fn get_message(&self) -> String {
        String::from(self.message)
    }
}

impl GetMessage for GitInformationWithoutLifetimes {
    fn get_message(&self) -> String {
        self.message.clone()
    }
}
