use crate::traits::error_display::ToStringHandleCodeOccurence;

pub trait GetSource {
    fn get_source(&self) -> String;
}

pub trait GetOriginSourceAsString {
    fn get_origin_source_as_string(&self) -> String;
}

// impl<HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric> GetOriginSourceAsString
//     for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
// where
//     HashMapKeyGeneric: std::fmt::Display,
//     HashMapValueGeneric: crate::traits::get_source::GetOriginSourceAsString
//         + crate::traits::get_code_occurence::GetCodeOccurenceOldWay,
// {
//     fn get_origin_source_as_string(&self) -> String {
//         let mut source_as_string = self.iter().fold(String::from(""), |mut acc, (key, value)| {
//             acc.push_str(&format!(
//                 "[key: \n]{} \n{} {}\n",
//                 key,
//                 value.get_origin_source_as_string(),
//                 value
//                     .get_code_occurence_old_way()
//                     .to_string_handle_code_occurence(config)
//             ));
//             acc
//         });
//         source_as_string.pop();
//         source_as_string
//     }
// }

// impl<VecElementGeneric> GetOriginSourceAsString
//     for Vec<VecElementGeneric>
// where
//     VecElementGeneric: crate::traits::get_source::GetOriginSourceAsString
//         + crate::traits::get_code_occurence::GetCodeOccurenceOldWay,
// {
//     fn get_origin_source_as_string(&self) -> String {
//         let mut source_as_string = self.iter().fold(String::from(""), |mut acc, vec_element| {
//             acc.push_str(&format!(
//                 "{}\n {}\n",
//                 vec_element.get_origin_source_as_string(),
//                 vec_element
//                     .get_code_occurence_old_way()
//                     .to_string_handle_code_occurence(config),
//             ));
//             acc
//         });
//         source_as_string.pop();
//         source_as_string
//     }
// }
