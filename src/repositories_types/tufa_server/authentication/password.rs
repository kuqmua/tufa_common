#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetStoredCredentialsErrorNamed {
    FetchOptional {
        #[eo_display]
        fetch_optional: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ValidateCredentialsErrorNamed {
    GetStoredCredentials {
        #[eo_error_occurence]
        get_stored_credentials: GetStoredCredentialsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    SpawnBlockingWithTracing {
        #[eo_display]
        spawn_blocking_with_tracing: tokio::task::JoinError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    VerifyPasswordHash {
        #[eo_error_occurence]
        spawn_blocking_with_tracing: VerifyPasswordHashErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnknownUsername {
        #[eo_display_with_serialize_deserialize]
        message: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum VerifyPasswordHashErrorNamed {
    ExposeSecret {
        #[eo_display]
        expose_secret: argon2::password_hash::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    InvalidPassword {
        #[eo_display]
        invalid_password: argon2::password_hash::Error, //todo - should add here or just addd message?
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ChangePasswordErrorNamed {
    SpawnBlockingWithTracing {
        #[eo_display]
        spawn_blocking_with_tracing: tokio::task::JoinError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ComputePasswordHash {
        #[eo_error_occurence]
        compute_password_hash: ComputePasswordHashErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PostgresQuery {
        #[eo_display]
        query_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ComputePasswordHashErrorNamed {
    PasswordHash {
        #[eo_display]
        argon2_password_hash_error: argon2::password_hash::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
