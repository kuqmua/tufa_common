use crate::config_mods::config_struct::ConfigStruct;
use crate::config_mods::traits::get_mongo_url_trait::GetMongoUrl;
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

impl GetMongoFifthHandleUrlPart for ConfigStruct {
    fn get_mongo_fifth_handle_url_part(&self) -> &String {
        &self.mongo_fifth_handle_url_part
    }
}

impl GetMongoFirstHandleUrlPart for ConfigStruct {
    fn get_mongo_first_handle_url_part(&self) -> &String {
        &self.mongo_first_handle_url_part
    }
}

impl GetMongoFourthHandleUrlPart for ConfigStruct {
    fn get_mongo_fourth_handle_url_part(&self) -> &String {
        &self.mongo_fourth_handle_url_part
    }
}

impl GetMongoIp for ConfigStruct {
    fn get_mongo_ip(&self) -> &String {
        &self.mongo_ip
    }
}

impl GetMongoLogin for ConfigStruct {
    fn get_mongo_login(&self) -> &String {
        &self.mongo_login
    }
}

impl GetMongoParams for ConfigStruct {
    fn get_mongo_params(&self) -> &String {
        &self.mongo_params
    }
}

impl GetMongoPassword for ConfigStruct {
    fn get_mongo_password(&self) -> &String {
        &self.mongo_password
    }
}

impl GetMongoPort for ConfigStruct {
    fn get_mongo_port(&self) -> u16 {
        self.mongo_port
    }
}

impl GetMongodSecondHandleUrlPart for ConfigStruct {
    fn get_mongo_second_handle_url_part(&self) -> &String {
        &self.mongo_second_handle_url_part
    }
}

impl GetMongoThirdHandleUrlPart for ConfigStruct {
    fn get_mongo_third_handle_url_part(&self) -> &String {
        &self.mongo_third_handle_url_part
    }
}
