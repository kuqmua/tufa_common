use crate::config_mods::config_struct::ConfigStruct;

impl ConfigStruct {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn get_postgres_url(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            self.postgres_first_handle_url_part,
            self.postgres_login,
            self.postgres_second_handle_url_part,
            self.postgres_password,
            self.postgres_third_handle_url_part,
            self.postgres_ip,
            self.postgres_fourth_handle_url_part,
            self.postgres_port,
            self.postgres_fifth_handle_url_part,
            self.postgres_db,
            self.postgres_sixth_handle_url_part,
            self.postgres_params
        )
    }
}
