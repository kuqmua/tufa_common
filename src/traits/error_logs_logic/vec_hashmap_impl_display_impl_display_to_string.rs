pub trait VecHashmapImplDisplayImplDisplayToString {
    fn vec_hashmap_impl_display_impl_display_to_string(&self) -> String;
}

impl<VecElementGeneric> VecHashmapImplDisplayImplDisplayToString for Vec<VecElementGeneric>
where
    VecElementGeneric: std::fmt::Display,
{
    fn vec_hashmap_impl_display_impl_display_to_string(&self) -> String {
        crate::traits::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            String::from(""),
            |mut acc, vec_element| {
                acc.push_str(&vec_element.to_string());
                acc
            },
        ))
    }
}

impl<HashMapKeyGeneric, HashMapValueGeneric> VecHashmapImplDisplayImplDisplayToString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: std::fmt::Display,
{
    fn vec_hashmap_impl_display_impl_display_to_string(&self) -> String {
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
