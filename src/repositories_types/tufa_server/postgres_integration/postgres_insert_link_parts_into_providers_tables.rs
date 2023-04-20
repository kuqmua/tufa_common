#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresInsertLinkPartsIntoProvidersTablesOriginError<'a> {
    Postgres {
        #[eo_hashmap_key_display_value_error_occurence]
        inner_errors: std::collections::HashMap<
            String,
            PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnumUnnamed<'a>,
        >,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnumUnnamed<'a> {
    Something(PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnum<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnum<'a> {
    Postgres {
        #[eo_display_foreign_type]
        error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
