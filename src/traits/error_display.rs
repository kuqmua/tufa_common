use crate::traits::code_path::CodePath;
use crate::traits::get_hostname::GetHostname;
use crate::traits::get_process_id::GetProcessId;
use crate::traits::get_time::GetTime;

pub trait ErrorDisplayInner<ConfigGeneric> {
    fn error_display_inner(&self, config: &ConfigGeneric) -> String;
}

impl<SelfGeneric, ConfigGeneric> ErrorDisplayInner<ConfigGeneric> for SelfGeneric
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
    SelfGeneric: crate::traits::get_source::GetSourceAsString<ConfigGeneric>
        + crate::traits::get_code_occurence::GetCodeOccurenceOldWay,
{
    fn error_display_inner(&self, config: &ConfigGeneric) -> String {
        let code_occurence = self.get_code_occurence_old_way();
        format!(
            "{}\n{} {} on {} {} pid: {}",
            self.get_source_as_string(config),
            code_occurence.get_code_path(config.get_source_place_type()),
            chrono::DateTime::<chrono::Utc>::from(
                std::time::UNIX_EPOCH + code_occurence.get_time(),
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

pub trait ToStringHandle<ConfigGeneric> {
    fn to_string_handle(&self, config: &ConfigGeneric) -> String;
}

impl<VecElementGeneric, ConfigGeneric> ToStringHandle<ConfigGeneric> for Vec<VecElementGeneric>
where
    VecElementGeneric: ToStringHandle<ConfigGeneric>,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        format!(
            "[\n{}]",
            self.iter().fold(String::from(""), |mut acc, vec_element| {
                acc.push_str(&vec_element.to_string_handle(config).lines().fold(
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

impl<HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric> ToStringHandle<ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: ToStringHandle<ConfigGeneric>,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(&format!(
                "{} [\n{}]\n",
                key,
                value
                    .to_string_handle(config)
                    .lines()
                    .fold(String::from(""), |mut acc, line| {
                        acc.push_str(&format!(" {}\n", line));
                        acc
                    })
            ));
            acc
        })
    }
}

pub trait ToStringHandleCodeOccurence<ConfigGeneric> {
    fn to_string_handle_code_occurence(&self, config: &ConfigGeneric) -> String;
}
