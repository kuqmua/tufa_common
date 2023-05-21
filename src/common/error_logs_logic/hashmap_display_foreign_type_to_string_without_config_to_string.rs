pub trait HashMapDisplayForeignTypeToStringWithoutConfigToString<'a> {
    fn hashmap_display_foreign_type_to_string_without_config_to_string(&self) -> String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric>
    HashMapDisplayForeignTypeToStringWithoutConfigToString<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: crate::common::display_foreign_type::DisplayForeignType,
    HashMapValueGeneric:
        crate::common::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>,
{
    fn hashmap_display_foreign_type_to_string_without_config_to_string(&self) -> String {
        crate::common::error_logs_logic::helpers::error_occurence_hashmap_formatter(
            self.iter().fold(String::from(""), |mut acc, (key, value)| {
                acc.push_str(
                    &crate::common::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                        key.display_foreign_type(),
                        value.to_string_without_config(),
                    ),
                );
                acc
            })
        )

    }
}

pub trait HashMapToStringDisplayForeignTypeToStringWithoutConfigWithSerializeDeserialize<'a> {
    fn hashmap_to_string_display_foreign_type_to_string_without_config_with_serialize_deserialize(
        &self,
    ) -> String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric>
    HashMapToStringDisplayForeignTypeToStringWithoutConfigWithSerializeDeserialize<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: crate::common::display_foreign_type::DisplayForeignType,
    HashMapValueGeneric:
        crate::common::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithSerializeDeserialize<'a>,
{
    fn hashmap_to_string_display_foreign_type_to_string_without_config_with_serialize_deserialize(&self) -> String {
        crate::common::error_logs_logic::helpers::error_occurence_hashmap_formatter(
            self.iter().fold(String::from(""), |mut acc, (key, value)| {
                acc.push_str(
                    &crate::common::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                        key.display_foreign_type(),
                        value.to_string_without_config_with_serialize_deserialize(),
                    ),
                );
                acc
            })
        )
    }
}
