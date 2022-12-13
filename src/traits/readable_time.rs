pub trait ReadableTime {
    fn readable_time(&self, timezone: i32) -> String;
}

impl<SelfGeneric> ReadableTime for SelfGeneric
where
    SelfGeneric: crate::traits::get_time::GetTime,
{
    fn readable_time(&self, timezone: i32) -> String {
        chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_time())
            .with_timezone(&chrono::FixedOffset::east(timezone))
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}
