#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CodeOccurence {
    file: std::string::String,
    line: u32,
    column: u32,
    git_info: crate::common::git::git_info::GitInfoWithoutLifetime,
    duration: std::time::Duration,
}

impl CodeOccurence {
    #[must_use]
    pub fn new(
        git_info: crate::common::git::git_info::GitInfoWithoutLifetime,
        file: std::string::String,
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
        }
    }
}

impl crate::common::error_logs_logic::get_file::GetFile for CodeOccurence {
    fn get_file(&self) -> &str {
        &self.file
    }
}

impl crate::common::error_logs_logic::get_line::GetLine for CodeOccurence {
    fn get_line(&self) -> &u32 {
        &self.line
    }
}

impl crate::common::error_logs_logic::get_column::GetColumn for CodeOccurence {
    fn get_column(&self) -> &u32 {
        &self.column
    }
}

impl crate::common::error_logs_logic::get_duration::GetDuration for CodeOccurence {
    fn get_duration(&self) -> std::time::Duration {
        self.duration
    }
}

impl std::fmt::Display for crate::common::code_occurence::CodeOccurence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            crate::common::error_logs_logic::code_occurence_prepare_for_log::CodeOccurencePrepareForLogWithoutConfig::code_occurence_prepare_for_log_without_config(self)
        )
    }
}

impl<'a> crate::common::git::get_git_source_file_link::GetGitSourceFileLink<'a>
    for crate::common::code_occurence::CodeOccurence
{
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        self.git_info.get_git_source_file_link(file, line)
    }
}

////////////////////////

impl CodeOccurence {
    pub fn into_serialize_deserialize_version(self) -> CodeOccurenceWithSerializeDeserialize {
        CodeOccurenceWithSerializeDeserialize::new(
            self.git_info,
            &self.file,
            self.line,
            self.column,
        )
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CodeOccurenceWithSerializeDeserialize {
    file: std::string::String,
    line: u32,
    column: u32,
    git_info: crate::common::git::git_info::GitInfoWithoutLifetime,
    duration: std::time::Duration,
}

impl<'a> CodeOccurenceWithSerializeDeserialize {
    #[must_use]
    pub fn new(
        git_info: crate::common::git::git_info::GitInfoWithoutLifetime,
        file: &'a str,
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            file: file.to_string(),
            line,
            column,
            git_info,
            duration: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("cannot convert time to unix_epoch"),
        }
    }
}

impl crate::common::error_logs_logic::get_file::GetFile for CodeOccurenceWithSerializeDeserialize {
    fn get_file(&self) -> &str {
        &self.file
    }
}

impl crate::common::error_logs_logic::get_line::GetLine for CodeOccurenceWithSerializeDeserialize {
    fn get_line(&self) -> &u32 {
        &self.line
    }
}

impl crate::common::error_logs_logic::get_column::GetColumn
    for CodeOccurenceWithSerializeDeserialize
{
    fn get_column(&self) -> &u32 {
        &self.column
    }
}

impl crate::common::error_logs_logic::get_duration::GetDuration
    for CodeOccurenceWithSerializeDeserialize
{
    fn get_duration(&self) -> std::time::Duration {
        self.duration
    }
}

impl std::fmt::Display for crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            crate::common::error_logs_logic::code_occurence_prepare_for_log::CodeOccurencePrepareForLogWithoutConfigWithSerializeDeserialize::code_occurence_prepare_for_log_without_config_with_serialize_deserialize(self)
        )
    }
}

impl<'a> crate::common::git::get_git_source_file_link::GetGitSourceFileLink<'a>
    for crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize
{
    fn get_git_source_file_link(&self, file: &str, line: u32) -> String {
        self.git_info.get_git_source_file_link(file, line)
    }
}
