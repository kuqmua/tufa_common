use crate::traits::fields::GetPostgresDb;
use crate::traits::fields::GetPostgresFifthHandleUrlPart;
use crate::traits::fields::GetPostgresFirstHandleUrlPart;
use crate::traits::fields::GetPostgresFourthHandleUrlPart;
use crate::traits::fields::GetPostgresIp;
use crate::traits::fields::GetPostgresLogin;
use crate::traits::fields::GetPostgresParams;
use crate::traits::fields::GetPostgresPassword;
use crate::traits::fields::GetPostgresPort;
use crate::traits::fields::GetPostgresSecondHandleUrlPart;
use crate::traits::fields::GetPostgresSixthHandleUrlPart;
use crate::traits::fields::GetPostgresThirdHandleUrlPart;

pub trait GetPostgresUrl<SomeGenericParam> {
    fn get_postgres_url(&self) -> String;
}

impl<T> GetPostgresUrl<Self> for T
where
    Self: GetPostgresFirstHandleUrlPart
        + GetPostgresLogin
        + GetPostgresSecondHandleUrlPart
        + GetPostgresPassword
        + GetPostgresThirdHandleUrlPart
        + GetPostgresIp
        + GetPostgresFourthHandleUrlPart
        + GetPostgresPort
        + GetPostgresFifthHandleUrlPart
        + GetPostgresDb
        + GetPostgresSixthHandleUrlPart
        + GetPostgresParams,
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
