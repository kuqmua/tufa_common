use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonExample {
    pub first: String,
    pub second: String,
}
