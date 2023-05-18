pub trait GetSettings {
    fn get_settings(&self) -> crate::repositories_types::tufa_server::settings::Settings;
}

impl<SelfGeneric> GetSettings for SelfGeneric
where
    Self: crate::traits::get_redis_url::GetRedisUrl,
{
    fn get_settings(&self) -> crate::repositories_types::tufa_server::settings::Settings {
        crate::repositories_types::tufa_server::settings::Settings {
            redis_uri: secrecy::Secret::new(self.get_redis_url()),
        }
    }
}