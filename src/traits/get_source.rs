pub trait GetSource {
    fn get_source(&self) -> String;
}

pub trait GetErrorWrapperSourceAsSting<ConfigGeneric> {
    fn get_error_wrapper_source_as_string(&self, config: &ConfigGeneric) -> String;
}

// pub trait GetOriginSourceAsString {
//     fn get_origin_source_as_string(&self) -> String;
// }

// impl<HashMapKeyGeneric, HashMapValueGeneric> GetOriginSourceAsString
//     for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
// where
//     HashMapKeyGeneric: std::fmt::Display,
//     HashMapValueGeneric: std::fmt::Display,
// {
//     fn get_origin_source_as_string(&self) -> String {
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

// impl<VecElementGeneric> GetOriginSourceAsString for Vec<VecElementGeneric>
// where
//     VecElementGeneric: std::fmt::Display,
// {
//     fn get_origin_source_as_string(&self) -> String {
//         format!(
//             "[\n{}]",
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
