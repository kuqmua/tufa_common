#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum Order {
    #[serde(rename(serialize = "asc", deserialize = "asc"))]
    Asc,
    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    Desc,
}

impl std::str::FromStr for Order {
    type Err = std::string::String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err(format!(
                "Invalid Order, expected one of \'asc\', \'desc\' found {value}"
            )),
        }
    }
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Order::Asc => write!(f, "{}", crate::server::postgres::constants::ASC_NAME),
            Order::Desc => write!(f, "{}", crate::server::postgres::constants::DESC_NAME),
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
