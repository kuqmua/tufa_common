#[derive(
    Debug,
    Clone,
    Copy,
    strum_macros::EnumIter,
    strum_macros::Display,
    enum_extension::EnumExtension,
    serde::Deserialize,
    PartialEq,
    Eq,
)]
pub enum LogType {
    Tracing,
    Stack,
    None,
}

impl crate::traits::console::Console for LogType {
    fn console(&self, style: &ansi_term::Style, occurence: String) {
        match self {
            LogType::Tracing => {
                tracing::error!(error = occurence);
            }
            LogType::Stack => {
                eprintln!("{}", style.paint(occurence));
            }
            LogType::None => (),
        }
    }
}

impl Default for LogType {
    fn default() -> Self {
        Self::Stack
    }
}

impl std::str::FromStr for LogType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tracing" => Ok(Self::Tracing),
            "stack" => Ok(Self::Stack),
            "none" => Ok(Self::None),
            _ => Err(String::from("cannot convert str to TracingType")),
        }
    }
}
