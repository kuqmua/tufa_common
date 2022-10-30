use crate::config_mods::config_struct::ConfigStruct;

impl ConfigStruct {
    pub fn get_server_address(&self) -> String {
        format!("{}:{}", self.server_ip, self.server_port)
    }
}
