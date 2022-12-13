pub trait ReadableTimeString {
    fn readable_time_string(&self) -> String;
}

impl<SelfGeneric> ReadableTimeString for SelfGeneric
where
    SelfGeneric: crate::traits::get_time::GetTime,
{
    fn readable_time_string(&self) -> String {
        chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_time())
            .format("%Y-%m-%d %H:%M:%S.%f")
            .to_string()
    }
}
