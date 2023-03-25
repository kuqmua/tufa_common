pub trait HashMapDisplayForeignTypeToStringWithoutConfigToString<'a> {
    fn hashmap_display_foreign_type_to_string_without_config_to_string(&self) -> String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric>
    HashMapDisplayForeignTypeToStringWithoutConfigToString<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: crate::traits::display_foreign_type::DisplayForeignType,
    HashMapValueGeneric:
        crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>,
{
    fn hashmap_display_foreign_type_to_string_without_config_to_string(&self) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(
                &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                    key.display_foreign_type(),
                    value.to_string_without_config(),
                ),
            );
            acc
        });
        stringified.pop();
        stringified
    }
}

pub trait HashMapToStringDisplayForeignTypeToStringWithoutConfigWithDeserialize<'a> {
    fn hashmap_to_string_display_foreign_type_to_string_without_config_with_deserialize(
        &self,
    ) -> String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric>
    HashMapToStringDisplayForeignTypeToStringWithoutConfigWithDeserialize<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: crate::traits::display_foreign_type::DisplayForeignType,
    HashMapValueGeneric:
        crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<'a>,
{
    fn hashmap_to_string_display_foreign_type_to_string_without_config_with_deserialize(&self) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(
                &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                    key.display_foreign_type(),
                    value.to_string_without_config_with_deserialize(),
                ),
            );
            acc
        });
        stringified.pop();
        stringified
    }
}
