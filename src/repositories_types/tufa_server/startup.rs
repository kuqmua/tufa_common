pub fn get_connection_pool(configuration: &crate::repositories_types::tufa_server::configuration::PostgresDatabaseSettings) -> sqlx::PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ApplicationBuildErrorNamed<'a> {
    TcpListenerBind {
        #[eo_display]
        tcp_listener_bind: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    TcpListenerLocalAddress {
        #[eo_display]
        tcp_listener_local_address: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ApplicationRun {
        #[eo_error_occurence]
        application_run: crate::repositories_types::tufa_server::startup::ApplicationRunErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ApplicationRunErrorNamed<'a> {
    NewRedisSessionStore {
        #[eo_display_with_serialize_deserialize]
        new_redis_session_store: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    HttpServerListen {
        #[eo_display]
        http_server_listen: std::io::Error,
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


