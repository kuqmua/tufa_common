//todo use std::sync::Arc<crate::common::git::git_info::GitInformationWithoutLifetimes> ?
use crate::traits::code_path::CodePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeOccurenceOldWay {
    pub file_line_column: crate::common::file_line_column::FileLineColumn,
    pub git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
    pub time: std::time::Duration,
    pub hostname: String,
    pub process_id: u32,
}

impl CodeOccurenceOldWay {
    pub fn new(
        git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self {
        let hostname_handle = match hostname::get() {
            Ok(os_string) => format!("{os_string:?}"),
            Err(_) => String::from("\"hostname::get() failed \""),
        };
        Self {
            file_line_column: crate::common::file_line_column::FileLineColumn {
                file,
                line,
                column,
            },
            git_info,
            time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("cannot convert time to unix_epoch"),
            hostname: hostname_handle,
            process_id: std::process::id(),
        }
    }
}

impl std::fmt::Display for crate::common::code_occurence::CodeOccurenceOldWay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //todo - its copypaste - remove it later
        let code_path = format!(
            "src/{}:{}:{}", //todo "src" - hardcode, for some reason vscode stops following just {}:{}:{} path(without prefix "src")
            self.file_line_column.file, self.file_line_column.line, self.file_line_column.column
        );
        write!(
            f,
            "{} {} {} pid: {}",
            code_path,
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.time,)
                .with_timezone(&chrono::FixedOffset::east_opt(10800).unwrap())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            self.hostname,
            self.process_id
        )
    }
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandleCodeOccurence<ConfigGeneric>
    for crate::common::code_occurence::CodeOccurenceOldWay
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle_code_occurence(&self, config: &ConfigGeneric) -> String {
        format!(
            "{} {} on {} {} pid: {}",
            self.get_code_path(config.get_source_place_type()),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.time,)
                .with_timezone(&chrono::FixedOffset::east_opt(*config.get_timezone()).unwrap())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            config.get_server_address(),
            self.hostname,
            self.process_id,
        )
    }
}

impl crate::traits::fields::GetFile for CodeOccurenceOldWay {
    fn get_file(&self) -> &String {
        self.file_line_column.get_file()
    }
}

impl crate::traits::fields::GetLine for CodeOccurenceOldWay {
    fn get_line(&self) -> &u32 {
        self.file_line_column.get_line()
    }
}

impl crate::traits::fields::GetColumn for CodeOccurenceOldWay {
    fn get_column(&self) -> &u32 {
        self.file_line_column.get_column()
    }
}

impl crate::traits::get_git_info::GetGitInfoWithoutLifetimes for CodeOccurenceOldWay {
    fn get_git_info_without_lifetimes(
        &self,
    ) -> &crate::common::git::git_info::GitInformationWithoutLifetimes {
        &self.git_info
    }
}

impl crate::traits::get_time::GetTime for CodeOccurenceOldWay {
    fn get_time(&self) -> std::time::Duration {
        self.time
    }
}

impl crate::traits::get_hostname::GetHostname for CodeOccurenceOldWay {
    fn get_hostname(&self) -> &String {
        &self.hostname
    }
}

impl crate::traits::get_process_id::GetProcessId for CodeOccurenceOldWay {
    fn get_process_id(&self) -> &u32 {
        &self.process_id
    }
}

#[derive(Debug, Clone)]
pub struct CodeOccurence {
    pub occurences: std::collections::HashMap<
        crate::common::git::git_info::GitInformationWithoutLifetimes,
        Vec<crate::common::pid_hostname_time_file_line_column::PidHostnameTimeFileLineColumn>,
    >,
}
