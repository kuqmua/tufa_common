pub trait GetServerAddress<SelfGeneric> {
    fn get_server_address(&self) -> String;
}

impl<SelfGeneric> GetServerAddress<Self> for SelfGeneric
where
    Self: crate::traits::fields::GetServerIp + crate::traits::fields::GetServerPort,
{
    fn get_server_address(&self) -> String {
        format!("{}:{}", self.get_server_ip(), self.get_server_port())
    }
}
