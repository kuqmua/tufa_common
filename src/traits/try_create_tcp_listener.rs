pub trait TryCreateTcpListener<'a> {
    fn try_create_tcp_listener(&self) -> Result<std::net::TcpListener, Box<crate::traits::try_create_tcp_listener::TryCreateTcpListenerErrorNamed<'a>>>;
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryCreateTcpListenerErrorNamed<'a> {
    TcpListenerBind {
        #[eo_display]
        tcp_listener_bind: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a, SelfGeneric> TryCreateTcpListener<'a> for SelfGeneric
where
    Self: crate::traits::get_server_address::GetServerAddress,
{
    fn try_create_tcp_listener(&self) -> Result<std::net::TcpListener, Box<crate::traits::try_create_tcp_listener::TryCreateTcpListenerErrorNamed<'a>>> {
        match std::net::TcpListener::bind(&self.get_server_address()) {
            Ok(listener) => Ok(listener),
            Err(e) => {
                return Err(Box::new(crate::traits::try_create_tcp_listener::TryCreateTcpListenerErrorNamed::TcpListenerBind {
                    tcp_listener_bind: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }))
            }
        }
    }
}