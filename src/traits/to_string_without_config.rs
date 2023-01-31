pub trait ToStringWithoutConfig {
    fn to_string_without_config(&self) -> String;
}

impl<VecElementGeneric> ToStringWithoutConfig for Vec<VecElementGeneric>
where
    VecElementGeneric: std::fmt::Display,
{
    fn to_string_without_config(&self) -> String {
        format!(
            "[\n{}]",
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

impl<HashMapKeyGeneric, HashMapValueGeneric> ToStringWithoutConfig
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: std::fmt::Display,
{
    fn to_string_without_config(&self) -> String {
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
    SelfGeneric: crate::traits::get_source::GetOriginSourceAsString
        + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn origin_to_string_without_config(&self) -> String {
        format!(
            "{}{}",
            self.get_origin_source_as_string(),
            self.get_code_occurence()
        )
    }
}
