use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourcePlaceType {
    Source,
    Github,
    None,
}

impl Default for SourcePlaceType {
    fn default() -> Self {
        Self::None
    }
}

pub struct ParseSourcePlaceTypeError {
    _incorrect_str: String,
}

impl FromStr for SourcePlaceType {
    type Err = ParseSourcePlaceTypeError;
    fn from_str(e: &str) -> Result<Self, ParseSourcePlaceTypeError> {
        match e {
            "source" => Ok(SourcePlaceType::Source),
            "github" => Ok(SourcePlaceType::Github),
            "none" => Ok(SourcePlaceType::None),
            _ => Err(ParseSourcePlaceTypeError {
                _incorrect_str: e.to_string(),
            }),
        }
    }
}
