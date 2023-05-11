pub trait GetMongoUrl<SelfGeneric> {
    fn get_mongo_url(&self) -> String;
}

impl<SelfGeneric> GetMongoUrl<Self> for SelfGeneric
where
    Self: crate::traits::config_fields::GetMongoFirstHandleUrlPart
        + crate::traits::config_fields::GetMongoSecondHandleUrlPart
        + crate::traits::config_fields::GetMongoThirdHandleUrlPart
        + crate::traits::config_fields::GetMongoFourthHandleUrlPart
        + crate::traits::config_fields::GetMongoFifthHandleUrlPart
        + crate::traits::config_fields::GetMongoIp
        + crate::traits::config_fields::GetMongoPort
        + crate::traits::config_fields::GetMongoLogin
        + crate::traits::config_fields::GetMongoPassword
        + crate::traits::config_fields::GetMongoParams,
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
