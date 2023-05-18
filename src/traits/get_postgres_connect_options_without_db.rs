pub trait GetPostgresConnectOptionsWithoutDb {
    fn get_postgres_connect_options_without_db(&self) -> sqlx::postgres::PgConnectOptions;
}

impl<SelfGeneric> GetPostgresConnectOptionsWithoutDb for SelfGeneric
where
    Self: crate::traits::config_fields::GetRequireSsl
        + crate::traits::config_fields::GetPostgresIp
        + crate::traits::config_fields::GetPostgresLogin
        + crate::traits::config_fields::GetPostgresPassword
        + crate::traits::config_fields::GetPostgresPort,
{
    fn get_postgres_connect_options_without_db(&self) -> sqlx::postgres::PgConnectOptions {
        sqlx::postgres::PgConnectOptions::new()
            .host(&self.get_postgres_ip())
            .username(&self.get_postgres_login())
            .password({
                use secrecy::ExposeSecret;//todo password must be store as secrecy::Secret not just a String
                secrecy::Secret::new(self.get_postgres_password().clone()).expose_secret()
            })
            .port(*self.get_postgres_port().port())
            .ssl_mode(match self.get_require_ssl() {//todo use sqlx::postgres::PgSslMode instead of bool in Config
                true => sqlx::postgres::PgSslMode::Require,
                false => sqlx::postgres::PgSslMode::Prefer,
            })
    }
}
