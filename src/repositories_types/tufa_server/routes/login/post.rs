#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: secrecy::Secret<String>,
}

pub async fn login<'a>(
    form: actix_web::web::Form<FormData>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    session: crate::repositories_types::tufa_server::session_state::TypedSession,
) -> Result<actix_web::HttpResponse, actix_web::error::InternalError<LoginErrorNamed<'a>>> {
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
                .map_err(|e| login_redirect(LoginErrorNamed::SessionInsert{
                    session_insert: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }))?;
            Ok(actix_web::HttpResponse::SeeOther()
                .insert_header((actix_web::http::header::LOCATION, "/admin/dashboard"))
                .finish())
        }
        Err(e) => {
            Err(login_redirect(LoginErrorNamed::AuthError{
                validate_credentials: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }))
        }
    }
}

fn login_redirect(e: LoginErrorNamed) -> actix_web::error::InternalError<LoginErrorNamed> {
    actix_web_flash_messages::FlashMessage::error(e.to_string()).send();
    let response = actix_web::HttpResponse::SeeOther()
        .insert_header((actix_web::http::header::LOCATION, "/login"))
        .finish();
    actix_web::error::InternalError::from_response(e, response)
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