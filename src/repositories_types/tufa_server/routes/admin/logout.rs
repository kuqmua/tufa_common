pub async fn log_out(
    session: crate::repositories_types::tufa_server::session_state::TypedSession,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    if session
        .get_user_id()
        .map_err(crate::repositories_types::tufa_server::utils::status_codes::e500)?
        .is_none()
    {
        Ok(crate::repositories_types::tufa_server::utils::status_codes::see_other("/login"))
    } else {
        session.log_out();
        actix_web_flash_messages::FlashMessage::info("You have successfully logged out.").send();
        Ok(crate::repositories_types::tufa_server::utils::status_codes::see_other("/login"))
    }
}
