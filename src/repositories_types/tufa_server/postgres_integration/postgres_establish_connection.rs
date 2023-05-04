#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresEstablishConnectionErrorNamed<'a> {
    Connect {
        #[eo_display]
        sqlx_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}