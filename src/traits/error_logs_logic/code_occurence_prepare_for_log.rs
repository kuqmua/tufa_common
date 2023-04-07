pub trait CodeOccurencePrepareForLogWithConfig<
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::get_server_address::GetServerAddress,
>
{
    fn code_occurence_prepare_for_log_with_config(&self, config: &ConfigGeneric) -> String;
}

impl<'a, SelfGeneric, ConfigGeneric> CodeOccurencePrepareForLogWithConfig<ConfigGeneric>
    for SelfGeneric
where
    SelfGeneric: crate::traits::fields::GetFile
        + crate::traits::fields::GetLine
        + crate::traits::fields::GetColumn
        + crate::traits::get_code_path_without_config::GetCodePathWithoutConfig
        + crate::traits::get_duration::GetDuration
        + crate::traits::get_hostname::GetHostname
        + crate::traits::get_process_id::GetProcessId
        + crate::traits::get_git_source_file_link::GetGitSourceFileLink<'a>,
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::get_server_address::GetServerAddress,
{
    fn code_occurence_prepare_for_log_with_config(&self, config: &ConfigGeneric) -> String {
        format!(
            "{} {} {} pid: {} {}",
            config.get_source_place_type().get_code_path(self),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(&chrono::FixedOffset::east_opt(*config.get_timezone()).unwrap())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            self.get_hostname(),
            self.get_process_id(),
            config.get_server_address()
        )
    }
}

pub trait CodeOccurencePrepareForLogWithoutConfig {
    fn code_occurence_prepare_for_log_without_config(&self) -> String;
}

impl<SelfGeneric> CodeOccurencePrepareForLogWithoutConfig for SelfGeneric
where
    SelfGeneric: crate::traits::fields::GetFile
        + crate::traits::fields::GetLine
        + crate::traits::fields::GetColumn
        //above its for crate::traits::get_code_path_without_config::GetCodePathWithoutConfig
        + crate::traits::get_code_path_without_config::GetCodePathWithoutConfig
        + crate::traits::get_duration::GetDuration
        + crate::traits::get_hostname::GetHostname
        + crate::traits::get_process_id::GetProcessId,
{
    fn code_occurence_prepare_for_log_without_config(&self) -> String {
        format!(
            "{} {} {} pid: {}",
            self.get_code_path_without_config(),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(&chrono::FixedOffset::east_opt(10800).unwrap())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            self.get_hostname(),
            self.get_process_id()
        )
    }
}

pub trait CodeOccurencePrepareForLogWithoutConfigWithSerializeDeserialize {
    fn code_occurence_prepare_for_log_without_config_with_serialize_deserialize(&self) -> String;
}

impl<'a, SelfGeneric> CodeOccurencePrepareForLogWithoutConfigWithSerializeDeserialize for SelfGeneric
where
    SelfGeneric: crate::traits::fields::GetFile
        + crate::traits::fields::GetLine
        + crate::traits::fields::GetColumn
        + crate::traits::get_code_path_without_config::GetCodePathWithoutConfig
        + crate::traits::get_git_source_file_link::GetGitSourceFileLink<'a>
        + crate::traits::get_duration::GetDuration
        + crate::traits::get_hostname::GetHostname
        + crate::traits::get_process_id::GetProcessId,
{
    fn code_occurence_prepare_for_log_without_config_with_serialize_deserialize(&self) -> String {
        use crate::traits::error_logs_logic::form_error_path::FormErrorPathGithub;
        format!(
            "{} {} {} pid: {}",
            self.form_error_path_github(),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(&chrono::FixedOffset::east_opt(10800).unwrap())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            self.get_hostname(),
            self.get_process_id()
        )
    }
}
