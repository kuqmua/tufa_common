#[derive(
    Debug,
    Clone,
    strum_macros::EnumIter,
    enum_extension::EnumExtension,
    serde::Serialize,
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

//todo make a proc macro for it
impl std::str::FromStr for TracingType {
    type Err = std::string::String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "trace" => Ok(Self::Trace),
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            _ => Err(format!(
                "Invalid TracingType, expected one of \'trace\', \'debug\', \'info\', \'warn\', \'error\' found {value}"
            )),
        }
    }
}

impl std::fmt::Display for TracingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_lower_snake_case())
    }
}
