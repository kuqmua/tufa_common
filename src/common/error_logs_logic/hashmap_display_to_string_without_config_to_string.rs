pub trait HashMapDisplayToStringWithoutConfigToString<'a> {
    fn hashmap_display_to_string_without_config_to_string(&self) -> String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric> HashMapDisplayToStringWithoutConfigToString<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric:
        crate::common::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>,
{
    fn hashmap_display_to_string_without_config_to_string(&self) -> String {
        crate::common::error_logs_logic::helpers::error_occurence_hashmap_formatter(
            self.iter().fold(String::from(""), |mut acc, (key, value)| {
                acc.push_str(
                    &crate::common::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                        key,
                        value.to_string_without_config(),
                    ),
                );
                acc
            })
        )
    }
}

pub trait HashMapDisplayToStringWithoutConfigToStringWithSerializeDeserialize<'a> {
    fn hashmap_display_to_string_without_config_to_string_with_serialize_deserialize(&self) -> String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric> HashMapDisplayToStringWithoutConfigToStringWithSerializeDeserialize<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric:
        crate::common::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithSerializeDeserialize<
            'a,
        >,
{
    fn hashmap_display_to_string_without_config_to_string_with_serialize_deserialize(&self) -> String {
        crate::common::error_logs_logic::helpers::error_occurence_hashmap_formatter(
            self.iter().fold(String::from(""), |mut acc, (key, value)| {
                acc.push_str(
                    &crate::common::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                        key,
                        value.to_string_without_config_with_serialize_deserialize(),
                    ),
                );
                acc
            })
        )
    }
}
