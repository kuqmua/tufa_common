use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resource {
    Local,
    Mongodb,
    PostgreSql,
}

impl Default for Resource {
    fn default() -> Self {
        Self::Local
    }
}

pub struct ParseResourceError {
    incorrect_str: String,
}

impl FromStr for Resource {
    type Err = ParseResourceError;

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn from_str(e: &str) -> Result<Self, ParseResourceError> {
        if e == "local" {
            return Ok(Resource::Local);
        } else if e == "mongo" {
            return Ok(Resource::Mongodb);
        } else if e == "postgres" {
            return Ok(Resource::PostgreSql);
        }
        Err(ParseResourceError {
            incorrect_str: e.to_string(),
        })
    }
}
