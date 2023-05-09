pub async fn login_form(flash_messages: actix_web_flash_messages::IncomingFlashMessages) -> actix_web::HttpResponse {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        use std::fmt::Write;
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    actix_web::HttpResponse::Ok()
        .content_type(actix_web::http::header::ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>
<body>
    {error_html}
    <form action="/login" method="post">
        <label>Username
            <input
                type="text"
                placeholder="Enter Username"
                name="username"
            >
        </label>
        <label>Password
            <input
                type="password"
                placeholder="Enter Password"
                name="password"
            >
        </label>
        <button type="submit">Login</button>
    </form>
</body>
</html>"#,
        ))
}
