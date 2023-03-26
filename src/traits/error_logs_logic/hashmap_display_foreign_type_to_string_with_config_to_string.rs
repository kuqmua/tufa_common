pub trait HashMapDisplayForeignTypeToStringWithConfigToString<'a, ConfigGeneric> {
    fn hashmap_display_foreign_type_to_string_with_config_to_string(
        &self,
        config: &ConfigGeneric,
    ) -> String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric>
    HashMapDisplayForeignTypeToStringWithConfigToString<'a, ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: crate::traits::display_foreign_type::DisplayForeignType,
    HashMapValueGeneric:
        crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<
            'a,
            ConfigGeneric,
        >,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn hashmap_display_foreign_type_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(
                &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                    key.display_foreign_type(),
                    value.to_string_with_config_for_source_to_string_with_config(config),
                ),
            );
            acc
        });
        stringified.pop();
        stringified
    }
}
