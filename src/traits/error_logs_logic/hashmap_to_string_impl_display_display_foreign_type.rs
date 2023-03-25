pub trait HashMapToStringImplDisplayDisplayForeignType {
    fn hashmap_to_string_impl_display_display_foreign_type(&self) -> String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric> HashMapToStringImplDisplayDisplayForeignType
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn hashmap_to_string_impl_display_display_foreign_type(&self) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(
                &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                    key,
                    value.display_foreign_type(),
                ),
            );
            acc
        });
        stringified.pop();
        stringified
    }
}
