// pub async fn change_password_form(
//     session: crate::repositories_types::tufa_server::session_state::TypedSession,
//     flash_messages: actix_web_flash_messages::IncomingFlashMessages,
// ) -> Result<actix_web::HttpResponse, actix_web::Error> {
//     if session
//         .get_user_id()
//         .map_err(crate::repositories_types::tufa_server::utils::status_codes::e500)?
//         .is_none()
//     {
//         return Ok(
//             crate::repositories_types::tufa_server::utils::status_codes::see_other("/login"),
//         );
//     };
//     let mut msg_html = std::string::String::new();
//     for m in flash_messages.iter() {
//         use std::fmt::Write;
//         writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
//     }
//     Ok(actix_web::HttpResponse::Ok()
//         .content_type(actix_web::http::header::ContentType::html())
//         .body(format!(
//             r#"<!DOCTYPE html>
// <html lang="en">
// <head>
//     <meta http-equiv="content-type" content="text/html; charset=utf-8">
//     <title>Change Password</title>
// </head>
// <body>
//     {msg_html}
//     <form action="/admin/password" method="post">
//         <label>Current password
//             <input
//                 type="password"
//                 placeholder="Enter current password"
//                 name="current_password"
//             >
//         </label>
//         <br>
//         <label>New password
//             <input
//                 type="password"
//                 placeholder="Enter new password"
//                 name="new_password"
//             >
//         </label>
//         <br>
//         <label>Confirm new password
//             <input
//                 type="password"
//                 placeholder="Type the new password again"
//                 name="new_password_check"
//             >
//         </label>
//         <br>
//         <button type="submit">Change password</button>
//     </form>
//     <p><a href="/admin/dashboard">&lt;- Back</a></p>
// </body>
// </html>"#,
//         )))
// }
