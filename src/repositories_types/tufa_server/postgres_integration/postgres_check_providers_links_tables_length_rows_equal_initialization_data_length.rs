#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorNamed<'a> {
    SelectCountOrigin {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        inner_errors: std::collections::HashMap<std::string::String, PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ProviderLinksTablesRowsLengthNotEqualOrigin {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        inner_errors: std::collections::HashMap<std::string::String, PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxUnnamed<
    'a,
> {
    Postgres(PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxNamed<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxNamed<
    'a,
> {
    Postgres {
        #[eo_display_foreign_type]
        error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorUnnamed<
    'a,
> {
    Postgres(PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorNamed<'a>),
}


#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorNamed<
    'a,
> {
    Postgres {
        #[eo_display_with_serialize_deserialize]
        table_rows_length: i64,
        #[eo_display_with_serialize_deserialize]
        initialization_data_length: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}