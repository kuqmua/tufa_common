pub trait GetSettings {
    fn get_settings(&self) -> crate::repositories_types::tufa_server::settings::Settings;
}

impl<SelfGeneric> GetSettings for SelfGeneric
where
    Self: crate::traits::get_postgres_database_settings::GetPostgresDatabaseSettings
        + crate::traits::get_application_settings::GetApplicationSettings
        + crate::traits::get_email_client_settings::GetEmailClientSettings
        + crate::traits::get_redis_url::GetRedisUrl,
{
    fn get_settings(&self) -> crate::repositories_types::tufa_server::settings::Settings {
        crate::repositories_types::tufa_server::settings::Settings {
            database: self.get_postgres_database_settings(),
            application: self.get_application_settings(),
            email_client: self.get_email_client_settings(),
            redis_uri: secrecy::Secret::new(self.get_redis_url()),
        }
    }
}