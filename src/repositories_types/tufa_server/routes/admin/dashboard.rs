pub async fn admin_dashboard(
    session: crate::repositories_types::tufa_server::session_state::TypedSession,
    pool: actix_web::web::Data<sqlx::PgPool>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(crate::repositories_types::tufa_server::utils::status_codes::e500)? {
        get_username(user_id, &pool).await.map_err(crate::repositories_types::tufa_server::utils::status_codes::e500)?
    } else {
        return Ok(actix_web::HttpResponse::SeeOther()
            .insert_header((actix_web::http::header::LOCATION, "/login"))
            .finish());
    };
    Ok(actix_web::HttpResponse::Ok()
        .content_type(actix_web::http::header::ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Admin dashboard</title>
</head>
<body>
    <p>Welcome {username}!</p>
    <p>Available actions:</p>
    <ol>
        <li><a href="/admin/password">Change password</a></li>
        <li>
            <form name="logoutForm" action="/admin/logout" method="post">
                <input type="submit" value="Logout">
            </form>
        </li>
    </ol>
</body>
</html>"#,
        )))
}

#[tracing::instrument(name = "Get username", skip(pool))]
pub async fn get_username(user_id: uuid::Uuid, pool: &sqlx::PgPool) -> Result<String, anyhow::Error> {
    let row = {
        use anyhow::Context;
        sqlx::query!(
            r#"
            SELECT username
            FROM users
            WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_one(pool)
        .await
        .context("Failed to perform a query to retrieve a username.")?
    };
    Ok(row.username)
}
