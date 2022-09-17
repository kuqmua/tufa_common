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

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn from_str(e: &str) -> Result<Self, ParseSourcePlaceTypeError> {
        if e == "source" {
            return Ok(SourcePlaceType::Source);
        } else if e == "github" {
            return Ok(SourcePlaceType::Github);
        } else if e == "none" {
            return Ok(SourcePlaceType::None);
        }
        Err(ParseSourcePlaceTypeError {
            _incorrect_str: e.to_string(),
        })
    }
}
