pub mod request;
pub mod route;

#[derive(serde::Deserialize)]
pub struct GetByIdPathParameters {
    pub id: i64,
}
