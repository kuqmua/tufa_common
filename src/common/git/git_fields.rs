pub trait GetGitCommitId<'a> {
    fn get_git_commit_id(&self) -> &'a str;
}

pub trait GetGitRepoLink<'a> {
    fn get_git_repo_link(&self) -> &'a str;
}

// pub trait GetGitAuthor<'a> {
//     fn get_git_author(&self) -> &'a str;
// }

// pub trait GetGitAuthorEmail<'a> {
//     fn get_git_author_email(&self) -> &'a str;
// }

// pub trait GetGitCommitUnixTime<'a> {
//     fn get_git_commit_unix_time(&self) -> &'a str;
// }

// pub trait GetGitTimezone<'a> {
//     fn get_git_timezone(&self) -> &'a str;
// }

// pub trait GetGitMessage<'a> {
//     fn get_git_message(&self) -> &'a str;
// }
