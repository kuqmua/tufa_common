pub trait GetApplicationSettings {
    fn get_application_settings(&self) -> crate::repositories_types::tufa_server::settings::ApplicationSettings;
}

impl<SelfGeneric> GetApplicationSettings for SelfGeneric
where
    Self: crate::traits::config_fields::GetServerPort
        + crate::traits::config_fields::GetHmacSecret,
{
    fn get_application_settings(&self) -> crate::repositories_types::tufa_server::settings::ApplicationSettings {
        crate::repositories_types::tufa_server::settings::ApplicationSettings {
            port: *self.get_server_port().port(),
            hmac_secret: secrecy::Secret::new(self.get_hmac_secret().clone()),
        }
    }
}
