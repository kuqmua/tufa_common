#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetStoredCredentialsErrorNamed<'a> {
    FetchOptional {
        #[eo_display]
        fetch_optional: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ValidateCredentialsErrorNamed<'a> {
    GetStoredCredentials {
        #[eo_error_occurence]
        get_stored_credentials: GetStoredCredentialsErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    SpawnBlockingWithTracing {
        #[eo_display]
        spawn_blocking_with_tracing: tokio::task::JoinError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    VerifyPasswordHash {
        #[eo_error_occurence]
        spawn_blocking_with_tracing: VerifyPasswordHashErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    UnknownUsername {
        #[eo_display_with_serialize_deserialize]
        message: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ComputePasswordHashErrorNamed<'a> {
    PasswordHash {
        #[eo_display]
        argon2_password_hash_error: argon2::password_hash::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
