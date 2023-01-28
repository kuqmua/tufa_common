use crate::traits::error_display::ToStringHandleCodeOccurence;

pub trait GetSource {
    fn get_source(&self) -> String;
}

pub trait GetSourceAsString<ConfigGeneric> {
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric> GetSourceAsString<ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: crate::traits::get_source::GetSourceAsString<ConfigGeneric>
        + crate::traits::get_code_occurence::GetCodeOccurenceOldWay,
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        let mut source_as_string = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(&format!(
                "[key: \n]{} \n{} {}\n",
                key,
                value.get_source_as_string(config),
                value
                    .get_code_occurence_old_way()
                    .to_string_handle_code_occurence(config)
            ));
            acc
        });
        source_as_string.pop();
        source_as_string
    }
}

impl<VecElementGeneric, ConfigGeneric> GetSourceAsString<ConfigGeneric> for Vec<VecElementGeneric>
where
    VecElementGeneric: crate::traits::get_source::GetSourceAsString<ConfigGeneric>
        + crate::traits::get_code_occurence::GetCodeOccurenceOldWay,
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        let mut source_as_string = self.iter().fold(String::from(""), |mut acc, vec_element| {
            acc.push_str(&format!(
                "{}\n {}\n",
                vec_element.get_source_as_string(config),
                vec_element
                    .get_code_occurence_old_way()
                    .to_string_handle_code_occurence(config),
            ));
            acc
        });
        source_as_string.pop();
        source_as_string
    }
}
