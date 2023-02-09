pub trait FormErrorPathDirectory {
    fn form_error_path_directory(&self) -> String;
}

impl<SelfGeneric> FormErrorPathDirectory for SelfGeneric
where
    SelfGeneric: crate::traits::fields::GetFile
        + crate::traits::fields::GetLine
        + crate::traits::fields::GetColumn,
{
    fn form_error_path_directory(&self) -> String {
        format!(
            "src/{}:{}:{}", //todo "src" - hardcode, for some reason vscode stops following just {}:{}:{} path(without prefix "src")
            self.get_file(),
            self.get_line(),
            self.get_column()
        )
    }
}

pub trait FormErrorPathGithub {
    fn form_error_path_github(&self) -> String;
}

impl<SelfGeneric> FormErrorPathGithub for SelfGeneric
where
    SelfGeneric: crate::traits::fields::GetFile
        + crate::traits::fields::GetLine
        + crate::traits::fields::GetColumn
        + crate::traits::get_git_info::GetClonedGitInfo,
{
    fn form_error_path_github(&self) -> String {
        use crate::traits::get_git_source_file_link::GetGitSourceFileLinkLifetime;
        let backslash = "/";
        let file = self.get_file();
        match file.find(backslash) {
            Some(index) => self
                .get_cloned_git_info()
                .get_git_source_file_link_lifetime(
                    &file[index + backslash.len()..],
                    *self.get_line(),
                ),
            None => String::from("cant find backslash symbol in file path of location"),
        }
    }
}
