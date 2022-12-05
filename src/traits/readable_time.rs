pub trait ReadableTime {
    fn readable_time(&self, timezone: i32) -> String;
}
