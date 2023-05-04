#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresDeleteAllFromProvidersTablesErrorNamed<'a> {
    DeleteTables {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        error_hashmap: std::collections::HashMap<std::string::String, sqlx::Error>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}