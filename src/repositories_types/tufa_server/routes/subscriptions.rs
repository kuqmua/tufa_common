// #[derive(serde::Deserialize)]
// pub struct FormData {
//     pub email: std::string::String,
//     pub name: std::string::String,
// }

// impl std::convert::TryFrom<FormData>
//     for crate::repositories_types::tufa_server::domain::NewSubscriber
// {
//     type Error = std::string::String;
//     fn try_from(value: FormData) -> Result<Self, Self::Error> {
//         let name =
//             crate::repositories_types::tufa_server::domain::SubscriberName::parse(value.name)?;
//         let email =
//             crate::repositories_types::tufa_server::domain::SubscriberEmail::try_from(value.email)?;
//         Ok(Self { email, name })
//     }
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum SubscribeErrorNamed {
//     TryIntoNewSubscriber {
//         #[eo_display_with_serialize_deserialize]
//         try_into_new_subscriber: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     PostgresPoolBegin {
//         #[eo_display]
//         postgres_pool_begin: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     InsertSubscriber {
//         #[eo_display]
//         insert_subscriber: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     StoreToken {
//         #[eo_display]
//         store_token: StoreTokenError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     PostgresTransactionCommit {
//         #[eo_display]
//         postgres_transaction_commit: sqlx::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     SendConfirmationEmail {
//         #[eo_display]
//         send_confirmation_email: reqwest::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// impl<'a> actix_web::ResponseError for SubscribeErrorNamed {
//     fn status_code(&self) -> actix_web::http::StatusCode {
//         match self {
//             SubscribeErrorNamed::TryIntoNewSubscriber {
//                 try_into_new_subscriber: _try_into_new_subscriber,
//                 code_occurence: _code_occurence,
//             } => actix_web::http::StatusCode::BAD_REQUEST,
//             SubscribeErrorNamed::PostgresPoolBegin {
//                 postgres_pool_begin: _postgres_pool_begin,
//                 code_occurence: _code_occurence,
//             } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
//             SubscribeErrorNamed::InsertSubscriber {
//                 insert_subscriber: _insert_subscriber,
//                 code_occurence: _code_occurence,
//             } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
//             SubscribeErrorNamed::StoreToken {
//                 store_token: _store_token,
//                 code_occurence: _code_occurence,
//             } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
//             SubscribeErrorNamed::PostgresTransactionCommit {
//                 postgres_transaction_commit: _postgres_transaction_commit,
//                 code_occurence: _code_occurence,
//             } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
//             SubscribeErrorNamed::SendConfirmationEmail {
//                 send_confirmation_email: _send_confirmation_email,
//                 code_occurence: _code_occurence,
//             } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }
// }

// pub fn generate_subscription_token() -> std::string::String {
//     let mut rng = rand::thread_rng();
//     std::iter::repeat_with(|| {
//         use rand::Rng;
//         rng.sample(rand::distributions::Alphanumeric)
//     })
//     .map(char::from)
//     .take(25)
//     .collect()
// }

// #[tracing::instrument(
//     name = "Send a confirmation email to a new subscriber",
//     skip(email_client, new_subscriber, base_url, subscription_token)
// )]
// pub async fn send_confirmation_email(
//     email_client: &crate::repositories_types::tufa_server::email_client::EmailClient,
//     new_subscriber: crate::repositories_types::tufa_server::domain::NewSubscriber,
//     base_url: &str,
//     subscription_token: &str,
// ) -> Result<(), reqwest::Error> {
//     let confirmation_link = format!(
//         "{}/subscriptions/confirm?subscription_token={}",
//         base_url, subscription_token
//     );
//     let plain_body = format!(
//         "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
//         confirmation_link
//     );
//     let html_body = format!(
//         "Welcome to our newsletter!<br />Click <a href=\"{}\">here</a> to confirm your subscription.",
//         confirmation_link
//     );
//     email_client
//         .send_email(&new_subscriber.email, "Welcome!", &html_body, &plain_body)
//         .await
// }

// pub struct StoreTokenError(pub sqlx::Error);

// impl std::error::Error for StoreTokenError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         Some(&self.0)
//     }
// }

// impl std::fmt::Debug for StoreTokenError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         error_chain_fmt(self, f)
//     }
// }

// impl std::fmt::Display for StoreTokenError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "A database failure was encountered while trying to store a subscription token."
//         )
//     }
// }

// pub fn error_chain_fmt(
//     e: &impl std::error::Error,
//     f: &mut std::fmt::Formatter<'_>,
// ) -> std::fmt::Result {
//     writeln!(f, "{}\n", e)?;
//     let mut current = e.source();
//     while let Some(cause) = current {
//         writeln!(f, "Caused by:\n\t{}", cause)?;
//         current = cause.source();
//     }
//     Ok(())
// }
