#[derive(Copy, Clone, Debug)]
pub struct UserId(uuid::Uuid);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::ops::Deref for UserId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn reject_anonymous_users(
    mut req: actix_web::dev::ServiceRequest,
    next: actix_web_lab::middleware::Next<impl actix_web::body::MessageBody>,
) -> Result<actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        {
            use actix_web::FromRequest;
            crate::repositories_types::tufa_server::session_state::TypedSession::from_request(http_request, payload)
        }.await
    }?;
    match session.get_user_id().map_err(crate::repositories_types::tufa_server::utils::status_codes::e500)? {
        Some(user_id) => {
            {
                use actix_web::HttpMessage;
                req.extensions_mut()
            }.insert(UserId(user_id));
            next.call(req).await
        }
        None => {
            let response = crate::repositories_types::tufa_server::utils::status_codes::see_other("/login");
            let e = anyhow::anyhow!("The user has not logged in");
            Err(actix_web::error::InternalError::from_response(e, response).into())
        }
    }
}
