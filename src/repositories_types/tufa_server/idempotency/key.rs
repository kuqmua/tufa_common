#[derive(Debug)]
pub struct IdempotencyKey(String);

#[derive(Debug, thiserror::Error)]
pub enum IdempotencyKeyTryFromStringError {
    CannotBeEmpty(std::string::String),
    MustBeShorter(std::string::String),
}

impl std::fmt::Display for IdempotencyKeyTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IdempotencyKeyTryFromStringError::CannotBeEmpty(message) => write!(f, "{}", message),
            IdempotencyKeyTryFromStringError::MustBeShorter(message) => write!(f, "{}", message),
        }
    }
}

impl TryFrom<String> for IdempotencyKey {
    type Error = IdempotencyKeyTryFromStringError;

    fn try_from(s: std::string::String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Err(IdempotencyKeyTryFromStringError::CannotBeEmpty(
                std::string::String::from("The idempotency key cannot be empty"),
            ));
        }
        let max_length = 50;
        if s.len() >= max_length {
            return Err(IdempotencyKeyTryFromStringError::MustBeShorter(format!(
                "The idempotency key must be shorter than {max_length} characters"
            )));
        }
        Ok(Self(s))
    }
}

impl From<IdempotencyKey> for std::string::String {
    fn from(k: IdempotencyKey) -> Self {
        k.0
    }
}

impl AsRef<str> for IdempotencyKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
