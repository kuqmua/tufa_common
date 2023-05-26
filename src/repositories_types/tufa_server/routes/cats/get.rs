#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum GetResponse {
    Ok(Vec<super::Cat>),
    Err(std::string::String)
}
