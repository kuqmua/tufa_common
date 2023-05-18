pub trait GetRedisUrl {
    fn get_redis_url(&self) -> secrecy::Secret<std::string::String>;
}

impl<SelfGeneric> GetRedisUrl for SelfGeneric
where
    Self: crate::traits::config_fields::GetRedisIp
        + crate::traits::config_fields::GetRedisPort,
{
    fn get_redis_url(&self) -> secrecy::Secret<std::string::String> {
        secrecy::Secret::new(format!("redis://{}:{}", self.get_redis_ip(), self.get_redis_port()))
    }
}