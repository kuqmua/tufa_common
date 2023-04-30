pub trait HashMapDisplayDisplayForeignTypeToString {
    fn hashmap_display_display_foreign_type_to_string(&self) -> String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric> HashMapDisplayDisplayForeignTypeToString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_display_foreign_type_to_string(&self) -> String {
       crate::traits::error_logs_logic::helpers::error_occurence_hashmap_formatter(
            self.iter().fold(String::from(""), |mut acc, (key, value)| {
                acc.push_str(
                    &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                        key,
                        value.display_foreign_type(),
                    ),
                );
                acc
            })
        )
        
    }
}
