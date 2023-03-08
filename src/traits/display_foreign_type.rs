pub trait DisplayForeignType {
    fn display_foreign_type(&self) -> String; //todo - maybe use &str here?
}

impl<VecElementGeneric> DisplayForeignType for Vec<VecElementGeneric>
where
    VecElementGeneric: DisplayForeignType,
{
    fn display_foreign_type(&self) -> String {
        crate::traits::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            String::from(""),
            |mut acc, vec_element| {
                acc.push_str(
                    &crate::traits::error_logs_logic::helpers::lines_space_backslash_addition(
                        vec_element.display_foreign_type(),
                    ),
                );
                acc
            },
        ))
    }
}

impl<HashMapKeyGeneric, HashMapValueGeneric> DisplayForeignType
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: DisplayForeignType,
{
    fn display_foreign_type(&self) -> String {
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
