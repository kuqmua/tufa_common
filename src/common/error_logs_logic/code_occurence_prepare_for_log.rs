pub trait CodeOccurencePrepareForLogWithConfig {
    fn code_occurence_prepare_for_log_with_config<
        ConfigGeneric: crate::common::config::config_fields::GetTimezone
            + crate::common::config::config_fields::GetSourcePlaceType
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> String;
}

impl<'a, SelfGeneric> CodeOccurencePrepareForLogWithConfig for SelfGeneric
where
    SelfGeneric: error_occurence_lib::get_file::GetFile
        + error_occurence_lib::get_line::GetLine
        + crate::common::error_logs_logic::get_column::GetColumn
        + crate::common::error_logs_logic::get_duration::GetDuration
        + crate::common::git::get_git_source_file_link::GetGitSourceFileLink<'a>,
{
    fn code_occurence_prepare_for_log_with_config<
        ConfigGeneric: crate::common::config::config_fields::GetTimezone
            + crate::common::config::config_fields::GetSourcePlaceType
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> String {
        prepare_for_log(
            config.get_source_place_type().get_code_path(self),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(config.get_timezone())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        )
    }
}

pub trait CodeOccurencePrepareForLogWithoutConfig {
    fn code_occurence_prepare_for_log_without_config(&self) -> String;
}

impl<SelfGeneric> CodeOccurencePrepareForLogWithoutConfig for SelfGeneric
where
    SelfGeneric: crate::common::error_logs_logic::form_error_path::FormErrorPathGithub
        + crate::common::error_logs_logic::get_duration::GetDuration,
{
    fn code_occurence_prepare_for_log_without_config(&self) -> String {
        prepare_for_log(
            self.form_error_path_github(),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(&chrono::FixedOffset::east_opt(10800).unwrap())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        )
    }
}

pub trait CodeOccurencePrepareForLogWithoutConfigWithSerializeDeserialize {
    fn code_occurence_prepare_for_log_without_config_with_serialize_deserialize(&self) -> String;
}

impl<'a, SelfGeneric> CodeOccurencePrepareForLogWithoutConfigWithSerializeDeserialize
    for SelfGeneric
where
    SelfGeneric: crate::common::error_logs_logic::form_error_path::FormErrorPathGithub
        + crate::common::error_logs_logic::get_duration::GetDuration,
{
    fn code_occurence_prepare_for_log_without_config_with_serialize_deserialize(&self) -> String {
        prepare_for_log(
            self.form_error_path_github(),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(&chrono::FixedOffset::east_opt(10800).unwrap())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        )
    }
}

fn prepare_for_log(path: String, time: String) -> String {
    format!("{path} {time}")
}
