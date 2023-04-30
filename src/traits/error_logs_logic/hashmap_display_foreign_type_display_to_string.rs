pub trait HashMapDisplayForeignTypeDisplayToString {
    fn hashmap_display_foreign_type_display_to_string(&self) -> String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric> HashMapDisplayForeignTypeDisplayToString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: crate::traits::display_foreign_type::DisplayForeignType,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_display_foreign_type_display_to_string(&self) -> String {
        crate::traits::error_logs_logic::helpers::error_occurence_hashmap_formatter(
            self.iter().fold(String::from(""), |mut acc, (key, value)| {
                acc.push_str(
                    &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                        key.display_foreign_type(),
                        value.to_string(),
                    ),
                );
                acc
            })
        )
    }
}
