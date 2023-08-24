#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, strum_macros::Display)]
pub enum Order {
    #[serde(rename(serialize = "asc", deserialize = "asc"))]
    Asc,
    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    Desc,
}

impl crate::common::url_encode::UrlEncode for Order {
    fn url_encode(&self) -> std::string::String {
        urlencoding::encode(&self.to_string()).to_string()
    }
}
