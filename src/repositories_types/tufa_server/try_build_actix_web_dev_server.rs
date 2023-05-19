#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryBuildActixWebDevServer<'a> {
    TcpListenerBind {
        #[eo_error_occurence]
        tcp_listener_bind: crate::traits::try_create_tcp_listener::TryCreateTcpListenerErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    TcpListenerLocalAddress {
        #[eo_display]
        tcp_listener_local_address: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
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


