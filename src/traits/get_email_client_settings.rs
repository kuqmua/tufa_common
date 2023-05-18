pub trait GetEmailClientSettings {
    fn get_email_client_settings(&self) -> crate::repositories_types::tufa_server::configuration::EmailClientSettings;
}

impl<SelfGeneric> GetEmailClientSettings for SelfGeneric
where
    Self: crate::traits::config_fields::GetBaseUrl
        + crate::traits::config_fields::GetPostgresConnectionTimeout,
{
    fn get_email_client_settings(&self) -> crate::repositories_types::tufa_server::configuration::EmailClientSettings {
        crate::repositories_types::tufa_server::configuration::EmailClientSettings {
            base_url: self.get_base_url().clone(),
            sender_email: "test@gmail.com".to_string(),//todo 
            authorization_token: secrecy::Secret::new("my-secret-token".to_string()),//todo
            timeout_milliseconds: *self.get_postgres_connection_timeout(),//10000
        }
    }
}
