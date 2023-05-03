#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct ArxivStructForParsing {
    #[serde(rename = "item", default)]
    pub items: Vec<ArxivStructForParsingItem>,
}
#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct ArxivStructForParsingItem {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub creator: Option<String>,
}
