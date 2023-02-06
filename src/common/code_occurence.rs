//todo use std::sync::Arc<crate::common::git::git_info::GitInformationWithoutLifetimes> ?
// use crate::traits::code_path::CodePathLifetime;
use crate::traits::get_code_path_without_config::GetCodePathWithoutConfig;
use crate::traits::get_duration::GetDuration;
use crate::traits::get_hostname::GetHostname;
use crate::traits::get_process_id::GetProcessId;
use serde::{Deserialize, Serialize};
use std::os::unix::process;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct CodeOccurence<'a> {
//     file: &'a str,
//     line: u32,
//     column: u32,
//     git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
//     duration: std::time::Duration,
//     hostname: String,
//     process_id: u32,
// }

// impl<'a> CodeOccurence<'a> {
//     pub fn new(
//         git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
//         file: &'a str,
//         line: u32,
//         column: u32,
//     ) -> Self {
//         let hostname_handle = match hostname::get() {
//             Ok(os_string) => format!("{os_string:?}"),
//             Err(_) => String::from("\"hostname::get() failed \""),
//         };
//         Self {
//             file,
//             line,
//             column,
//             git_info,
//             duration: std::time::SystemTime::now()
//                 .duration_since(std::time::UNIX_EPOCH)
//                 .expect("cannot convert time to unix_epoch"),
//             hostname: hostname_handle,
//             process_id: std::process::id(),
//         }
//     }
//     pub fn prepare_for_log(
//         &self,
//         path: String,
//         time: String,
//         hostname: &String,
//         process_id: &u32,
//     ) -> String {
//         format!("{} {} {} pid: {}", path, time, hostname, process_id,)
//     }
// }

// impl<'a> std::fmt::Display for crate::common::code_occurence::CodeOccurence<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             self.prepare_for_log(
//                 self.get_project_code_path(),
//                 chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
//                     .with_timezone(&chrono::FixedOffset::east_opt(10800).unwrap())
//                     .format("%Y-%m-%d %H:%M:%S")
//                     .to_string(),
//                 self.get_hostname(),
//                 self.get_process_id()
//             )
//         )
//     }
// }

// impl<'a, ConfigGeneric>
//     crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime<
//         'a,
//         ConfigGeneric,
//     > for crate::common::code_occurence::CodeOccurence<'a>
// where
//     ConfigGeneric: crate::traits::fields::GetTimezone
//         + crate::traits::fields::GetSourcePlaceType
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
//         format!(
//             "{} {}",
//             self.prepare_for_log(
//                 self.get_code_path(config.get_source_place_type()),
//                 chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
//                     .with_timezone(&chrono::FixedOffset::east_opt(*config.get_timezone()).unwrap())
//                     .format("%Y-%m-%d %H:%M:%S")
//                     .to_string(),
//                 self.get_hostname(),
//                 self.get_process_id(),
//             ),
//             config.get_server_address(),
//         )
//     }
// }

// impl<'a> crate::traits::fields::GetFile for CodeOccurence<'a> {
//     fn get_file(&self) -> &str {
//         &self.file
//     }
// }

// impl<'a> crate::traits::fields::GetLine for CodeOccurence<'a> {
//     fn get_line(&self) -> &u32 {
//         &self.line
//     }
// }

// impl<'a> crate::traits::fields::GetColumn for CodeOccurence<'a> {
//     fn get_column(&self) -> &u32 {
//         &self.column
//     }
// }

// impl<'a> crate::traits::get_git_info::GetGitInfoWithoutLifetimes for CodeOccurence<'a> {
//     fn get_git_info_without_lifetimes(
//         &self,
//     ) -> &crate::common::git::git_info::GitInformationWithoutLifetimes {
//         &self.git_info
//     }
// }

// impl<'a> crate::traits::get_duration::GetDuration for CodeOccurence<'a> {
//     fn get_duration(&self) -> std::time::Duration {
//         self.duration
//     }
// }

// impl<'a> crate::traits::get_hostname::GetHostname for CodeOccurence<'a> {
//     fn get_hostname(&self) -> &String {
//         &self.hostname
//     }
// }

// impl<'a> crate::traits::get_process_id::GetProcessId for CodeOccurence<'a> {
//     fn get_process_id(&self) -> &u32 {
//         &self.process_id
//     }
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeOccurenceLifetime<'a> {
    file: &'a str,
    line: u32,
    column: u32,
    #[serde(borrow)]
    git_info: crate::common::git::git_info::GitInformation<'a>,
    duration: std::time::Duration,
    hostname: String,
    process_id: u32,
}

impl<'a> CodeOccurenceLifetime<'a> {
    pub fn new(
        git_info: crate::common::git::git_info::GitInformation<'a>,//todo - maybe create trait what returns git_info in tufa_common, but implementation create inside tufa_server and others services
        file: &'a str,
        line: u32,
        column: u32,
    ) -> Self {
        let hostname_handle = match hostname::get() {
            Ok(os_string) => format!("{os_string:?}"),
            Err(_) => String::from("\"hostname::get() failed \""),
        };
        Self {
            file,
            line,
            column,
            git_info,
            duration: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("cannot convert time to unix_epoch"),
            hostname: hostname_handle,
            process_id: std::process::id(),
        }
    }
    pub fn prepare_for_log(
        &self,
        path: String,
        time: String,
        hostname: &String,
        process_id: &u32,
    ) -> String {
        format!("{} {} {} pid: {}", path, time, hostname, process_id)
    }
}

//
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

//

// + crate::traits::get_git_source_file_link::GetGitSourceFileLinkLifetime<'a>,
//

use crate::traits::fields::GetColumn;
use crate::traits::fields::GetFile;
use crate::traits::fields::GetLine;
use crate::traits::get_git_info::GetGitInfo;
use crate::traits::get_git_source_file_link::GetGitSourceFileLinkLifetime;

impl<'a> std::fmt::Display for crate::common::code_occurence::CodeOccurenceLifetime<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.prepare_for_log(
                self.get_code_path_without_config(),
                chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                    .with_timezone(&chrono::FixedOffset::east_opt(10800).unwrap())
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                self.get_hostname(),
                self.get_process_id()
            )
        )
    }
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for crate::common::code_occurence::CodeOccurenceLifetime<'a>
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::get_server_address::GetServerAddress,
    // ,
    //
    // + crate::traits::get_git_info::GetGitInfo,
{
    fn to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        //     fn get_code_path_lifetime(&self, source_place_type: &SourcePlaceType) -> String {

        //     }
        let f = match config.get_source_place_type() {
            crate::config_mods::source_place_type::SourcePlaceType::Source => format!(
                "src/{}:{}:{}", //todo "src" - hardcode, for some reason vscode stops following just {}:{}:{} path(without prefix "src")
                self.get_file(),
                self.get_line(),
                self.get_column()
            ),
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                let backslash = "/";
                let file = self.get_file();
                match file.find(backslash) {
                    Some(index) => {
                        let f = &self.git_info;
                        let g = f.get_git_source_file_link_lifetime(
                            &file[index + backslash.len()..],
                            *self.get_line(),
                        );
                        g
                    }
                    None => String::from("cant find backslash symbol in file path of location"),
                }
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => String::from(""), //todo maybe incorrect?
        };
        format!(
            "{} {}",
            self.prepare_for_log(
                // config.get_code_path_lifetime(config.get_source_place_type()),
                f,
                // String::from("kekw"),
                chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                    .with_timezone(&chrono::FixedOffset::east_opt(*config.get_timezone()).unwrap())
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                self.get_hostname(),
                self.get_process_id(),
            ),
            config.get_server_address(),
        )
    }
}
