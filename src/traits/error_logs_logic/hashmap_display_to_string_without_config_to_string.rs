pub trait HashMapDisplayToStringWithoutConfigToString<'a> {
    fn hashmap_display_to_string_without_config_to_string(&self) -> String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric> HashMapDisplayToStringWithoutConfigToString<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric:
        crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>,
{
    fn hashmap_display_to_string_without_config_to_string(&self) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(
                &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                    key,
                    value.to_string_without_config(),
                ),
            );
            acc
        });
        stringified.pop();
        stringified
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
        crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithSerializeDeserialize<
            'a,
        >,
{
    fn hashmap_display_to_string_without_config_to_string_with_serialize_deserialize(&self) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(
                &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                    key,
                    value.to_string_without_config_with_serialize_deserialize(),
                ),
            );
            acc
        });
        stringified.pop();
        stringified
    }
}
