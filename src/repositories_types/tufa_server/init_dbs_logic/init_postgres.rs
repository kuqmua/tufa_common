#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresInitErrorNamed<'a> {
    EstablishConnection{
        #[eo_error_occurence]
        establish_connection: crate::server::postgres::postgres_establish_connection::PostgresEstablishConnectionErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CreateTableQueries{
        #[eo_error_occurence]
        create_table_queries: crate::server::postgres::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CheckProviderLinksTablesAreEmpty{
        #[eo_error_occurence]
        check_provider_links_tables_are_empty: crate::server::postgres::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    DeleteAllFromProvidersTables{
        #[eo_error_occurence]
        delete_all_from_providers_tables: crate::server::postgres::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength{
        #[eo_error_occurence]
        check_providers_links_tables_length_rows_equal_initialization_data_length: crate::repositories_types::tufa_server::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    InsertLinkPartsIntoProvidersTables{
        #[eo_error_occurence]
        insert_link_parts_into_providers_tables: crate::repositories_types::tufa_server::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}