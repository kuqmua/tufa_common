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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryExecuteTaskErrorNamed<'a> {
    DequeueTask {
        #[eo_error_occurence]
        dequeue_task: DequeueTaskErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    GetIssue {
        #[eo_error_occurence]
        get_issue: GetIssueErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    DeleteTask {
        #[eo_error_occurence]
        delete_task: DeleteTaskErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn try_execute_task<'a>(
    pool: &sqlx::PgPool,
    email_client: &crate::repositories_types::tufa_server::email_client::EmailClient,
) -> Result<ExecutionOutcome, TryExecuteTaskErrorNamed<'a>> {
    let task = match dequeue_task(pool).await {
        Err(e) => {
            return Err(TryExecuteTaskErrorNamed::DequeueTask {
                dequeue_task: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }, 
        Ok(option_task) => option_task,
    };
    match task {
        None => Ok(ExecutionOutcome::EmptyQueue),
        Some(task) => {
            let (transaction, issue_id, email) = task;
            tracing::Span::current()
                .record("newsletter_issue_id", &tracing::field::display(issue_id))
                .record("subscriber_email", &tracing::field::display(&email));
            match crate::repositories_types::tufa_server::domain::SubscriberEmail::parse(email.clone()) {
                Ok(email) => {
                    let issue = match get_issue(pool, issue_id).await {
                        Err(e) => {
                            return Err(TryExecuteTaskErrorNamed::GetIssue {
                                get_issue: e,
                                code_occurence: crate::code_occurence_tufa_common!(),
                            });
                        },
                        Ok(newletter_issue) => newletter_issue,
                    };
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
            if let Err(e) = delete_task(transaction, issue_id, &email).await {
                return Err(TryExecuteTaskErrorNamed::DeleteTask {
                    delete_task: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
            Ok(ExecutionOutcome::TaskCompleted)
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum DequeueTaskErrorNamed<'a> {
    PostgresPoolBegin {
        #[eo_display]
        postgres_pool_begin: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresSelect {
        #[eo_display]
        postgres_select: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

async fn dequeue_task<'a>(
    pool: &sqlx::PgPool,
) -> Result<Option<(sqlx::Transaction<'static, sqlx::Postgres>, uuid::Uuid, String)>, DequeueTaskErrorNamed<'a>> {
    match pool.begin().await {
        Err(e) => Err(DequeueTaskErrorNamed::PostgresPoolBegin {
            postgres_pool_begin: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
        Ok(mut transaction) => match sqlx::query!(
            r#"
            SELECT newsletter_issue_id, subscriber_email
            FROM issue_delivery_queue
            FOR UPDATE
            SKIP LOCKED
            LIMIT 1
            "#,
        )
        .fetch_optional(&mut transaction)
        .await {
            Err(e) => Err(DequeueTaskErrorNamed::PostgresSelect {
                postgres_select: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
            Ok(option_record) => match option_record {
                Some(record) => Ok(Some((
                    transaction,
                    record.newsletter_issue_id,
                    record.subscriber_email,
                ))),
                None => Ok(None),
            }
        },
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum DeleteTaskErrorNamed<'a> {
    PostgresDeleteTask {
        #[eo_display]
        postgres_delete_task: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresTransactionCommit {
        #[eo_display]
        postgres_transaction_commit: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[tracing::instrument(skip_all)]
async fn delete_task<'a>(
    mut transaction: sqlx::Transaction<'static, sqlx::Postgres>,
    issue_id: uuid::Uuid,
    email: &str,
) -> Result<(), DeleteTaskErrorNamed<'a>> {
    if let Err(e) = sqlx::query!(
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
    .await {
        return Err(DeleteTaskErrorNamed::PostgresDeleteTask {
            postgres_delete_task: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    if let Err(e) = transaction.commit().await {
        return Err(DeleteTaskErrorNamed::PostgresTransactionCommit {
            postgres_transaction_commit: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    Ok(())
}

struct NewsletterIssue {
    title: String,
    text_content: String,
    html_content: String,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetIssueErrorNamed<'a> {
    PostgresSelectNewsletterIssues {
        #[eo_display]
        postgres_select_newsletter_issues: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[tracing::instrument(skip_all)]
async fn get_issue<'a>(pool: &sqlx::PgPool, issue_id: uuid::Uuid) -> Result<NewsletterIssue, GetIssueErrorNamed<'a>> {
    match sqlx::query_as!(
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
    .await {
        Err(e) => Err(GetIssueErrorNamed::PostgresSelectNewsletterIssues {
            postgres_select_newsletter_issues: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
        Ok(issue) => Ok(issue)
    }
}
