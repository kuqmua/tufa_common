use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)] //Deserialize // #[serde(borrow)] - need for field with lifetime
pub struct CodeOccurenceLifetime<'a> {
    file: &'a str,
    line: u32,
    column: u32,
    git_info: &'a crate::common::git::git_info::GitInformation<'a>,
    duration: std::time::Duration,
    hostname: String,
    process_id: u32,
}

impl<'a> CodeOccurenceLifetime<'a> {
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

impl<'a> crate::traits::fields::GetFile for CodeOccurenceLifetime<'a> {
    fn get_file(&self) -> &str {
        &self.file
    }
}

impl<'a> crate::traits::fields::GetLine for CodeOccurenceLifetime<'a> {
    fn get_line(&self) -> &u32 {
        &self.line
    }
}

impl<'a> crate::traits::fields::GetColumn for CodeOccurenceLifetime<'a> {
    fn get_column(&self) -> &u32 {
        &self.column
    }
}

impl<'a> crate::traits::get_git_info::GetGitInfo<'a> for CodeOccurenceLifetime<'a> {
    fn get_git_info(&self) -> &crate::common::git::git_info::GitInformation {
        &self.git_info
    }
}

impl<'a> crate::traits::get_duration::GetDuration for CodeOccurenceLifetime<'a> {
    fn get_duration(&self) -> std::time::Duration {
        self.duration
    }
}

impl<'a> crate::traits::get_hostname::GetHostname for CodeOccurenceLifetime<'a> {
    fn get_hostname(&self) -> &String {
        &self.hostname
    }
}

impl<'a> crate::traits::get_process_id::GetProcessId for CodeOccurenceLifetime<'a> {
    fn get_process_id(&self) -> &u32 {
        &self.process_id
    }
}

impl<'a> std::fmt::Display for crate::common::code_occurence::CodeOccurenceLifetime<'a> {
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
    for crate::common::code_occurence::CodeOccurenceLifetime<'a>
{
    fn get_git_source_file_link_lifetime(&self, file: &str, line: u32) -> String {
        self.git_info.get_git_source_file_link_lifetime(file, line)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeOccurenceLifetimeWithDeserialize<'a> {
    file: &'a str,
    line: u32,
    column: u32,
    git_info: crate::common::git::git_info::GitInformation<'a>,
    duration: std::time::Duration,
    hostname: String,
    process_id: u32,
}

impl<'a> CodeOccurenceLifetimeWithDeserialize<'a> {
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

impl<'a> crate::traits::fields::GetFile for CodeOccurenceLifetimeWithDeserialize<'a> {
    fn get_file(&self) -> &str {
        &self.file
    }
}

impl<'a> crate::traits::fields::GetLine for CodeOccurenceLifetimeWithDeserialize<'a> {
    fn get_line(&self) -> &u32 {
        &self.line
    }
}

impl<'a> crate::traits::fields::GetColumn for CodeOccurenceLifetimeWithDeserialize<'a> {
    fn get_column(&self) -> &u32 {
        &self.column
    }
}

impl<'a> crate::traits::get_git_info::GetGitInfo<'a> for CodeOccurenceLifetimeWithDeserialize<'a> {
    fn get_git_info(&self) -> &crate::common::git::git_info::GitInformation {
        &self.git_info
    }
}

impl<'a> crate::traits::get_duration::GetDuration for CodeOccurenceLifetimeWithDeserialize<'a> {
    fn get_duration(&self) -> std::time::Duration {
        self.duration
    }
}

impl<'a> crate::traits::get_hostname::GetHostname for CodeOccurenceLifetimeWithDeserialize<'a> {
    fn get_hostname(&self) -> &String {
        &self.hostname
    }
}

impl<'a> crate::traits::get_process_id::GetProcessId for CodeOccurenceLifetimeWithDeserialize<'a> {
    fn get_process_id(&self) -> &u32 {
        &self.process_id
    }
}

impl<'a> std::fmt::Display
    for crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a>
{
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
    for crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a>
{
    fn get_git_source_file_link_lifetime(&self, file: &str, line: u32) -> String {
        self.git_info.get_git_source_file_link_lifetime(file, line)
    }
}
