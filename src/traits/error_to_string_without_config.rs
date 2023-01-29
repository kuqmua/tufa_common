pub trait ErrorToStringWithoutConfig {
    fn error_to_string_without_config(&self) -> String;
}

impl<VecElementGeneric> ErrorToStringWithoutConfig for Vec<VecElementGeneric>
where
    VecElementGeneric: std::fmt::Display,
{
    fn error_to_string_without_config(&self) -> String {
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

impl<HashMapKeyGeneric, HashMapValueGeneric> ErrorToStringWithoutConfig
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: std::fmt::Display,
{
    fn error_to_string_without_config(&self) -> String {
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
