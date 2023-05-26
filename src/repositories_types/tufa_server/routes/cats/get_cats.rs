#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum CatsResult {
    Ok(Vec<Cats>),
    Err(std::string::String)
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Cats {
  pub id: i64,
  pub name: String
}
