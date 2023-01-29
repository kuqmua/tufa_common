pub trait ReadableTime {
    fn readable_time(&self, timezone: i32) -> String;
}

impl<SelfGeneric> ReadableTime for SelfGeneric
where
    SelfGeneric: crate::traits::get_duration::GetDuration,
{
    fn readable_time(&self, timezone: i32) -> String {
        chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
            .with_timezone(&chrono::FixedOffset::east_opt(timezone).unwrap())
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}
