pub static CONFIG: once_cell::sync::Lazy<crate::config_mods::config_struct::ConfigStruct> = once_cell::sync::Lazy::new(|| {
    crate::config_mods::config_struct::ConfigStruct::new()
        .expect("cannot create config")
        .wrap_config_checks()
        .expect("wrap_config_checks error")
});
