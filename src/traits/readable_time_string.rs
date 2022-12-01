use std::time::Duration;

pub trait ReadableTimeStringTrait {
    fn readable_time_string(&self, duration: Duration) -> String;
}
