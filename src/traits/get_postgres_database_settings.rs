pub trait GetPostgresDatabaseSettings {
    fn get_postgres_database_settings(&self) -> crate::repositories_types::tufa_server::configuration::PostgresDatabaseSettings;
}

impl<SelfGeneric> GetPostgresDatabaseSettings for SelfGeneric
where
    Self: crate::traits::config_fields::GetPostgresIp
        + crate::traits::config_fields::GetPostgresPort
        + crate::traits::config_fields::GetPostgresLogin
        + crate::traits::config_fields::GetPostgresPassword
        + crate::traits::config_fields::GetPostgresDb
        + crate::traits::config_fields::GetRequireSsl,
{
    fn get_postgres_database_settings(&self) -> crate::repositories_types::tufa_server::configuration::PostgresDatabaseSettings {
        crate::repositories_types::tufa_server::configuration::PostgresDatabaseSettings {
            host: self.get_postgres_ip().clone(),
            port: *self.get_postgres_port().port(),
            username: self.get_postgres_login().clone(),
            password: secrecy::Secret::new(self.get_postgres_password().clone()),
            database_name: self.get_postgres_db().clone(),
            require_ssl: *self.get_require_ssl(),
        }
    }
}
