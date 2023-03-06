#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum PostgresInsertLinkPartsIntoProvidersTablesOriginError<'a> {
    Postgres {
        inner_errors: std::collections::HashMap<
            String,
            PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnum<'a>,
        >,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnum<'a> {
    Postgres {
        error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
