#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperError<'a> {
    SelectCountOrigin {
        #[eo_error_occurence]
        inner_error: PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOrigin<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ProviderLinksTablesRowsLengthNotEqualOrigin {
        #[eo_error_occurence]
        inner_error: PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorProviderLinksTablesRowsLengthNotEqualOrigin<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOrigin<
    'a,
> {
    Postgres {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        inner_errors: std::collections::HashMap<std::string::String, PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxUnnamed<
    'a,
> {
    Postgres(PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlx<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlx<
    'a,
> {
    Postgres {
        #[eo_display_foreign_type]
        error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorProviderLinksTablesRowsLengthNotEqualOrigin<
    'a,
> {
    Postgres {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        inner_errors: std::collections::HashMap<std::string::String, PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorUnnamed<
    'a,
> {
    Postgres(PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginError<'a>),
}


#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginError<
    'a,
> {
    Postgres {
        #[eo_display_with_serialize_deserialize]
        error: ProviderLinksTablesLengthRowsNotEqualInitializationDataLength,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProviderLinksTablesLengthRowsNotEqualInitializationDataLength {
    pub table_rows_length: i64,
    pub initialization_data_length: usize,
}

impl std::fmt::Display for ProviderLinksTablesLengthRowsNotEqualInitializationDataLength {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "table_rows_length: {}, initialization_data_length: {}",
            self.table_rows_length, self.initialization_data_length
        )
    }
}
