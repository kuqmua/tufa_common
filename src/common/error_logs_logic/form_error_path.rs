pub trait FormErrorPathDirectory {
    fn form_error_path_directory(&self) -> String;
}

impl<SelfGeneric> FormErrorPathDirectory for SelfGeneric
where
    SelfGeneric: error_occurence_lib::get_file::GetFile
        + error_occurence_lib::get_line::GetLine
        + error_occurence_lib::get_column::GetColumn,
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

impl<'a, SelfGeneric> FormErrorPathGithub for SelfGeneric
where
    SelfGeneric: error_occurence_lib::get_file::GetFile
        + error_occurence_lib::get_line::GetLine
        + error_occurence_lib::get_column::GetColumn
        + crate::common::git::get_git_source_file_link::GetGitSourceFileLink<'a>,
{
    fn form_error_path_github(&self) -> String {
        let backslash = "/";
        let file = self.get_file();
        match file.find(backslash) {
            Some(index) => {
                self.get_git_source_file_link(&file[index + backslash.len()..], *self.get_line())
            }
            None => String::from("cant find backslash symbol in file path of location"),
        }
    }
}
