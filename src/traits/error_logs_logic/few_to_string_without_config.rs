pub trait FewToStringWithoutConfigLifetime<'a> {
    fn few_to_string_without_config_lifetime(&self) -> String;
}

impl<'a, VecElementGeneric> FewToStringWithoutConfigLifetime<'a> for Vec<VecElementGeneric>
where
    VecElementGeneric:
        crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime<
            'a,
        >,
{
    fn few_to_string_without_config_lifetime(&self) -> String {
        crate::traits::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            String::from(""),
            |mut acc, vec_element| {
                acc.push_str(
                    &crate::traits::error_logs_logic::helpers::lines_space_backslash_addition(
                        vec_element.to_string_without_config_lifetime(),
                    ),
                );
                acc
            },
        ))
    }
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric> FewToStringWithoutConfigLifetime<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric:
        crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime<
            'a,
        >,
{
    fn few_to_string_without_config_lifetime(&self) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(
                &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                    key,
                    value.to_string_without_config_lifetime(),
                ),
            );
            acc
        });
        stringified.pop();
        stringified
    }
}
