#[derive(serde::Deserialize)]
pub struct FormData {
    pub username: String,
    pub password: secrecy::Secret<String>,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum LoginErrorNamed<'a> {
    AuthError {
        #[eo_error_occurence]
        validate_credentials: crate::repositories_types::tufa_server::authentication::password::ValidateCredentialsErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    SessionInsert {
        #[eo_error_occurence]
        session_insert: crate::repositories_types::tufa_server::session_state::InsertUserIdErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    }
}