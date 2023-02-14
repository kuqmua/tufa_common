pub trait FewToStringWithConfig<'a, ConfigGeneric> {
    fn few_to_string_with_config(&self, config: &ConfigGeneric) -> String;
}

impl<'a, VecElementGeneric, ConfigGeneric> FewToStringWithConfig<'a, ConfigGeneric>
    for Vec<VecElementGeneric>
where
    VecElementGeneric:
        crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime<
            'a,
            ConfigGeneric,
        >,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn few_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        crate::traits::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            String::from(""),
            |mut acc, vec_element| {
                acc.push_str(
                    &crate::traits::error_logs_logic::helpers::lines_space_backslash_addition(
                        vec_element.to_string_with_config_lifetime(config),
                    ),
                );
                acc
            },
        ))
    }
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric>
    FewToStringWithConfig<'a, ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric:
        crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime<
            'a,
            ConfigGeneric,
        >,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn few_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(
                &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                    key,
                    value.to_string_with_config_lifetime(config),
                ),
            );
            acc
        });
        stringified.pop();
        stringified
    }
}
