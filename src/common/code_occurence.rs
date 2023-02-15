use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)] //Deserialize // #[serde(borrow)] - need for field with lifetime
pub struct CodeOccurence<'a> {
    file: &'a str,
    line: u32,
    column: u32,
    git_info: &'a crate::common::git::git_info::GitInformation<'a>,
    duration: std::time::Duration,
    hostname: String,
    process_id: u32,
}

impl<'a> CodeOccurence<'a> {
    pub fn new(
        git_info: &'a crate::common::git::git_info::GitInformation<'a>,
        file: &'a str,
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            file,
            line,
            column,
            git_info,
            duration: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("cannot convert time to unix_epoch"),
            hostname: match hostname::get() {
                Ok(os_string) => format!("{os_string:?}"),
                Err(_) => String::from("\"hostname::get() failed \""),
            },
            process_id: std::process::id(),
        }
    }
}

impl<'a> crate::traits::fields::GetFile for CodeOccurence<'a> {
    fn get_file(&self) -> &str {
        &self.file
    }
}

impl<'a> crate::traits::fields::GetLine for CodeOccurence<'a> {
    fn get_line(&self) -> &u32 {
        &self.line
    }
}

impl<'a> crate::traits::fields::GetColumn for CodeOccurence<'a> {
    fn get_column(&self) -> &u32 {
        &self.column
    }
}

impl<'a> crate::traits::get_git_info::GetGitInfo<'a> for CodeOccurence<'a> {
    fn get_git_info(&self) -> &crate::common::git::git_info::GitInformation {
        &self.git_info
    }
}

impl<'a> crate::traits::get_duration::GetDuration for CodeOccurence<'a> {
    fn get_duration(&self) -> std::time::Duration {
        self.duration
    }
}

impl<'a> crate::traits::get_hostname::GetHostname for CodeOccurence<'a> {
    fn get_hostname(&self) -> &String {
        &self.hostname
    }
}

impl<'a> crate::traits::get_process_id::GetProcessId for CodeOccurence<'a> {
    fn get_process_id(&self) -> &u32 {
        &self.process_id
    }
}

impl<'a> std::fmt::Display for crate::common::code_occurence::CodeOccurence<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::code_occurence_prepare_for_log::CodeOccurencePrepareForLogWithoutConfig;
        write!(
            f,
            "{}",
            self.code_occurence_prepare_for_log_without_config()
        )
    }
}

impl<'a> crate::traits::get_git_source_file_link::GetGitSourceFileLinkLifetime<'a>
    for crate::common::code_occurence::CodeOccurence<'a>
{
    fn get_git_source_file_link_lifetime(&self, file: &str, line: u32) -> String {
        self.git_info.get_git_source_file_link_lifetime(file, line)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeOccurenceWithDeserialize<'a> {
    file: &'a str,
    line: u32,
    column: u32,
    git_info: crate::common::git::git_info::GitInformation<'a>,
    duration: std::time::Duration,
    hostname: String,
    process_id: u32,
}

impl<'a> CodeOccurenceWithDeserialize<'a> {
    pub fn new(
        git_info: &'a crate::common::git::git_info::GitInformation<'a>,
        file: &'a str,
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            file,
            line,
            column,
            git_info: git_info.clone(),
            duration: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("cannot convert time to unix_epoch"),
            hostname: match hostname::get() {
                Ok(os_string) => format!("{os_string:?}"),
                Err(_) => String::from("\"hostname::get() failed \""),
            },
            process_id: std::process::id(),
        }
    }
}

impl<'a> crate::traits::fields::GetFile for CodeOccurenceWithDeserialize<'a> {
    fn get_file(&self) -> &str {
        &self.file
    }
}

impl<'a> crate::traits::fields::GetLine for CodeOccurenceWithDeserialize<'a> {
    fn get_line(&self) -> &u32 {
        &self.line
    }
}

impl<'a> crate::traits::fields::GetColumn for CodeOccurenceWithDeserialize<'a> {
    fn get_column(&self) -> &u32 {
        &self.column
    }
}

impl<'a> crate::traits::get_git_info::GetGitInfo<'a> for CodeOccurenceWithDeserialize<'a> {
    fn get_git_info(&self) -> &crate::common::git::git_info::GitInformation {
        &self.git_info
    }
}

impl<'a> crate::traits::get_duration::GetDuration for CodeOccurenceWithDeserialize<'a> {
    fn get_duration(&self) -> std::time::Duration {
        self.duration
    }
}

impl<'a> crate::traits::get_hostname::GetHostname for CodeOccurenceWithDeserialize<'a> {
    fn get_hostname(&self) -> &String {
        &self.hostname
    }
}

impl<'a> crate::traits::get_process_id::GetProcessId for CodeOccurenceWithDeserialize<'a> {
    fn get_process_id(&self) -> &u32 {
        &self.process_id
    }
}

impl<'a> std::fmt::Display for crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::code_occurence_prepare_for_log::CodeOccurencePrepareForLogWithoutConfigWithDeserialize;
        write!(
            f,
            "{}",
            self.code_occurence_prepare_for_log_without_config_with_deserialize()
        )
    }
}

impl<'a> crate::traits::get_git_source_file_link::GetGitSourceFileLinkLifetime<'a>
    for crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>
{
    fn get_git_source_file_link_lifetime(&self, file: &str, line: u32) -> String {
        self.git_info.get_git_source_file_link_lifetime(file, line)
    }
}
