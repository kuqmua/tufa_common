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

pub trait FewToStringWithoutConfig {
    fn few_to_string_without_config(&self) -> String;
}
//
// impl<SelfGeneric> FewToStringWithoutConfig for SelfGeneric
// where
//     SelfGeneric: ToStringWithoutConfig,
// {
//     fn few_to_string_without_config(&self) -> String {
//         self.to_string_without_config()
//     }
// }

impl<VecElementGeneric> FewToStringWithoutConfig for Vec<VecElementGeneric>
where
    VecElementGeneric: ToStringWithoutConfig,
{
    fn few_to_string_without_config(&self) -> String {
        format!(
            "[\n{}]\n",
            self.iter().fold(String::from(""), |mut acc, vec_element| {
                acc.push_str(&vec_element.to_string_without_config().lines().fold(
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

impl<HashMapKeyGeneric, HashMapValueGeneric> FewToStringWithoutConfig
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: ToStringWithoutConfig,
{
    fn few_to_string_without_config(&self) -> String {
        self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(&format!(
                "{} [\n{}]\n",
                key,
                value
                    .to_string_without_config()
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
