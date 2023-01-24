pub trait GetServerAddress {
    fn get_server_address(&self) -> String;
}

impl<SelfGeneric> GetServerAddress for SelfGeneric
where
    Self: crate::traits::fields::GetServerIp + crate::traits::fields::GetServerPort,
{
    fn get_server_address(&self) -> String {
        format!("http://{}:{}", self.get_server_ip(), self.get_server_port())
    }
}
