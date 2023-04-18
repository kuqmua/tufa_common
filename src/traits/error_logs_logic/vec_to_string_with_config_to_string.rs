pub trait VecToStringWithConfigToString<'a, ConfigGeneric> {
    fn vec_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> String;
}

impl<'a, VecElementGeneric, ConfigGeneric> VecToStringWithConfigToString<'a, ConfigGeneric>
    for Vec<VecElementGeneric>
where
    VecElementGeneric:
        crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<
            'a,
            ConfigGeneric,
        >,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
{
    fn vec_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> String {
        crate::traits::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            String::from(""),
            |mut acc, vec_element| {
                acc.push_str(
                    &crate::traits::error_logs_logic::helpers::lines_space_backslash_addition(
                        vec_element.to_string_with_config_for_source_to_string_with_config(config),
                    ),
                );
                acc
            },
        ))
    }
}
