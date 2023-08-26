#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum Order {
    #[serde(rename(serialize = "asc", deserialize = "asc"))]
    Asc,
    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    Desc,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Order::Asc => write!(f, "asc"),
            Order::Desc => write!(f, "desc"),
        }
    }
}

impl Default for Order {
    fn default() -> Self {
        Self::Asc
    }
}

impl crate::common::url_encode::UrlEncode for Order {
    fn url_encode(&self) -> std::string::String {
        urlencoding::encode(&self.to_string()).to_string()
    }
}
