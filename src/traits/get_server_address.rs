pub trait GetServerAddress {
    fn get_server_address(&self) -> std::string::String;
}

impl<SelfGeneric> GetServerAddress for SelfGeneric
where
    Self: crate::traits::config_fields::GetServerPort,
{
    fn get_server_address(&self) -> std::string::String {
        format!("localhost:{}", *self.get_server_port().port())
    }
}