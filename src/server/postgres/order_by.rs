#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum OrderBy {
    #[serde(rename(serialize = "asc", deserialize = "asc"))]
    Asc,
    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    Desc,
}
