use crate::traits::code_path::CodePath;
use crate::traits::get_duration::GetDuration;
use crate::traits::get_hostname::GetHostname;
use crate::traits::get_process_id::GetProcessId;

pub trait ErrorToStringWithConfig<ConfigGeneric> {
    fn error_to_string_with_config(&self, config: &ConfigGeneric) -> String;
}

impl<SelfGeneric, ConfigGeneric> ErrorToStringWithConfig<ConfigGeneric> for SelfGeneric
where
    SelfGeneric: crate::traits::get_source::GetOriginSourceAsString
        + crate::traits::get_code_occurence::GetCodeOccurence,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn error_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        let code_occurence = self.get_code_occurence();
        format!(
            "{}\n{} {} on {} {} pid: {}",
            self.get_origin_source_as_string(),
            code_occurence.get_code_path(config.get_source_place_type()),
            chrono::DateTime::<chrono::Utc>::from(
                std::time::UNIX_EPOCH + code_occurence.get_duration(),
            )
            .with_timezone(&chrono::FixedOffset::east_opt(*config.get_timezone()).unwrap())
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
            config.get_server_address(),
            code_occurence.get_hostname(),
            code_occurence.get_process_id(),
        )
    }
}

impl<VecElementGeneric, ConfigGeneric> ErrorToStringWithConfig<ConfigGeneric>
    for Vec<VecElementGeneric>
where
    VecElementGeneric: ErrorToStringWithConfig<ConfigGeneric>,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn error_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        format!(
            "[\n{}]",
            self.iter().fold(String::from(""), |mut acc, vec_element| {
                acc.push_str(
                    &vec_element
                        .error_to_string_with_config(config)
                        .lines()
                        .fold(String::from(""), |mut acc, vec_element| {
                            acc.push_str(&format!(" {}\n", vec_element));
                            acc
                        }),
                );
                acc
            })
        )
    }
}

impl<HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric> ErrorToStringWithConfig<ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: ErrorToStringWithConfig<ConfigGeneric>,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn error_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(&format!(
                "{} [\n{}]\n",
                key,
                value.error_to_string_with_config(config).lines().fold(
                    String::from(""),
                    |mut acc, line| {
                        acc.push_str(&format!(" {}\n", line));
                        acc
                    }
                )
            ));
            acc
        })
    }
}
