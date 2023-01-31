use crate::traits::code_path::CodePath;
use crate::traits::get_duration::GetDuration;
use crate::traits::get_hostname::GetHostname;
use crate::traits::get_process_id::GetProcessId;

// this doesnt make sense coz config not needed in this situation
// pub trait OriginSourceToStringWithConfig<ConfigGeneric> {
//     fn origin_source_to_string_with_config(&self, config: &ConfigGeneric) -> String;
// }
pub trait OriginToStringWithConfig<ConfigGeneric> {
    fn origin_to_string_with_config(&self, config: &ConfigGeneric) -> String;
}

impl<SelfGeneric, ConfigGeneric> OriginToStringWithConfig<ConfigGeneric> for SelfGeneric
where
    SelfGeneric: crate::traits::to_string_without_config::OriginSourceToStringWithoutConfig
        + crate::traits::get_code_occurence::GetCodeOccurence,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn origin_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        format!(
            "{}{}",
            self.origin_source_to_string_without_config(),
            self.get_code_occurence().to_string_with_config(config),
        )
    }
}
//
pub trait ToStringWithConfig<ConfigGeneric> {
    fn to_string_with_config(&self, config: &ConfigGeneric) -> String;
}

// impl<SelfGeneric, ConfigGeneric>
//     crate::traits::to_string_with_config::ToStringWithConfig<ConfigGeneric> for SelfGeneric
// where
//     SelfGeneric: crate::traits::get_source::GetErrorWrapperSourceAsSting<ConfigGeneric>
//         + crate::traits::get_code_occurence::GetCodeOccurence,
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         format!(
//             "{}{}",
//             self.get_error_wrapper_source_as_string(config),
//             self.get_code_occurence().to_string_with_config(config),
//         )
//     }
// }
///////////////////////////
// impl<SelfGeneric, ConfigGeneric> ToStringWithConfig<ConfigGeneric> for SelfGeneric
// where
//     SelfGeneric: crate::traits::get_source::GetOriginSourceAsString
//         // crate::traits::to_string_without_config::ToStringWithoutConfig
//         + crate::traits::get_code_occurence::GetCodeOccurence,
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         let code_occurence = self.get_code_occurence();
//         format!(
//             "{}{} {}",
//             // self.to_string_without_config(),
//             self.get_origin_source_as_string(),
//             code_occurence.prepare_for_log(
//                 code_occurence.get_code_path(config.get_source_place_type()),
//                 chrono::DateTime::<chrono::Utc>::from(
//                     std::time::UNIX_EPOCH + code_occurence.get_duration(),
//                 )
//                 .with_timezone(&chrono::FixedOffset::east_opt(*config.get_timezone()).unwrap())
//                 .format("%Y-%m-%d %H:%M:%S")
//                 .to_string(),
//                 code_occurence.get_hostname(),
//                 code_occurence.get_process_id(),
//             ),
//             config.get_server_address(),
//         )
//     }
// }

// impl<VecElementGeneric, ConfigGeneric> ToStringWithConfig<ConfigGeneric> for Vec<VecElementGeneric>
// where
//     VecElementGeneric: ToStringWithConfig<ConfigGeneric>,
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         format!(
//             "[\n{}]",
//             self.iter().fold(String::from(""), |mut acc, vec_element| {
//                 acc.push_str(&vec_element.to_string_with_config(config).lines().fold(
//                     String::from(""),
//                     |mut acc, vec_element| {
//                         acc.push_str(&format!(" {}\n", vec_element));
//                         acc
//                     },
//                 ));
//                 acc
//             })
//         )
//     }
// }

// impl<HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric> ToStringWithConfig<ConfigGeneric>
//     for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
// where
//     HashMapKeyGeneric: std::fmt::Display,
//     HashMapValueGeneric: ToStringWithConfig<ConfigGeneric>,
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         self.iter().fold(String::from(""), |mut acc, (key, value)| {
//             acc.push_str(&format!(
//                 "{} [\n{}]\n",
//                 key,
//                 value.to_string_with_config(config).lines().fold(
//                     String::from(""),
//                     |mut acc, line| {
//                         acc.push_str(&format!(" {}\n", line));
//                         acc
//                     }
//                 )
//             ));
//             acc
//         })
//     }
// }
////////////////////////
//
// impl<VecElementGeneric, ConfigGeneric> ToStringWithConfig<ConfigGeneric> for Vec<VecElementGeneric>
// where
//     VecElementGeneric: ToStringWithConfig<ConfigGeneric>,
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         format!(
//             "[\n{}]",
//             self.iter().fold(String::from(""), |mut acc, vec_element| {
//                 acc.push_str(&vec_element.to_string_with_config(config).lines().fold(
//                     String::from(""),
//                     |mut acc, vec_element| {
//                         acc.push_str(&format!(" {}\n", vec_element));
//                         acc
//                     },
//                 ));
//                 acc
//             })
//         )
//     }
// }

// impl<HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric> ToStringWithConfig<ConfigGeneric>
//     for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
// where
//     HashMapKeyGeneric: std::fmt::Display,
//     HashMapValueGeneric: ToStringWithConfig<ConfigGeneric>,
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
//         self.iter().fold(String::from(""), |mut acc, (key, value)| {
//             acc.push_str(&format!(
//                 "{} [\n{}]\n",
//                 key,
//                 value.to_string_with_config(config).lines().fold(
//                     String::from(""),
//                     |mut acc, line| {
//                         acc.push_str(&format!(" {}\n", line));
//                         acc
//                     }
//                 )
//             ));
//             acc
//         })
//     }
// }
