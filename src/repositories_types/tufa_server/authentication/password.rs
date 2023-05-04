#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_stored_credentials(
    username: &str,
    pool: &sqlx::PgPool,
) -> Result<Option<(uuid::Uuid, secrecy::Secret<String>)>, anyhow::Error> {
    use anyhow::Context;
    let row = sqlx::query!(
        r#"
        SELECT user_id, password_hash
        FROM users
        WHERE username = $1
        "#,
        username,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to performed a query to retrieve stored credentials.")?
    .map(|row| (row.user_id, secrecy::Secret::new(row.password_hash)));
    Ok(row)
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: crate::common::postgres_credentials::PostgresCredentials,
    pool: &sqlx::PgPool,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = secrecy::Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );
    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credentials.username, pool).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }
    {   
        use anyhow::Context;
        crate::repositories_types::tufa_server::telemetry::spawn_blocking_with_tracing::spawn_blocking_with_tracing(move || {
            verify_password_hash(expected_password_hash, credentials.password)
        })
        .await
        .context("Failed to spawn blocking task.")
    }??;
    user_id
        .ok_or_else(|| anyhow::anyhow!("Unknown username."))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(
    name = "Validate credentials",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: secrecy::Secret<String>,
    password_candidate: secrecy::Secret<String>,
) -> Result<(), AuthError> {
    let expected_password_hash = {
        use anyhow::Context;
        argon2::PasswordHash::new({
            use secrecy::ExposeSecret;
            expected_password_hash.expose_secret()
        })
        .context("Failed to parse hash in PHC string format.")
    }?;
    {
        use anyhow::Context;
        {
            use argon2::PasswordVerifier;
            argon2::Argon2::default()
            .verify_password(
                {
                    use secrecy::ExposeSecret;
                    password_candidate.expose_secret()
                }.as_bytes(),
                &expected_password_hash,
            )
        }
        .context("Invalid password.")
    }
    .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Change password", skip(password, pool))]
pub async fn change_password(
    user_id: uuid::Uuid,
    password: secrecy::Secret<String>,
    pool: &sqlx::PgPool,
) -> Result<(), anyhow::Error> {
    let password_hash = {
        use anyhow::Context;
        crate::repositories_types::tufa_server::telemetry::spawn_blocking_with_tracing::spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await?
        .context("Failed to hash password")
    }?;
    {
        use anyhow::Context;
        sqlx::query!(
            r#"
            UPDATE users
            SET password_hash = $1
            WHERE user_id = $2
            "#,
            {
                use secrecy::ExposeSecret;
                password_hash.expose_secret()
            },
            user_id
        )
        .execute(pool)
        .await
        .context("Failed to change user's password in the database.")
    }?;
    Ok(())
}

fn compute_password_hash(password: secrecy::Secret<String>) -> Result<secrecy::Secret<String>, anyhow::Error> {
    let salt = argon2::password_hash::SaltString::generate(&mut rand::thread_rng());
    let password_hash = {
        use argon2::PasswordHasher;
        argon2::Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password({
            use secrecy::ExposeSecret;
            password.expose_secret()
        }.as_bytes(), &salt)
    }?
    .to_string();
    Ok(secrecy::Secret::new(password_hash))
}
