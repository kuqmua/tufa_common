pub trait ToStringWithoutConfig {
    fn to_string_without_config(&self) -> String;
}

impl<SelfGeneric> ToStringWithoutConfig for SelfGeneric
where
    SelfGeneric:
        crate::traits::error_logs_logic::to_string_without_config::SourceToStringWithoutConfig
            + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn to_string_without_config(&self) -> String {
        crate::traits::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_without_config(),
            self.get_code_occurence().to_string(),
        )
    }
}

pub trait SourceToStringWithoutConfig {
    fn source_to_string_without_config(&self) -> String;
}

pub trait FewToStringWithoutConfig {
    fn few_to_string_without_config(&self) -> String;
}

impl<VecElementGeneric> FewToStringWithoutConfig for Vec<VecElementGeneric>
where
    VecElementGeneric: ToStringWithoutConfig,
{
    fn few_to_string_without_config(&self) -> String {
        crate::traits::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            String::from(""),
            |mut acc, vec_element| {
                acc.push_str(
                    &crate::traits::error_logs_logic::helpers::lines_space_backslash_addition(
                        vec_element.to_string_without_config(),
                    ),
                );
                acc
            },
        ))
    }
}

impl<HashMapKeyGeneric, HashMapValueGeneric> FewToStringWithoutConfig
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: ToStringWithoutConfig,
{
    fn few_to_string_without_config(&self) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(
                &crate::traits::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                    key,
                    value.to_string_without_config(),
                ),
            );
            acc
        });
        stringified.pop();
        stringified
    }
}
