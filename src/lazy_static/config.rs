use crate::config_mods::config_struct::ConfigStruct;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: ConfigStruct = ConfigStruct::new()
        .expect("cannot create config")
        .wrap_config_checks()
        .expect("wrap_config_checks error");
}
