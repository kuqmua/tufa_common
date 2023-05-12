#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

impl std::convert::TryFrom<FormData> for crate::repositories_types::tufa_server::domain::NewSubscriber {
    type Error = String;
    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = crate::repositories_types::tufa_server::domain::SubscriberName::parse(value.name)?;
        let email = crate::repositories_types::tufa_server::domain::SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum SubscribeErrorNamed<'a> {
    TryIntoNewSubscriber {
        #[eo_display_with_serialize_deserialize]
        try_into_new_subscriber: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresPoolBegin {
        #[eo_display]
        postgres_pool_begin: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    InsertSubscriber {
        #[eo_display]
        insert_subscriber: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    StoreToken {
        #[eo_display]
        store_token: StoreTokenError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresTransactionCommit {
        #[eo_display]
        postgres_transaction_commit: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    SendConfirmationEmail {
        #[eo_display]
        send_confirmation_email: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> actix_web::ResponseError for SubscribeErrorNamed<'a> {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            SubscribeErrorNamed::TryIntoNewSubscriber { try_into_new_subscriber, code_occurence } => actix_web::http::StatusCode::BAD_REQUEST,
            SubscribeErrorNamed::PostgresPoolBegin { postgres_pool_begin, code_occurence } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            SubscribeErrorNamed::InsertSubscriber { insert_subscriber, code_occurence } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            SubscribeErrorNamed::StoreToken { store_token, code_occurence } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            SubscribeErrorNamed::PostgresTransactionCommit { postgres_transaction_commit, code_occurence } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            SubscribeErrorNamed::SendConfirmationEmail { send_confirmation_email, code_occurence } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn subscribe<'a>(
    form: actix_web::web::Form<FormData>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    email_client: actix_web::web::Data<crate::repositories_types::tufa_server::email_client::EmailClient>,
    base_url: actix_web::web::Data<crate::repositories_types::tufa_server::startup::ApplicationBaseUrl>,
) -> Result<actix_web::HttpResponse, SubscribeErrorNamed<'a>> {
    let new_subscriber: crate::repositories_types::tufa_server::domain::NewSubscriber = match form.0.try_into() {
        Err(e) => {
            return Err(SubscribeErrorNamed::TryIntoNewSubscriber {
                try_into_new_subscriber: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        },
        Ok(new_subscriber) => new_subscriber,
    };
    //"Failed to acquire a Postgres connection from the pool"
    let mut transaction = match pool.begin().await {
        Err(e) => {
            return Err(SubscribeErrorNamed::PostgresPoolBegin {
                postgres_pool_begin: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        },
        Ok(transaction) => transaction,
    };
    //"Failed to insert new subscriber in the database."
    let subscriber_id = match insert_subscriber(&mut transaction, &new_subscriber).await {
        Err(e) => {
            return Err(SubscribeErrorNamed::InsertSubscriber {
                insert_subscriber: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        },
        Ok(subscriber_id) => subscriber_id,
    };
    let subscription_token = generate_subscription_token();
    //"Failed to store the confirmation token for a new subscriber."
    if let Err(e) = store_token(&mut transaction, subscriber_id, &subscription_token)
    .await {
        return Err(SubscribeErrorNamed::StoreToken {
            store_token: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    //"Failed to commit SQL transaction to store a new subscriber."
    if let Err(e) = transaction.commit().await {
        return Err(SubscribeErrorNamed::PostgresTransactionCommit {
            postgres_transaction_commit: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    //"Failed to send a confirmation email."
    if let Err(e) = send_confirmation_email(
        &email_client,
        new_subscriber,
        &base_url.0,
        &subscription_token,
    ).await {
        return Err(SubscribeErrorNamed::SendConfirmationEmail {
            send_confirmation_email: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    Ok(actix_web::HttpResponse::Ok().finish())
}

fn generate_subscription_token() -> String {
    let mut rng = rand::thread_rng();
    std::iter::repeat_with(|| {
        use rand::Rng;
        rng.sample(rand::distributions::Alphanumeric)
    })
        .map(char::from)
        .take(25)
        .collect()
}

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, new_subscriber, base_url, subscription_token)
)]
pub async fn send_confirmation_email(
    email_client: &crate::repositories_types::tufa_server::email_client::EmailClient,
    new_subscriber: crate::repositories_types::tufa_server::domain::NewSubscriber,
    base_url: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/subscriptions/confirm?subscription_token={}",
        base_url, subscription_token
    );
    let plain_body = format!(
        "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
        confirmation_link
    );
    let html_body = format!(
        "Welcome to our newsletter!<br />Click <a href=\"{}\">here</a> to confirm your subscription.",
        confirmation_link
    );
    email_client
        .send_email(&new_subscriber.email, "Welcome!", &html_body, &plain_body)
        .await
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, transaction)
)]
pub async fn insert_subscriber(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    new_subscriber: &crate::repositories_types::tufa_server::domain::NewSubscriber,
) -> Result<uuid::Uuid, sqlx::Error> {
    let subscriber_id = uuid::Uuid::new_v4();
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at, status)
    VALUES ($1, $2, $3, $4, 'pending_confirmation')
            "#,
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        chrono::Utc::now()
    )
    .execute(transaction)
    .await?;
    Ok(subscriber_id)
}

#[tracing::instrument(
    name = "Store subscription token in the database",
    skip(subscription_token, transaction)
)]
pub async fn store_token(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subscriber_id: uuid::Uuid,
    subscription_token: &str,
) -> Result<(), StoreTokenError> {
    sqlx::query!(
        r#"
    INSERT INTO subscription_tokens (subscription_token, subscriber_id)
    VALUES ($1, $2)
        "#,
        subscription_token,
        subscriber_id
    )
    .execute(transaction)
    .await
    .map_err(StoreTokenError)?;
    Ok(())
}

pub struct StoreTokenError(sqlx::Error);

impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database failure was encountered while trying to store a subscription token."
        )
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
