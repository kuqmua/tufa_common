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
        run_until_stopped: crate::repositories_types::tufa_server::startup::RunUntilStoppedErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}