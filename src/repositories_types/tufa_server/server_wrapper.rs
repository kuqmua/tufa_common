#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ServerWrapperErrorNamed {
    TryGetPostgresPool {
        #[eo_display]
        try_get_postgres_pool:
            crate::common::config::try_get_postgres_pool::TryGetPostgresPoolError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // TryGetRedisSessionStorage {
    //     #[eo_display]
    //     try_get_redis_session_storage: crate::common::config::try_get_redis_session_storage::TryGetRedisSessionStorageError,
    //     code_occurence: crate::common::code_occurence::CodeOccurence,
    // },
    BuildServer {
        #[eo_error_occurence]
        build_server: crate::repositories_types::tufa_server::try_build_server::TryBuildServer,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
