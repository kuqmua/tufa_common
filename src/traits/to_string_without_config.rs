pub trait ToStringWithoutConfig {
    fn to_string_without_config(&self) -> String;
}

// impl<SelfGeneric> ToStringWithoutConfig for SelfGeneric
// where
//     SelfGeneric: crate::traits::to_string_without_config::SourceToStringWithoutConfig
//         + crate::traits::get_code_occurence::GetCodeOccurence,
// {
//     fn to_string_without_config(&self) -> String {
//         format!(
//             "{}{}",
//             self.source_to_string_without_config(),
//             self.get_code_occurence()
//         )
//     }
// }

pub trait SourceToStringWithoutConfig {
    fn source_to_string_without_config(&self) -> String;
}

// conflict impl
// impl<SelfGeneric> ToStringWithoutConfig for SelfGeneric
// where
//     SelfGeneric: std::fmt::Display,
// {
//     fn to_string_without_config(&self) -> String {
//         format!("{}\n", self)
//     }
// }

// impl<VecElementGeneric> ToStringWithoutConfig for Vec<VecElementGeneric>
// where
//     VecElementGeneric: std::fmt::Display,
// {
//     fn to_string_without_config(&self) -> String {
//         format!(
//             "[\n{}]\n",
//             self.iter().fold(String::from(""), |mut acc, vec_element| {
//                 acc.push_str(&vec_element.to_string().lines().fold(
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

// impl<HashMapKeyGeneric, HashMapValueGeneric> ToStringWithoutConfig
//     for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
// where
//     HashMapKeyGeneric: std::fmt::Display,
//     HashMapValueGeneric: std::fmt::Display,
// {
//     fn to_string_without_config(&self) -> String {
//         // format!("{}\n")
//         self.iter().fold(String::from(""), |mut acc, (key, value)| {
//             acc.push_str(&format!(
//                 "{} [\n{}]\n",
//                 key,
//                 value
//                     .to_string()
//                     .lines()
//                     .fold(String::from(""), |mut acc, line| {
//                         acc.push_str(&format!(" {}\n", line));
//                         acc
//                     })
//             ));
//             acc
//         })
//     }
// }
//later rename it coz there is no situation for origin source as string with config
// pub trait OriginSourceToStringWithoutConfig {
//     fn origin_source_to_string_without_config(&self) -> String;
// }

// // conflict impl
// // impl<SelfGeneric> OriginSourceToStringWithoutConfig for SelfGeneric
// // where
// //     SelfGeneric: std::fmt::Display,
// // {
// //     fn origin_source_to_string_without_config(&self) -> String {
// //         format!("{}\n", self)
// //     }
// // }

// impl<VecElementGeneric> OriginSourceToStringWithoutConfig for Vec<VecElementGeneric>
// where
//     VecElementGeneric: std::fmt::Display,
// {
//     fn origin_source_to_string_without_config(&self) -> String {
//         format!(
//             "[\n{}]\n",
//             self.iter().fold(String::from(""), |mut acc, vec_element| {
//                 acc.push_str(&vec_element.to_string().lines().fold(
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

// impl<HashMapKeyGeneric, HashMapValueGeneric> OriginSourceToStringWithoutConfig
//     for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
// where
//     HashMapKeyGeneric: std::fmt::Display,
//     HashMapValueGeneric: std::fmt::Display,
// {
//     fn origin_source_to_string_without_config(&self) -> String {
//         // format!("{}\n")
//         self.iter().fold(String::from(""), |mut acc, (key, value)| {
//             acc.push_str(&format!(
//                 "{} [\n{}]\n",
//                 key,
//                 value
//                     .to_string()
//                     .lines()
//                     .fold(String::from(""), |mut acc, line| {
//                         acc.push_str(&format!(" {}\n", line));
//                         acc
//                     })
//             ));
//             acc
//         })
//     }
// }

// pub trait OriginToStringWithoutConfig {
//     fn origin_to_string_without_config(&self) -> String;
// }

// impl<SelfGeneric> OriginToStringWithoutConfig for SelfGeneric
// where
//     SelfGeneric: crate::traits::to_string_without_config::OriginSourceToStringWithoutConfig
//         + crate::traits::get_code_occurence::GetCodeOccurence,
// {
//     fn origin_to_string_without_config(&self) -> String {
//         format!(
//             "{}{}",
//             self.origin_source_to_string_without_config(),
//             self.get_code_occurence()
//         )
//     }
// }

// pub trait WrapperSourceToStringWithoutConfigFromOrigin {
//     fn wrapper_source_to_string_without_config_from_origin(&self) -> String;
// }

// impl<SelfGeneric> WrapperSourceToStringWithoutConfigFromOrigin for SelfGeneric
// where
//     SelfGeneric: crate::traits::to_string_without_config::OriginToStringWithoutConfig,
// {
//     fn wrapper_source_to_string_without_config_from_origin(&self) -> String {
//         format!("{}", self.origin_to_string_without_config())
//     }
// }

// impl<VecElementGeneric> WrapperSourceToStringWithoutConfigFromOrigin for Vec<VecElementGeneric>
// where
//     VecElementGeneric: crate::traits::to_string_without_config::OriginToStringWithoutConfig,
// {
//     fn wrapper_source_to_string_without_config_from_origin(&self) -> String {
//         format!(
//             "[\n{}]\n",
//             self.iter().fold(String::from(""), |mut acc, vec_element| {
//                 acc.push_str(&vec_element.origin_to_string_without_config().lines().fold(
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

// impl<HashMapKeyGeneric, HashMapValueGeneric> WrapperSourceToStringWithoutConfigFromOrigin
//     for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
// where
//     HashMapKeyGeneric: std::fmt::Display,
//     HashMapValueGeneric: crate::traits::to_string_without_config::OriginToStringWithoutConfig,
// {
//     fn wrapper_source_to_string_without_config_from_origin(&self) -> String {
//         // format!("{}\n")
//         self.iter().fold(String::from(""), |mut acc, (key, value)| {
//             acc.push_str(&format!(
//                 "{} [\n{}]\n",
//                 key,
//                 value.origin_to_string_without_config().lines().fold(
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

// pub trait WrapperToStringWithoutConfigFromWrapperOrigin {
//     fn wrapper_to_string_without_config_from_wrapper_origin(&self) -> String;
// }

// impl<SelfGeneric> WrapperToStringWithoutConfigFromWrapperOrigin for SelfGeneric
// where
//     SelfGeneric:
//         crate::traits::to_string_without_config::WrapperSourceToStringWithoutConfigFromOrigin
//             + crate::traits::get_code_occurence::GetCodeOccurence,
// {
//     fn wrapper_to_string_without_config_from_wrapper_origin(&self) -> String {
//         format!(
//             "{}{}",
//             self.wrapper_source_to_string_without_config_from_origin(),
//             self.get_code_occurence()
//         )
//     }
// }

// pub trait WrapperSourceToStringWithoutConfigFromWrapperWrapper {
//     fn wrapper_source_to_string_without_config_from_wrapper_wrapper(&self) -> String;
// }

// impl<SelfGeneric> WrapperSourceToStringWithoutConfigFromWrapperWrapper for SelfGeneric
// where
//     SelfGeneric:
//         crate::traits::to_string_without_config::WrapperToStringWithoutConfigFromWrapperOrigin,
// {
//     fn wrapper_source_to_string_without_config_from_wrapper_wrapper(&self) -> String {
//         format!(
//             "{}",
//             self.wrapper_to_string_without_config_from_wrapper_origin()
//         )
//     }
// }

// impl<VecElementGeneric> WrapperSourceToStringWithoutConfigFromWrapperWrapper
//     for Vec<VecElementGeneric>
// where
//     VecElementGeneric:
//         crate::traits::to_string_without_config::WrapperToStringWithoutConfigFromWrapperOrigin,
// {
//     fn wrapper_source_to_string_without_config_from_wrapper_wrapper(&self) -> String {
//         format!(
//             "[\n{}]\n",
//             self.iter().fold(String::from(""), |mut acc, vec_element| {
//                 acc.push_str(
//                     &vec_element
//                         .wrapper_to_string_without_config_from_wrapper_origin()
//                         .lines()
//                         .fold(String::from(""), |mut acc, vec_element| {
//                             acc.push_str(&format!(" {}\n", vec_element));
//                             acc
//                         }),
//                 );
//                 acc
//             })
//         )
//     }
// }

// impl<HashMapKeyGeneric, HashMapValueGeneric> WrapperSourceToStringWithoutConfigFromWrapperWrapper
//     for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
// where
//     HashMapKeyGeneric: std::fmt::Display,
//     HashMapValueGeneric:
//         crate::traits::to_string_without_config::WrapperToStringWithoutConfigFromWrapperOrigin,
// {
//     fn wrapper_source_to_string_without_config_from_wrapper_wrapper(&self) -> String {
//         // format!("{}\n")
//         self.iter().fold(String::from(""), |mut acc, (key, value)| {
//             acc.push_str(&format!(
//                 "{} [\n{}]\n",
//                 key,
//                 value
//                     .wrapper_to_string_without_config_from_wrapper_origin()
//                     .lines()
//                     .fold(String::from(""), |mut acc, line| {
//                         acc.push_str(&format!(" {}\n", line));
//                         acc
//                     })
//             ));
//             acc
//         })
//     }
// }

// pub trait WrapperToStringWithoutConfigFromWrapperWrapper {
//     fn wrapper_to_string_without_config_from_wrapper_wrapper(&self) -> String;
// }

// impl<SelfGeneric> WrapperToStringWithoutConfigFromWrapperWrapper for SelfGeneric
// where
//     SelfGeneric:
//         crate::traits::to_string_without_config::WrapperSourceToStringWithoutConfigFromWrapperWrapper
//             + crate::traits::get_code_occurence::GetCodeOccurence,
// {
//     fn wrapper_to_string_without_config_from_wrapper_wrapper(&self) -> String {
//         format!(
//             "{}{}",
//             self.wrapper_source_to_string_without_config_from_wrapper_wrapper(),
//             self.get_code_occurence()
//         )
//     }
// }
// // // diffrenet traits WrapperSourceToStringWithoutConfigFromWrapperOrigin
// // // diffrenet traits WrapperToStringWithoutConfigFromWrapperOrigin
// // // diffrenet traits WrapperSourceToStringWithoutConfigFromWrapperWrapper
// // // diffrenet traits WrapperToStringWithoutConfigFromWrapperWrapper
