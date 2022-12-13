pub trait GetMongoUrl<SelfGeneric> {
    fn get_mongo_url(&self) -> String;
}

impl<SelfGeneric> GetMongoUrl<Self> for SelfGeneric
where
    Self: crate::traits::fields::GetMongoFirstHandleUrlPart
        + crate::traits::fields::GetMongoSecondHandleUrlPart
        + crate::traits::fields::GetMongoThirdHandleUrlPart
        + crate::traits::fields::GetMongoFourthHandleUrlPart
        + crate::traits::fields::GetMongoFifthHandleUrlPart
        + crate::traits::fields::GetMongoIp
        + crate::traits::fields::GetMongoPort
        + crate::traits::fields::GetMongoLogin
        + crate::traits::fields::GetMongoPassword
        + crate::traits::fields::GetMongoParams,
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
