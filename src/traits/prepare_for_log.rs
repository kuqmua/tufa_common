pub trait PrepareForLogWithConfig {
    fn prepare_for_log_with_config(
        &self,
        config: impl crate::traits::fields::GetTimezone,
    ) -> String;
}

impl<SelfGeneric> PrepareForLogWithConfig for SelfGeneric
where
    SelfGeneric: crate::traits::fields::GetFile
        + crate::traits::fields::GetLine
        + crate::traits::fields::GetColumn
        + crate::traits::get_code_path_without_config::GetCodePathWithoutConfig
        + crate::traits::get_duration::GetDuration
        + crate::traits::get_hostname::GetHostname
        + crate::traits::get_process_id::GetProcessId,
{
    fn prepare_for_log_with_config(
        &self,
        config: impl crate::traits::fields::GetTimezone,
    ) -> String {
        format!(
            "{} {} {} pid: {}",
            self.get_code_path_without_config(),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(&chrono::FixedOffset::east_opt(*config.get_timezone()).unwrap())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            self.get_hostname(),
            self.get_process_id()
        )
    }
}

pub trait PrepareForLogWithoutConfig {
    fn prepare_for_log_without_config(&self) -> String;
}

impl<SelfGeneric> PrepareForLogWithoutConfig for SelfGeneric
where
    SelfGeneric: crate::traits::fields::GetFile
        + crate::traits::fields::GetLine
        + crate::traits::fields::GetColumn
        //above its for crate::traits::get_code_path_without_config::GetCodePathWithoutConfig
        + crate::traits::get_code_path_without_config::GetCodePathWithoutConfig
        + crate::traits::get_duration::GetDuration //todo - think about
        + crate::traits::get_hostname::GetHostname
        + crate::traits::get_process_id::GetProcessId,
{
    fn prepare_for_log_without_config(&self) -> String {
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
