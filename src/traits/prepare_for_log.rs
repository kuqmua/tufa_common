pub trait PrepareForLog {
    fn prepare_for_log(
        &self,
        path: String,
        time: String,
        hostname: &String,
        process_id: &u32,
    ) -> String {
        format!("{} {} {} pid: {}", path, time, hostname, process_id)
    }
}

impl<SelfGeneric> PrepareForLog for SelfGeneric
where
    SelfGeneric: crate::traits::fields::GetFile
        + crate::traits::fields::GetLine
        + crate::traits::fields::GetColumn
        + crate::traits::fields::GetCodePathWithoutConfig
        + crate::traits::get_duration::GetDuration
        + crate::traits::get_hostname::GetHostname
        + crate::traits::get_process_id::GetProcessId,
{
    fn prepare_for_log(
        &self,
        // path: String,
        // time: String,
        // hostname: &String,
        // process_id: &u32,
    ) -> String {
        format!("{} {} {} pid: {}", path, time, hostname, process_id)
    }
}
