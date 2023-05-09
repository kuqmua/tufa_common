#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: secrecy::Secret<String>,
}

#[tracing::instrument(
    skip(form, pool, session),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
    form: actix_web::web::Form<FormData>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    session: crate::repositories_types::tufa_server::session_state::TypedSession,
) -> Result<actix_web::HttpResponse, actix_web::error::InternalError<LoginError>> {
    let credentials = crate::common::postgres_credentials::PostgresCredentials {
        username: form.0.username,
        password: form.0.password,
    };
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
    match crate::repositories_types::tufa_server::authentication::validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;
            Ok(actix_web::HttpResponse::SeeOther()
                .insert_header((actix_web::http::header::LOCATION, "/admin/dashboard"))
                .finish())
        }
        Err(e) => {
            let e = match e {
                crate::repositories_types::tufa_server::authentication::AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                crate::repositories_types::tufa_server::authentication::AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            Err(login_redirect(e))
        }
    }
}

fn login_redirect(e: LoginError) -> actix_web::error::InternalError<LoginError> {
    actix_web_flash_messages::FlashMessage::error(e.to_string()).send();
    let response = actix_web::HttpResponse::SeeOther()
        .insert_header((actix_web::http::header::LOCATION, "/login"))
        .finish();
    actix_web::error::InternalError::from_response(e, response)
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::repositories_types::tufa_server::routes::subscriptions::error_chain_fmt(self, f)
    }
}
