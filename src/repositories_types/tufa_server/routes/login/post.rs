// #[derive(serde::Deserialize)]
// pub struct FormData {
//     pub username: String,
//     pub password: secrecy::Secret<String>,
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum LoginErrorNamed {
//     AuthError {
//         #[eo_error_occurence]
//         validate_credentials: crate::repositories_types::tufa_server::authentication::password::ValidateCredentialsErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     SessionInsert {
//         #[eo_error_occurence]
//         session_insert: crate::repositories_types::tufa_server::session_state::InsertUserIdErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     }
// }
