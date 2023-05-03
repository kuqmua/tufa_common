#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct HabrStructForParsing {
    #[serde(rename = "item", default)]
    pub items: Vec<HabrStructForParsingItem>,
}
#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct HabrStructForParsingItem {
    pub title: Option<String>,
    pub guid: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "pubDate", default)]
    pub pub_date: Option<String>,
    pub creator: Option<String>,
    pub category: Option<Vec<String>>, //Option<Vec<Option<String>>>???????
}
