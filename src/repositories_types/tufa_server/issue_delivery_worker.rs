pub async fn run_worker_until_stopped<'a>(configuration: crate::repositories_types::tufa_server::configuration::Settings<'a>) -> Result<(), anyhow::Error> {
    let connection_pool = crate::repositories_types::tufa_server::startup::get_connection_pool(&configuration.database);
    let email_client = configuration.email_client.client();
    worker_loop(connection_pool, email_client).await
}

async fn worker_loop(pool: sqlx::PgPool, email_client: crate::repositories_types::tufa_server::email_client::EmailClient) -> Result<(), anyhow::Error> {
    loop {
        match try_execute_task(&pool, &email_client).await {
            Ok(ExecutionOutcome::EmptyQueue) => {
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
            Err(_) => {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            Ok(ExecutionOutcome::TaskCompleted) => {}
        }
    }
}

pub enum ExecutionOutcome {
    TaskCompleted,
    EmptyQueue,
}

#[tracing::instrument(
    skip_all,
    fields(
        newsletter_issue_id=tracing::field::Empty,
        subscriber_email=tracing::field::Empty
    ),
    err
)]
pub async fn try_execute_task(
    pool: &sqlx::PgPool,
    email_client: &crate::repositories_types::tufa_server::email_client::EmailClient,
) -> Result<ExecutionOutcome, anyhow::Error> {
    let task = dequeue_task(pool).await?;
    match task {
        None => Ok(ExecutionOutcome::EmptyQueue),
        Some(task) => {
            let (transaction, issue_id, email) = task;
            tracing::Span::current()
                .record("newsletter_issue_id", &tracing::field::display(issue_id))
                .record("subscriber_email", &tracing::field::display(&email));
            match crate::repositories_types::tufa_server::domain::SubscriberEmail::parse(email.clone()) {
                Ok(email) => {
                    let issue = get_issue(pool, issue_id).await?;
                    if let Err(e) = email_client
                        .send_email(
                            &email,
                            &issue.title,
                            &issue.html_content,
                            &issue.text_content,
                        )
                        .await
                    {
                        tracing::error!(
                            error.cause_chain = ?e,
                            error.message = %e,
                            "Failed to deliver issue to a confirmed subscriber. \
                                Skipping.",
                        );
                    }
                }
                Err(e) => {
                    tracing::error!(
                        error.cause_chain = ?e,
                        error.message = %e,
                        "Skipping a confirmed subscriber. \
                            Their stored contact details are invalid",
                    );
                }
            }
            delete_task(transaction, issue_id, &email).await?;
            Ok(ExecutionOutcome::TaskCompleted)
        }
    }
}

type PgTransaction = sqlx::Transaction<'static, sqlx::Postgres>;

#[tracing::instrument(skip_all)]
async fn dequeue_task(
    pool: &sqlx::PgPool,
) -> Result<Option<(PgTransaction, uuid::Uuid, String)>, anyhow::Error> {
    let mut transaction = pool.begin().await?;
    let r = sqlx::query!(
        r#"
        SELECT newsletter_issue_id, subscriber_email
        FROM issue_delivery_queue
        FOR UPDATE
        SKIP LOCKED
        LIMIT 1
        "#,
    )
    .fetch_optional(&mut transaction)
    .await?;
    if let Some(r) = r {
        Ok(Some((
            transaction,
            r.newsletter_issue_id,
            r.subscriber_email,
        )))
    } else {
        Ok(None)
    }
}

#[tracing::instrument(skip_all)]
async fn delete_task(
    mut transaction: PgTransaction,
    issue_id: uuid::Uuid,
    email: &str,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
        DELETE FROM issue_delivery_queue
        WHERE 
            newsletter_issue_id = $1 AND
            subscriber_email = $2 
        "#,
        issue_id,
        email
    )
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

struct NewsletterIssue {
    title: String,
    text_content: String,
    html_content: String,
}

#[tracing::instrument(skip_all)]
async fn get_issue(pool: &sqlx::PgPool, issue_id: uuid::Uuid) -> Result<NewsletterIssue, anyhow::Error> {
    let issue = sqlx::query_as!(
        NewsletterIssue,
        r#"
        SELECT title, text_content, html_content
        FROM newsletter_issues
        WHERE
            newsletter_issue_id = $1
        "#,
        issue_id
    )
    .fetch_one(pool)
    .await?;
    Ok(issue)
}
