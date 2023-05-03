#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct BiorxivStructForParsing {
    #[serde(rename = "item", default)]
    pub items: Vec<BiorxivStructForParsingItem>,
}
#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct BiorxivStructForParsingItem {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
    pub creator: Option<String>,
    pub identifier: Option<String>,
    pub publisher: Option<String>,
    #[serde(rename = "publicationDate", default)]
    pub publication_date: Option<String>,
}
