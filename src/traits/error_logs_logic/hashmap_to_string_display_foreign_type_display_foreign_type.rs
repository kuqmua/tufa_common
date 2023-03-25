pub trait HashMapToStringDisplayForeignTypeDisplayForeignType {
    fn hashmap_to_string_display_foreign_type_display_foreign_type(&self) -> String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric> HashMapToStringDisplayForeignTypeDisplayForeignType
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: crate::traits::display_foreign_type::DisplayForeignType,
    HashMapValueGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn hashmap_to_string_display_foreign_type_display_foreign_type(&self) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(
                &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                    key.display_foreign_type(),
                    value.display_foreign_type(),
                ),
            );
            acc
        });
        stringified.pop();
        stringified
    }
}
