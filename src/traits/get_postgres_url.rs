pub trait GetPostgresUrl<SelfGeneric> {
    fn get_postgres_url(&self) -> String;
}

impl<SelfGeneric> GetPostgresUrl<Self> for SelfGeneric
where
    Self: crate::traits::fields::GetPostgresFirstHandleUrlPart
        + crate::traits::fields::GetPostgresLogin
        + crate::traits::fields::GetPostgresSecondHandleUrlPart
        + crate::traits::fields::GetPostgresPassword
        + crate::traits::fields::GetPostgresThirdHandleUrlPart
        + crate::traits::fields::GetPostgresIp
        + crate::traits::fields::GetPostgresFourthHandleUrlPart
        + crate::traits::fields::GetPostgresPort
        + crate::traits::fields::GetPostgresFifthHandleUrlPart
        + crate::traits::fields::GetPostgresDb
        + crate::traits::fields::GetPostgresSixthHandleUrlPart
        + crate::traits::fields::GetPostgresParams,
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
