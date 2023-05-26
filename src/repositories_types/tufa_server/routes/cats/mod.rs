pub mod get;
pub mod post;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Cat {
  pub id: i64,
  pub name: String
}