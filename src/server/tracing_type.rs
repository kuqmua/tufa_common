#[derive(
    Debug,
    Clone,
    strum_macros::EnumIter,
    strum_macros::Display,
    enum_extension::EnumExtension,
    serde::Deserialize,
    PartialEq,
    Eq,
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

impl std::str::FromStr for TracingType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "trace" => Ok(Self::Trace),
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            _ => Err(String::from("wtf")),
        }
    }
}
