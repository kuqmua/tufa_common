pub trait ToStringHandleWithoutConfig {
    fn to_string_handle_without_config(&self) -> String;
}

impl<VecElementGeneric> ToStringHandleWithoutConfig for Vec<VecElementGeneric>
where
    VecElementGeneric: std::fmt::Display,
{
    fn to_string_handle_without_config(&self) -> String {
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

impl<HashMapKeyGeneric, HashMapValueGeneric> ToStringHandleWithoutConfig
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: std::fmt::Display,
{
    fn to_string_handle_without_config(&self) -> String {
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

pub trait ToStringHandleCodeOccurence<ConfigGeneric> {
    fn to_string_handle_code_occurence(&self, config: &ConfigGeneric) -> String;
}
