pub trait HashMapDisplayDisplayToString {
    fn hashmap_display_display_to_string(&self) -> std::string::String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric> HashMapDisplayDisplayToString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_display_display_to_string(&self) -> std::string::String {
        crate::common::error_logs_logic::helpers::error_occurence_hashmap_formatter(
            self.iter().fold(String::from(""), |mut acc, (key, value)| {
                acc.push_str(
                    &crate::common::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                        key,
                        value.to_string(),
                    ),
                );
                acc
            })
        )
    }
}
