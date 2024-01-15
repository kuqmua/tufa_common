#[derive(
    Debug,
    Clone,
    strum_macros::EnumIter,
    enum_extension::EnumExtension,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    from_str::FromStr,
)]
pub enum TracingType {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for TracingType {
    fn default() -> Self {
        Self::Error
    }
}

impl std::fmt::Display for TracingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_snake_case())
    }
}
