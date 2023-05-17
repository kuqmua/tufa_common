pub trait GetApplicationSettings<'a> {
    fn get_application_settings(&self) -> crate::repositories_types::tufa_server::configuration::ApplicationSettings;
}

impl<'a, SelfGeneric> GetApplicationSettings<'a> for SelfGeneric
where
    Self: crate::traits::config_fields::GetServerPort
        + crate::traits::config_fields::GetHmacSecret,
{
    fn get_application_settings(&self) -> crate::repositories_types::tufa_server::configuration::ApplicationSettings {
        crate::repositories_types::tufa_server::configuration::ApplicationSettings {
            port: *self.get_server_port().port(),
            hmac_secret: secrecy::Secret::new(self.get_hmac_secret().clone()),
        }
    }
}
