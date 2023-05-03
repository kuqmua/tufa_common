#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct TwitterStructForParsing {
    #[serde(rename = "item", default)]
    pub items: Vec<TwitterStructForParsingItem>,
    pub image: TwitterStructForParsingImage,
}
#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct TwitterStructForParsingImage {
    //title
    //link
    //width
    //height
    pub url: Option<String>,
}
#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct TwitterStructForParsingItem {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "pubDate", default)]
    pub pub_date: Option<String>,
    pub guid: Option<String>,
    pub creator: Option<String>,
}
