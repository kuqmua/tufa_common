#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct GithubStructForParsing {
    #[serde(rename = "entry", default)]
    pub entries: Vec<GithubStructForParsingItem>,
}
#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct GithubStructForParsingItem {
    pub id: Option<String>,
    pub published: Option<String>,
    pub updated: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>,
    pub author: GithubStructForParsingItemAuthor,
    pub media: Option<String>,
    pub content: Option<String>,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct GithubStructForParsingItemAuthor {
    pub name: Option<String>,
    pub uri: Option<String>,
}
