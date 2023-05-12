#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetStoredCredentialsErrorNamed<'a> {
    FetchOptional {
        #[eo_display]
        fetch_optional: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_stored_credentials<'a>(
    username: &str,
    pool: &sqlx::PgPool,
) -> Result<Option<(uuid::Uuid, secrecy::Secret<String>)>, GetStoredCredentialsErrorNamed<'a>> {
    match sqlx::query!(
        r#"
        SELECT user_id, password_hash
        FROM users
        WHERE username = $1
        "#,
        username,
    )
    .fetch_optional(pool)
    .await
    {
        Err(e) => Err(GetStoredCredentialsErrorNamed::FetchOptional {
            fetch_optional: e,
            code_occurence: crate::code_occurence_tufa_common!()
        }),
        Ok(option_record) => Ok(option_record.map(|row| (row.user_id, secrecy::Secret::new(row.password_hash)))),
    }
}

pub async fn validate_credentials(
    credentials: crate::common::postgres_credentials::PostgresCredentials,
    pool: &sqlx::PgPool,
) -> Result<uuid::Uuid, AuthError> {
    todo!()
    // let mut user_id = None;
    // let mut expected_password_hash = secrecy::Secret::new(
    //     "$argon2id$v=19$m=15000,t=2,p=1$\
    //     gZiV/M1gPc22ElAH/Jh1Hw$\
    //     CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
    //         .to_string(),
    // );
    // // //

    // // match get_stored_credentials(&credentials.username, pool).await {
    // //     Err(e) => todo!(),
    // //     Ok(_) => todo!(),
    // // }

    // // 
    // if let Some((stored_user_id, stored_password_hash)) =
    //     get_stored_credentials(&credentials.username, pool).await?
    // {
    //     user_id = Some(stored_user_id);
    //     expected_password_hash = stored_password_hash;
    // }
    // {   
    //     use anyhow::Context;
    //     crate::repositories_types::tufa_server::telemetry::spawn_blocking_with_tracing::spawn_blocking_with_tracing(move || {
    //         verify_password_hash(expected_password_hash, credentials.password)
    //     })
    //     .await
    //     .context("Failed to spawn blocking task.")
    // }??;
    // user_id
    //     .ok_or_else(|| anyhow::anyhow!("Unknown username."))
    //     .map_err(AuthError::InvalidCredentials)
}


#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum VerifyPasswordHashErrorNamed<'a> {
    ExposeSecret {
        #[eo_display]
        expose_secret: argon2::password_hash::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    InvalidPassword {
        #[eo_display]
        invalid_password: argon2::password_hash::Error,//todo - should add here or just addd message?
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    }
}

#[tracing::instrument(
    name = "Validate credentials",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash<'a>(
    expected_password_hash: secrecy::Secret<String>,
    password_candidate: secrecy::Secret<String>,
) -> Result<(), VerifyPasswordHashErrorNamed<'a>> {
    match argon2::PasswordHash::new({
        use secrecy::ExposeSecret;
        expected_password_hash.expose_secret()
    }) {
        Err(e) => Err(VerifyPasswordHashErrorNamed::ExposeSecret {
            expose_secret: e,
            code_occurence: crate::code_occurence_tufa_common!()
        }),
        Ok(expected_password_hash) => match {
            use argon2::PasswordVerifier;
            argon2::Argon2::default()
            .verify_password(
                {
                    use secrecy::ExposeSecret;
                    password_candidate.expose_secret()
                }.as_bytes(),
                &expected_password_hash,
            )
        } {
            Err(e) => Err(VerifyPasswordHashErrorNamed::InvalidPassword {
                invalid_password: e,
                code_occurence: crate::code_occurence_tufa_common!()
            }),
            Ok(_) => Ok(())
        }
    }
    // let expected_password_hash = {
    //     use anyhow::Context;
    //     argon2::PasswordHash::new({
    //         use secrecy::ExposeSecret;
    //         expected_password_hash.expose_secret()
    //     })
    //     .context("Failed to parse hash in PHC string format.")
    // }?;
    // {
    //     use anyhow::Context;
    //     {
    //         use argon2::PasswordVerifier;
    //         argon2::Argon2::default()
    //         .verify_password(
    //             {
    //                 use secrecy::ExposeSecret;
    //                 password_candidate.expose_secret()
    //             }.as_bytes(),
    //             &expected_password_hash,
    //         )
    //     }
    //     .context("Invalid password.")
    // }
    // .map_err(AuthError::InvalidCredentials)
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ChangePasswordErrorNamed<'a> {
    SpawnBlockingWithTracing {
        #[eo_display]
        spawn_blocking_with_tracing: tokio::task::JoinError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ComputePasswordHash {
        #[eo_error_occurence]
        compute_password_hash: ComputePasswordHashErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresQuery {
        #[eo_display]
        query_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    }
}

pub async fn change_password<'a>(
    user_id: uuid::Uuid,
    password: secrecy::Secret<String>,
    pool: &sqlx::PgPool,
) -> Result<(), ChangePasswordErrorNamed<'a>> {
    match crate::repositories_types::tufa_server::telemetry::spawn_blocking_with_tracing::spawn_blocking_with_tracing(
        move || compute_password_hash(password)
    ).await {
        Err(e) => Err(ChangePasswordErrorNamed::SpawnBlockingWithTracing {
            spawn_blocking_with_tracing: e,
            code_occurence: crate::code_occurence_tufa_common!()
        }),
        Ok(res) => match res {
            Err(e) => Err(ChangePasswordErrorNamed::ComputePasswordHash {
                compute_password_hash: e,
                code_occurence: crate::code_occurence_tufa_common!()
            }),
            Ok(password_hash) => match sqlx::query!(
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
            .await {
                Err(e) => Err(ChangePasswordErrorNamed::PostgresQuery {
                    query_error: e,
                    code_occurence: crate::code_occurence_tufa_common!()
                }),
                Ok(_) => Ok(()),
            }
        },
    }
}


#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ComputePasswordHashErrorNamed<'a> {
    PasswordHash {
        #[eo_display]
        argon2_password_hash_error: argon2::password_hash::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

fn compute_password_hash<'a>(password: secrecy::Secret<String>) -> Result<secrecy::Secret<String>, ComputePasswordHashErrorNamed<'a>> {
    use argon2::PasswordHasher;
    match argon2::Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password({
        use secrecy::ExposeSecret;
        password.expose_secret()
    }.as_bytes(), &argon2::password_hash::SaltString::generate(&mut rand::thread_rng())) {
        Ok(password_hash) => Ok(secrecy::Secret::new(password_hash.to_string())),
        Err(e) => Err(
            ComputePasswordHashErrorNamed::PasswordHash {
                argon2_password_hash_error: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        ),
    }
}
