use crate::config_mods::traits::mongo::get_mongo_fifth_handle_url_part::GetMongoFifthHandleUrlPart;
use crate::config_mods::traits::mongo::get_mongo_first_handle_url_part::GetMongoFirstHandleUrlPart;
use crate::config_mods::traits::mongo::get_mongo_fourth_handle_url_part::GetMongoFourthHandleUrlPart;
use crate::config_mods::traits::mongo::get_mongo_ip::GetMongoIp;
use crate::config_mods::traits::mongo::get_mongo_login::GetMongoLogin;
use crate::config_mods::traits::mongo::get_mongo_params::GetMongoParams;
use crate::config_mods::traits::mongo::get_mongo_password::GetMongoPassword;
use crate::config_mods::traits::mongo::get_mongo_port::GetMongoPort;
use crate::config_mods::traits::mongo::get_mongo_second_handle_url_part::GetMongodSecondHandleUrlPart;
use crate::config_mods::traits::mongo::get_mongo_third_handle_url_part::GetMongoThirdHandleUrlPart;

pub trait GetMongoUrl<SomeGenericParam> {
    fn get_mongo_url(&self) -> String;
}

impl<T> GetMongoUrl<Self> for T
where
    Self: GetMongoFirstHandleUrlPart
        + GetMongodSecondHandleUrlPart
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
