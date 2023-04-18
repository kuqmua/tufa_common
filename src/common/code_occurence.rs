#[derive(Debug, serde::Serialize)]
pub struct CodeOccurence<'a> {
    file: &'a str,
    line: u32,
    column: u32,
    git_info: &'a crate::common::git::git_info::GitInformation<'a>,
    duration: std::time::Duration,
    process_id: u32,
    server_port: u16,
}

impl<'a> CodeOccurence<'a> {
    pub fn new(
        git_info: &'a crate::common::git::git_info::GitInformation<'a>,
        file: &'a str,
        line: u32,
        column: u32,
        server_port: u16,
    ) -> Self {
        Self {
            file,
            line,
            column,
            git_info,
            duration: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("cannot convert time to unix_epoch"),
            process_id: std::process::id(),
            server_port,
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

impl<'a> crate::traits::get_process_id::GetProcessId for CodeOccurence<'a> {
    fn get_process_id(&self) -> &u32 {
        &self.process_id
    }
}

impl<'a> crate::traits::fields::GetServerPort for CodeOccurence<'a> {
    fn get_server_port(&self) -> &u16 {
        &self.server_port
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

impl<'a> crate::traits::get_git_source_file_link::GetGitSourceFileLink<'a>
    for crate::common::code_occurence::CodeOccurence<'a>
{
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        self.git_info.get_git_source_file_link(file, line)
    }
}

impl<'a> CodeOccurence<'a> {
    pub fn into_serialize_deserialize_version(self) -> CodeOccurenceWithSerializeDeserialize<'a> {
        CodeOccurenceWithSerializeDeserialize {
            file: self.file,
            line: self.line,
            column: self.column,
            git_info: self.git_info.clone(),
            duration: self.duration,
            process_id: self.process_id,
            server_port: self.server_port,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeOccurenceWithSerializeDeserialize<'a> {
    file: &'a str,
    line: u32,
    column: u32,
    git_info: crate::common::git::git_info::GitInformation<'a>,
    duration: std::time::Duration,
    process_id: u32,
    server_port: u16,
}

impl<'a> CodeOccurenceWithSerializeDeserialize<'a> {
    pub fn new(
        git_info: &'a crate::common::git::git_info::GitInformation<'a>,
        file: &'a str,
        line: u32,
        column: u32,
        server_port: u16,
    ) -> Self {
        Self {
            file,
            line,
            column,
            git_info: git_info.clone(),
            duration: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("cannot convert time to unix_epoch"),
            process_id: std::process::id(),
            server_port,
        }
    }
}

impl<'a> crate::traits::fields::GetFile for CodeOccurenceWithSerializeDeserialize<'a> {
    fn get_file(&self) -> &str {
        &self.file
    }
}

impl<'a> crate::traits::fields::GetLine for CodeOccurenceWithSerializeDeserialize<'a> {
    fn get_line(&self) -> &u32 {
        &self.line
    }
}

impl<'a> crate::traits::fields::GetColumn for CodeOccurenceWithSerializeDeserialize<'a> {
    fn get_column(&self) -> &u32 {
        &self.column
    }
}

impl<'a> crate::traits::get_git_info::GetGitInfo<'a> for CodeOccurenceWithSerializeDeserialize<'a> {
    fn get_git_info(&self) -> &crate::common::git::git_info::GitInformation {
        &self.git_info
    }
}

impl<'a> crate::traits::get_duration::GetDuration for CodeOccurenceWithSerializeDeserialize<'a> {
    fn get_duration(&self) -> std::time::Duration {
        self.duration
    }
}

impl<'a> crate::traits::get_process_id::GetProcessId for CodeOccurenceWithSerializeDeserialize<'a> {
    fn get_process_id(&self) -> &u32 {
        &self.process_id
    }
}

impl<'a> crate::traits::fields::GetServerPort for CodeOccurenceWithSerializeDeserialize<'a> {
    fn get_server_port(&self) -> &u16 {
        &self.server_port
    }
}

impl<'a> std::fmt::Display for crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::code_occurence_prepare_for_log::CodeOccurencePrepareForLogWithoutConfigWithSerializeDeserialize;//here
        write!(
            f,
            "{}",
            self.code_occurence_prepare_for_log_without_config_with_serialize_deserialize()
        )
    }
}

impl<'a> crate::traits::get_git_source_file_link::GetGitSourceFileLink<'a>
    for crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize<'a>
{
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        self.git_info.get_git_source_file_link(file, line)
    }
}
