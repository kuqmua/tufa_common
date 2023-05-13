#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum NoItemsErrorNamed<'a> {
    ThereIsTag {
        #[eo_display_with_serialize_deserialize]
        tag: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ConversionFromStrError {
        #[eo_display_with_serialize_deserialize]
        string: std::string::String,
        #[eo_display_with_serialize_deserialize]
        error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NoTag {
        #[eo_display_with_serialize_deserialize]
        no_tag: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> NoItemsErrorNamed<'a> {
    pub fn get_stringified_kind(error: &NoItemsErrorNamed) -> &'static str {
        match error {
            NoItemsErrorNamed::ThereIsTag{ tag: _tag, code_occurence: _code_occurence } => stringify!(NoItemsErrorNamed::ThereIsTag),
            NoItemsErrorNamed::ConversionFromStrError{ string: _string, error: _error, code_occurence: _code_occurence} => {
                stringify!(NoItemsErrorNamed::ConversionFromStrError)
            }
            NoItemsErrorNamed::NoTag{ no_tag: _no_tag, code_occurence: _code_occurence } => stringify!(NoItemsErrorNamed::NoTag),
        }
    }
    // pub fn into_json_with_link_and_provider_kind(
    //     link: &str,
    //     no_items_error: &NoItemsErrorNamed,
    //     pk: &crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind,
    // ) -> serde_json::Value {
    //     match no_items_error {
    //         NoItemsErrorNamed::ThereIsTag{ tag, code_occurence: _code_occurence } => {
    //             serde_json::json!({
    //                 "error_kind": NoItemsErrorNamed::get_stringified_kind(no_items_error),
    //                 "link": link,
    //                 "tag": tag,
    //                 "part_of": format!("{pk}"),
    //                 "date": chrono::Local::now().to_string()
    //             })
    //         }
    //         NoItemsErrorNamed::ConversionFromStrError{ string, error, code_occurence: _code_occurence} => serde_json::json!({
    //             "error_kind": NoItemsErrorNamed::get_stringified_kind(no_items_error),
    //             "link": link,
    //             "string": string,
    //             "error": error,
    //             "part_of": format!("{pk}"),
    //             "date": chrono::Local::now().to_string()
    //         }),
    //         NoItemsErrorNamed::NoTag{ no_tag, code_occurence: _code_occurence} => serde_json::json!({
    //             "error_kind": NoItemsErrorNamed::get_stringified_kind(no_items_error),
    //             "link": link,
    //             "tag": no_tag,
    //             "part_of": format!("{pk}"),
    //             "date": chrono::Local::now().to_string()
    //         }),
    //     }
    // }
}
