pub trait HashmapImplDisplayToStringWithoutConfigToString<'a> {
    fn hashmap_impl_display_to_string_without_config_to_string(&self) -> String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric> HashmapImplDisplayToStringWithoutConfigToString<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric:
        crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>,
{
    fn hashmap_impl_display_to_string_without_config_to_string(&self) -> String {
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

pub trait HashmapImplDisplayToStringWithoutConfigToStringWithDeserialize<'a> {
    fn hashmap_impl_display_to_string_without_config_to_string_with_deserialize(&self) -> String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric> HashmapImplDisplayToStringWithoutConfigToStringWithDeserialize<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric:
        crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
            'a,
        >,
{
    fn hashmap_impl_display_to_string_without_config_to_string_with_deserialize(&self) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(
                &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                    key,
                    value.to_string_without_config_with_deserialize(),
                ),
            );
            acc
        });
        stringified.pop();
        stringified
    }
}
