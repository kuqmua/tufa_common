pub trait GetSource {
    fn get_source(&self) -> String;
}

pub trait GetSourceAsString {
    fn get_source_as_string(&self) -> String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric> GetSourceAsString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: crate::traits::get_source::GetSourceAsString
        + crate::traits::get_code_occurence::GetCodeOccurenceAsString,
{
    fn get_source_as_string(&self) -> String {
        let mut source_as_string = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            acc.push_str(&format!(
                "[key: \n]{} \n{} {}\n",
                key,
                value.get_source_as_string(),
                value.get_code_occurence_as_string()
            ));
            acc
        });
        source_as_string.pop();
        source_as_string
    }
}

impl<VecElementGeneric> GetSourceAsString for Vec<VecElementGeneric>
where
    VecElementGeneric: crate::traits::get_source::GetSourceAsString
        + crate::traits::get_code_occurence::GetCodeOccurenceAsString,
{
    fn get_source_as_string(&self) -> String {
        let mut source_as_string = self.iter().fold(String::from(""), |mut acc, vec_element| {
            acc.push_str(&format!(
                "{}\n {}\n",
                vec_element.get_source_as_string(),
                vec_element.get_code_occurence_as_string(),
            ));
            acc
        });
        source_as_string.pop();
        source_as_string
    }
}
