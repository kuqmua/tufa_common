use tokio::task::JoinError;

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ServerWrapperErrorNamed<'a> {
    TcpListenerBind {
        #[eo_error_occurence]
        tcp_listener_bind: crate::traits::try_create_tcp_listener::TryCreateTcpListenerErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    TryGetPostgresPool {
        #[eo_display]
        try_get_postgres_pool: crate::traits::try_get_postgres_pool::TryGetPostgresPoolError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ApplicationBuild {
        #[eo_error_occurence]
        application_build: crate::repositories_types::tufa_server::try_build_actix_web_dev_server::TryBuildActixWebDevServer<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    TokioSpawn {
        #[eo_display]
        tokio_spawn: JoinError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    RunUntilStopped {
        #[eo_error_occurence]
        run_until_stopped: crate::repositories_types::tufa_server::server_wrapper::RunUntilStoppedErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum RunUntilStoppedErrorNamed<'a> {
    RunUntilStopped {
        #[eo_display]
        run_until_stopped: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    }
}