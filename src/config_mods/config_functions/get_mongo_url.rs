use crate::config_mods::config_struct::ConfigStruct;

impl ConfigStruct {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn get_mongo_url(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}",
            self.mongo_first_handle_url_part,
            self.mongo_login,
            self.mongo_second_handle_url_part,
            self.mongo_password,
            self.mongo_third_handle_url_part,
            self.mongo_ip,
            self.mongo_fourth_handle_url_part,
            self.mongo_port,
            self.mongo_fifth_handle_url_part,
            self.mongo_params
        )
    }
}
