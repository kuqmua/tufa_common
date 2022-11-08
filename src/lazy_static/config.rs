use crate::config_mods::config_struct::ConfigStruct;
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<ConfigStruct> = Lazy::new(|| {
    ConfigStruct::new()
        .expect("cannot create config")
        .wrap_config_checks()
        .expect("wrap_config_checks error")
});
