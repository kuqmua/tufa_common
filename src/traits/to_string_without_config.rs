//later rename it coz there is no situation for origin source as string with config
pub trait OriginSourceToStringWithoutConfig {
    fn origin_source_to_string_without_config(&self) -> String;
}

//conflict impl
// impl<SelfGeneric> OriginSourceToStringWithoutConfig for SelfGeneric
// where
//     SelfGeneric: std::fmt::Display,
// {
//     fn origin_source_to_string_without_config(&self) -> String {
//         format!("{}\n", self)
//     }
// }

impl<VecElementGeneric> OriginSourceToStringWithoutConfig for Vec<VecElementGeneric>
where
    VecElementGeneric: std::fmt::Display,
{
    fn origin_source_to_string_without_config(&self) -> String {
        format!(
            "[\n{}]\n",
            self.iter().fold(String::from(""), |mut acc, vec_element| {
                acc.push_str(&vec_element.to_string().lines().fold(
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

impl<HashMapKeyGeneric, HashMapValueGeneric> OriginSourceToStringWithoutConfig
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: std::fmt::Display,
{
    fn origin_source_to_string_without_config(&self) -> String {
        // format!("{}\n")
        self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(&format!(
                "{} [\n{}]\n",
                key,
                value
                    .to_string()
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

pub trait OriginToStringWithoutConfig {
    fn origin_to_string_without_config(&self) -> String;
}

impl<SelfGeneric> OriginToStringWithoutConfig for SelfGeneric
where
    SelfGeneric: crate::traits::to_string_without_config::OriginSourceToStringWithoutConfig
        + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn origin_to_string_without_config(&self) -> String {
        format!(
            "{}{}",
            self.origin_source_to_string_without_config(),
            self.get_code_occurence()
        )
    }
}

//variants with origin and wrapper must be implemented
pub trait WrapperSourceToStringWithoutConfigFromOrigin {
    fn wrapper_source_to_string_without_config_from_origin(&self) -> String;
}
//
impl<SelfGeneric> WrapperSourceToStringWithoutConfigFromOrigin for SelfGeneric
where
    SelfGeneric: crate::traits::to_string_without_config::OriginToStringWithoutConfig,
{
    fn wrapper_source_to_string_without_config_from_origin(&self) -> String {
        format!("{}", self.origin_to_string_without_config())
    }
}

impl<VecElementGeneric> WrapperSourceToStringWithoutConfigFromOrigin for Vec<VecElementGeneric>
where
    VecElementGeneric: crate::traits::to_string_without_config::OriginToStringWithoutConfig,
{
    fn wrapper_source_to_string_without_config_from_origin(&self) -> String {
        format!(
            "[\n{}]\n",
            self.iter().fold(String::from(""), |mut acc, vec_element| {
                acc.push_str(&vec_element.origin_to_string_without_config().lines().fold(
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

impl<HashMapKeyGeneric, HashMapValueGeneric> WrapperSourceToStringWithoutConfigFromOrigin
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: crate::traits::to_string_without_config::OriginToStringWithoutConfig,
{
    fn wrapper_source_to_string_without_config_from_origin(&self) -> String {
        // format!("{}\n")
        self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(&format!(
                "{} [\n{}]\n",
                key,
                value.origin_to_string_without_config().lines().fold(
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

pub trait WrapperSourceToStringWithoutConfigFromWrapper {
    fn wrapper_source_to_string_without_config_from_wrapper(&self) -> String;
}

//conflicting impl
// impl<SelfGeneric> WrapperSourceToStringWithoutConfigFromWrapper for SelfGeneric
// where
//     SelfGeneric:
//         crate::traits::to_string_without_config::WrapperSourceToStringWithoutConfigFromOrigin,
// {
//     fn wrapper_source_to_string_without_config_from_wrapper(&self) -> String {
//         format!(
//             "{}",
//             self.wrapper_source_to_string_without_config_from_origin()
//         )
//     }
// }

impl<VecElementGeneric> WrapperSourceToStringWithoutConfigFromWrapper for Vec<VecElementGeneric>
where
    VecElementGeneric:
        crate::traits::to_string_without_config::WrapperSourceToStringWithoutConfigFromOrigin,
{
    fn wrapper_source_to_string_without_config_from_wrapper(&self) -> String {
        format!(
            "[\n{}]\n",
            self.iter().fold(String::from(""), |mut acc, vec_element| {
                acc.push_str(
                    &vec_element
                        .wrapper_source_to_string_without_config_from_origin()
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

impl<HashMapKeyGeneric, HashMapValueGeneric> WrapperSourceToStringWithoutConfigFromWrapper
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric:
        crate::traits::to_string_without_config::WrapperSourceToStringWithoutConfigFromOrigin,
{
    fn wrapper_source_to_string_without_config_from_wrapper(&self) -> String {
        // format!("{}\n")
        self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(&format!(
                "{} [\n{}]\n",
                key,
                value
                    .wrapper_source_to_string_without_config_from_origin()
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
// diffrenet traits WrapperSourceToStringWithoutConfigFromWrapperOrigin
// diffrenet traits WrapperSourceToStringWithoutConfigFromWrapperWrapper
//todo impl WrapperSourceToStringWithoutConfigFromWrapper for crate::traits::to_string_without_config::WrapperSourceToStringWithoutConfigFromWrapper

// pub trait WrapperToStringWithoutConfig {
//     fn wrapper_to_string_without_config(&self) -> String;
// }

// impl<SelfGeneric> WrapperToStringWithoutConfig for SelfGeneric
// where
//     SelfGeneric: crate::traits::to_string_without_config::WrapperSourceToStringWithoutConfig
//         + crate::traits::get_code_occurence::GetCodeOccurence,
// {
//     fn wrapper_to_string_without_config(&self) -> String {
//         format!(
//             "{}{}",
//             self.wrapper_source_to_string_without_config(),
//             self.get_code_occurence()
//         )
//     }
// }
//
