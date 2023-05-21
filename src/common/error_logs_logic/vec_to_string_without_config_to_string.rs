pub trait VecToStringWithoutConfigToString<'a> {
    fn vec_to_string_without_config_to_string(&self) -> String;
}

impl<'a, VecElementGeneric> VecToStringWithoutConfigToString<'a> for Vec<VecElementGeneric>
where
    VecElementGeneric:
        crate::common::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>,
{
    fn vec_to_string_without_config_to_string(&self) -> String {
        crate::common::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            String::from(""),
            |mut acc, vec_element| {
                acc.push_str(
                    &crate::common::error_logs_logic::helpers::lines_space_backslash_addition(
                        vec_element.to_string_without_config(),
                    ),
                );
                acc
            },
        ))
    }
}

pub trait VecToStringWithoutConfigToStringWithSerializeDeserialize<'a> {
    fn vec_to_string_without_config_to_string_with_serialize_deserialize(&self) -> String;
}

impl<'a, VecElementGeneric> VecToStringWithoutConfigToStringWithSerializeDeserialize<'a> for Vec<VecElementGeneric>
where
    VecElementGeneric:
        crate::common::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithSerializeDeserialize<
            'a,
        >,
{
    fn vec_to_string_without_config_to_string_with_serialize_deserialize(&self) -> String {
        crate::common::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            String::from(""),
            |mut acc, vec_element| {
                acc.push_str(
                    &crate::common::error_logs_logic::helpers::lines_space_backslash_addition(
                        vec_element.to_string_without_config_with_serialize_deserialize(),
                    ),
                );
                acc
            },
        ))
    }
}
