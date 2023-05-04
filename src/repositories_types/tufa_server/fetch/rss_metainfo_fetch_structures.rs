
#[derive(Debug, Clone)] //Debug only for prints
pub enum NoItemsError {
    ThereIsTag(String),
    ConversionFromStrError(String, String),
    NoTag(String),
}

impl NoItemsError {
    pub fn get_stringified_kind(error: &NoItemsError) -> &'static str {
        match error {
            NoItemsError::ThereIsTag(_) => stringify!(NoItemsError::ThereIsTag),
            NoItemsError::ConversionFromStrError(_, _) => {
                stringify!(NoItemsError::ConversionFromStrError)
            }
            NoItemsError::NoTag(_) => stringify!(NoItemsError::NoTag),
        }
    }
    pub fn into_json_with_link_and_provider_kind(
        link: &str,
        no_items_error: &NoItemsError,
        pk: &crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind,
    ) -> serde_json::Value {
        match no_items_error {
            NoItemsError::ThereIsTag(tag) => {
                serde_json::json!({
                    "error_kind": NoItemsError::get_stringified_kind(no_items_error),
                    "link": link,
                    "tag": tag,
                    "part_of": format!("{pk}"),
                    "date": chrono::Local::now().to_string()
                })
            }
            NoItemsError::ConversionFromStrError(string, error) => serde_json::json!({
                "error_kind": NoItemsError::get_stringified_kind(no_items_error),
                "link": link,
                "string": string,
                "error": error,
                "part_of": format!("{pk}"),
                "date": chrono::Local::now().to_string()
            }),
            NoItemsError::NoTag(tag) => serde_json::json!({
                "error_kind": NoItemsError::get_stringified_kind(no_items_error),
                "link": link,
                "tag": tag,
                "part_of": format!("{pk}"),
                "date": chrono::Local::now().to_string()
            }),
        }
    }
}
