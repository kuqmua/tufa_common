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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetUsernameErrorNamed<'a> {
    PostgresQuery {
        #[eo_display_foreign_type]
        get_username: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}


pub async fn get_username<'a>(user_id: uuid::Uuid, pool: &sqlx::PgPool) -> Result<String, GetUsernameErrorNamed<'a>> {
    match sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await {
        Ok(row) => Ok(row.username),
        Err(e) => Err(
            GetUsernameErrorNamed::PostgresQuery {
                get_username: e,
                code_occurence: crate::code_occurence_tufa_common!()
            }
        ), 
    }
}
