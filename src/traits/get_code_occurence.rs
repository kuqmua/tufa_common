pub trait GetCodeOccurence {
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence;
}

// impl<KeyGeneric, ValueGeneric> GetCodeOccurence
//     for HashMap<KeyGeneric, ValueGeneric>
// where
//     ValueGeneric: GetCodeOccurence,
// {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
//         let
//         // let mut formatted = self
//         // .source
//         // .iter()
//         // .map(|(key, error)| format!("{} {},", key, #origin_or_wrapper_for_hashmap))
//         // .collect::<Vec<String>>()
//         // .iter()
//         // .fold(String::from(""), |mut acc, elem| {
//         //     acc.push_str(elem);
//         //     acc
//         // });
//         // if !formatted.is_empty() {
//         //     formatted.pop();
//         // }
//         // formatted
//     }
// }
