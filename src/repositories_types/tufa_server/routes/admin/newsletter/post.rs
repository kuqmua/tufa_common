#[derive(serde::Deserialize)]
pub struct FormData {
    title: String,
    text_content: String,
    html_content: String,
    idempotency_key: String,
}

#[tracing::instrument(
    name = "Publish a newsletter issue",
    skip_all,
    fields(user_id=%*user_id)
)]
pub async fn publish_newsletter(
    form: actix_web::web::Form<FormData>,
    user_id: actix_web::web::ReqData<crate::repositories_types::tufa_server::authentication::UserId>,
    pool: actix_web::web::Data<sqlx::PgPool>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    let FormData {
        title,
        text_content,
        html_content,
        idempotency_key,
    } = form.0;
    let idempotency_key: crate::repositories_types::tufa_server::idempotency::IdempotencyKey = idempotency_key.try_into().map_err(crate::repositories_types::tufa_server::utils::status_codes::e400)?;
    let mut transaction = match crate::repositories_types::tufa_server::idempotency::try_processing(&pool, &idempotency_key, *user_id)
        .await
        .map_err(crate::repositories_types::tufa_server::utils::status_codes::e500)?
    {
        crate::repositories_types::tufa_server::idempotency::NextAction::StartProcessing(t) => t,
        crate::repositories_types::tufa_server::idempotency::NextAction::ReturnSavedResponse(saved_response) => {
            success_message().send();
            return Ok(saved_response);
        }
    };
    //"Failed to store newsletter issue details"
    let issue_id = insert_newsletter_issue(&mut transaction, &title, &text_content, &html_content)
        .await
        .map_err(crate::repositories_types::tufa_server::utils::status_codes::e500)?;
    //"Failed to enqueue delivery tasks"
    enqueue_delivery_tasks(&mut transaction, issue_id)
        .await
        .map_err(crate::repositories_types::tufa_server::utils::status_codes::e500)?;
    let response = crate::repositories_types::tufa_server::utils::status_codes::see_other("/admin/newsletters");
    let response = crate::repositories_types::tufa_server::idempotency::save_response(transaction, &idempotency_key, *user_id, response)
        .await
        .map_err(crate::repositories_types::tufa_server::utils::status_codes::e500)?;
    success_message().send();
    Ok(response)
}

fn success_message() -> actix_web_flash_messages::FlashMessage {
    actix_web_flash_messages::FlashMessage::info(
        "The newsletter issue has been accepted - \
        emails will go out shortly.",
    )
}

#[tracing::instrument(skip_all)]
async fn insert_newsletter_issue(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    title: &str,
    text_content: &str,
    html_content: &str,
) -> Result<uuid::Uuid, sqlx::Error> {
    let newsletter_issue_id = uuid::Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO newsletter_issues (
            newsletter_issue_id, 
            title, 
            text_content, 
            html_content,
            published_at
        )
        VALUES ($1, $2, $3, $4, now())
        "#,
        newsletter_issue_id,
        title,
        text_content,
        html_content
    )
    .execute(transaction)
    .await?;
    Ok(newsletter_issue_id)
}

#[tracing::instrument(skip_all)]
async fn enqueue_delivery_tasks(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    newsletter_issue_id: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO issue_delivery_queue (
            newsletter_issue_id, 
            subscriber_email
        )
        SELECT $1, email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#,
        newsletter_issue_id,
    )
    .execute(transaction)
    .await?;
    Ok(())
}
