use crate::traits::fields::GetMongoFifthHandleUrlPart;
use crate::traits::fields::GetMongoFirstHandleUrlPart;
use crate::traits::fields::GetMongoFourthHandleUrlPart;
use crate::traits::fields::GetMongoIp;
use crate::traits::fields::GetMongoLogin;
use crate::traits::fields::GetMongoParams;
use crate::traits::fields::GetMongoPassword;
use crate::traits::fields::GetMongoPort;
use crate::traits::fields::GetMongoSecondHandleUrlPart;
use crate::traits::fields::GetMongoThirdHandleUrlPart;

pub trait GetMongoUrl<SomeGenericParam> {
    fn get_mongo_url(&self) -> String;
}

impl<T> GetMongoUrl<Self> for T
where
    Self: GetMongoFirstHandleUrlPart
        + GetMongoSecondHandleUrlPart
        + GetMongoThirdHandleUrlPart
        + GetMongoFourthHandleUrlPart
        + GetMongoFifthHandleUrlPart
        + GetMongoFifthHandleUrlPart
        + GetMongoIp
        + GetMongoPort
        + GetMongoLogin
        + GetMongoPassword
        + GetMongoParams,
{
    fn get_mongo_url(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}",
            self.get_mongo_first_handle_url_part(),
            self.get_mongo_login(),
            self.get_mongo_second_handle_url_part(),
            self.get_mongo_password(),
            self.get_mongo_third_handle_url_part(),
            self.get_mongo_ip(),
            self.get_mongo_fourth_handle_url_part(),
            self.get_mongo_port(),
            self.get_mongo_fifth_handle_url_part(),
            self.get_mongo_params()
        )
    }
}
