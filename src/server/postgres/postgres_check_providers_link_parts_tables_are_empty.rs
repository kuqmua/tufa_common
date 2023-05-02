#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinkPartsTablesEmptyErrorNamed<'a> {
    SelectCountOrigin {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        hashmap_provider_kind_sqlx_error: std::collections::HashMap<std::string::String, sqlx::Error>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NotEmptyOrigin {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        hashmap_provider_kind_len: std::collections::HashMap<std::string::String, i64>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    }
}