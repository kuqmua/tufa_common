#[track_caller]
pub fn location() -> core::panic::Location<'static> {
    *core::panic::Location::caller()
}
