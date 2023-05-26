pub trait GetServerAddress {
    fn get_server_address(&self) -> std::string::String;
}

impl<SelfGeneric> GetServerAddress for SelfGeneric
where
    Self: crate::common::config::config_fields::GetServerPort,
{
    fn get_server_address(&self) -> std::string::String {
        format!("127.0.0.1:{}", *self.get_server_port().port())
    }
}