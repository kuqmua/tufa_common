pub trait VecHashMapDisplayForeignTypeDisplayForeignTypeToString {
    fn vec_hashmap_display_foreign_type_display_foreign_type_to_string(&self) -> String;
}

impl<VecElementGeneric> VecHashMapDisplayForeignTypeDisplayForeignTypeToString
    for Vec<VecElementGeneric>
where
    VecElementGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn vec_hashmap_display_foreign_type_display_foreign_type_to_string(&self) -> String {
        crate::traits::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            String::from(""),
            |mut acc, vec_element| {
                acc.push_str(&vec_element.display_foreign_type());
                acc
            },
        ))
    }
}

impl<HashMapKeyGeneric, HashMapValueGeneric> VecHashMapDisplayForeignTypeDisplayForeignTypeToString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: crate::traits::display_foreign_type::DisplayForeignType,
    HashMapValueGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn vec_hashmap_display_foreign_type_display_foreign_type_to_string(&self) -> String {
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
