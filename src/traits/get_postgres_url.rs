pub trait GetPostgresUrl<SelfGeneric> {
    fn get_postgres_url(&self) -> String;
}

impl<SelfGeneric> GetPostgresUrl<Self> for SelfGeneric
where
    Self: crate::traits::config_fields::GetPostgresFirstHandleUrlPart
        + crate::traits::config_fields::GetPostgresLogin
        + crate::traits::config_fields::GetPostgresSecondHandleUrlPart
        + crate::traits::config_fields::GetPostgresPassword
        + crate::traits::config_fields::GetPostgresThirdHandleUrlPart
        + crate::traits::config_fields::GetPostgresIp
        + crate::traits::config_fields::GetPostgresFourthHandleUrlPart
        + crate::traits::config_fields::GetPostgresPort
        + crate::traits::config_fields::GetPostgresFifthHandleUrlPart
        + crate::traits::config_fields::GetPostgresDb
        + crate::traits::config_fields::GetPostgresSixthHandleUrlPart
        + crate::traits::config_fields::GetPostgresParams,
{
    fn get_postgres_url(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            self.get_postgres_first_handle_url_part(),
            self.get_postgres_login(),
            self.get_postgres_second_handle_url_part(),
            self.get_postgres_password(),
            self.get_postgres_third_handle_url_part(),
            self.get_postgres_ip(),
            self.get_postgres_fourth_handle_url_part(),
            self.get_postgres_port(),
            self.get_postgres_fifth_handle_url_part(),
            self.get_postgres_db(),
            self.get_postgres_sixth_handle_url_part(),
            self.get_postgres_params(),
        )
    }
}
