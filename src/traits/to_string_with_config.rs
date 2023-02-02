use crate::traits::code_path::CodePath;
use crate::traits::get_duration::GetDuration;
use crate::traits::get_hostname::GetHostname;
use crate::traits::get_process_id::GetProcessId;

pub trait OriginToStringWithConfig<ConfigGeneric> {
    fn origin_to_string_with_config(&self, config: &ConfigGeneric) -> String;
}

impl<SelfGeneric, ConfigGeneric> OriginToStringWithConfig<ConfigGeneric> for SelfGeneric
where
    SelfGeneric: crate::traits::to_string_without_config::SourceToStringWithoutConfig
        + crate::traits::get_code_occurence::GetCodeOccurence,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn origin_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        format!(
            "{}\n{}",
            self.source_to_string_without_config(),
            self.get_code_occurence().to_string_with_config(config),
        )
    }
}
pub trait ToStringWithConfig<ConfigGeneric> {
    fn to_string_with_config(&self, config: &ConfigGeneric) -> String;
}

impl<SelfGeneric, ConfigGeneric> ToStringWithConfig<ConfigGeneric> for SelfGeneric
where
    SelfGeneric: crate::traits::to_string_with_config::SourceToStringWithConfig<ConfigGeneric>
        + crate::traits::get_code_occurence::GetCodeOccurence,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
        format!(
            "{}\n{}",
            self.source_to_string_with_config(config),
            self.get_code_occurence().to_string_with_config(config),
        )
    }
}

pub trait SourceToStringWithConfig<ConfigGeneric> {
    fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String;
}

pub trait FewToStringWithConfig<ConfigGeneric> {
    fn few_to_string_with_config(&self, config: &ConfigGeneric) -> String;
}

impl<VecElementGeneric, ConfigGeneric> FewToStringWithConfig<ConfigGeneric>
    for Vec<VecElementGeneric>
where
    VecElementGeneric: crate::traits::to_string_with_config::ToStringWithConfig<ConfigGeneric>,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn few_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        format!(
            "[\n{}]",
            self.iter().fold(String::from(""), |mut acc, vec_element| {
                acc.push_str(&vec_element.to_string_with_config(config).lines().fold(
                    String::from(""),
                    |mut acc, vec_element| {
                        acc.push_str(&format!(" {}\n", vec_element));
                        acc
                    },
                ));
                acc
            })
        )
    }
}

impl<HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric> FewToStringWithConfig<ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: crate::traits::to_string_with_config::ToStringWithConfig<ConfigGeneric>,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn few_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        let mut stringified = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(&format!(
                "{} [\n{}]\n",
                key,
                value.to_string_with_config(config).lines().fold(
                    String::from(""),
                    |mut acc, line| {
                        acc.push_str(&format!(" {}\n", line));
                        acc
                    }
                )
            ));
            acc
        });
        stringified.pop();
        stringified
    }
}
