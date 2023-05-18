pub trait GetPostgresUrl<SelfGeneric> {
    fn get_postgres_url(&self) -> String;
}

impl<SelfGeneric> GetPostgresUrl<Self> for SelfGeneric
where
    Self: crate::traits::config_fields::GetPostgresLogin
        + crate::traits::config_fields::GetPostgresPassword
        + crate::traits::config_fields::GetPostgresIp
        + crate::traits::config_fields::GetPostgresFourthHandleUrlPart
        + crate::traits::config_fields::GetPostgresPort
        + crate::traits::config_fields::GetPostgresFifthHandleUrlPart
        + crate::traits::config_fields::GetPostgresDb
        + crate::traits::config_fields::GetPostgresParams,
{
    fn get_postgres_url(&self) -> String {//todo maybe use secrecy::Secret<std::string::String>
        format!(
            "postgres://{}:{}@{}{}{}{}{}{}",
            self.get_postgres_login(),
            {
                use secrecy::ExposeSecret;
                self.get_postgres_password().expose_secret()
            },
            self.get_postgres_ip(),
            self.get_postgres_fourth_handle_url_part(),
            self.get_postgres_port(),
            self.get_postgres_fifth_handle_url_part(),
            self.get_postgres_db(),
            self.get_postgres_params(),
        )
    }
}
