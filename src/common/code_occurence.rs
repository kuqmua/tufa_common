//todo use std::sync::Arc<crate::common::git::git_info::GitInformationWithoutLifetimes> ?
use crate::traits::code_path::CodePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeOccurenceOldWay {
    pub git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
    pub pid_hostname_time_file_line_column:
        crate::common::pid_hostname_time_file_line_column::PidHostnameTimeFileLineColumn,
}

impl<ConfigGeneric> crate::traits::error_display::ToStringHandle<ConfigGeneric>
    for crate::common::code_occurence::CodeOccurenceOldWay
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        format!(
            "{} {} on {} {} pid: {}",
            self.get_code_path(config.get_source_place_type()),
            chrono::DateTime::<chrono::Utc>::from(
                std::time::UNIX_EPOCH + self.pid_hostname_time_file_line_column.time,
            )
            .with_timezone(&chrono::FixedOffset::east_opt(*config.get_timezone()).unwrap())
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
            config.get_server_address(),
            self.pid_hostname_time_file_line_column.hostname,
            self.pid_hostname_time_file_line_column.process_id,
        )
    }
}

impl crate::traits::get_hostname::GetHostname for CodeOccurenceOldWay {
    fn get_hostname(&self) -> &String {
        self.pid_hostname_time_file_line_column.get_hostname()
    }
}

impl crate::traits::get_git_info::GetGitInfoWithoutLifetimes for CodeOccurenceOldWay {
    fn get_git_info_without_lifetimes(
        &self,
    ) -> &crate::common::git::git_info::GitInformationWithoutLifetimes {
        &self.git_info
    }
}

impl crate::traits::fields::GetFile for CodeOccurenceOldWay {
    fn get_file(&self) -> &String {
        &self.pid_hostname_time_file_line_column.get_file()
    }
}

impl crate::traits::fields::GetLine for CodeOccurenceOldWay {
    fn get_line(&self) -> &u32 {
        &self.pid_hostname_time_file_line_column.get_line()
    }
}

impl crate::traits::fields::GetColumn for CodeOccurenceOldWay {
    fn get_column(&self) -> &u32 {
        &self.pid_hostname_time_file_line_column.get_column()
    }
}

#[derive(Debug, Clone)]
pub struct CodeOccurence {
    pub occurences: std::collections::HashMap<
        crate::common::git::git_info::GitInformationWithoutLifetimes,
        Vec<crate::common::pid_hostname_time_file_line_column::PidHostnameTimeFileLineColumn>,
    >,
}
