#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresInsertLinkPartsIntoProvidersTablesOriginErrorNamed<'a> {
    Postgres {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        inner_errors: std::collections::HashMap<
            std::string::String,
            PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnumUnnamed<'a>,
        >,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnumUnnamed<'a> {
    PostgresInsertLinkPartsIntoProvidersTablesOriginHandle(PostgresInsertLinkPartsIntoProvidersTablesOriginHandleErrorNamed<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresInsertLinkPartsIntoProvidersTablesOriginHandleErrorNamed<'a> {
    Postgres {
        #[eo_display_foreign_type]
        error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}