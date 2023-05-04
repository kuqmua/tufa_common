#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCreateProvidersDbsErrorNamed<'a> {
    Postgres {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        sqlx_error_hashmap: std::collections::HashMap<std::string::String, sqlx::Error>,//todo key ProviderKind instead of std::string::String
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
