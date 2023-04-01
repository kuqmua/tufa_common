pub trait HashmapDisplayDisplayToString {
    fn hashmap_display_display_to_string(&self) -> String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric> HashmapDisplayDisplayToString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_display_display_to_string(&self) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(
                &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                    key,
                    value.to_string(),
                ),
            );
            acc
        });
        stringified.pop();
        stringified
    }
}
