use tokio::task::JoinError;

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ServerWrapperErrorNamed<'a> {
    ApplicationBuild {
        #[eo_error_occurence]
        application_build: crate::repositories_types::tufa_server::startup::ApplicationBuildErrorNamed<'a>,
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