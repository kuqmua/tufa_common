pub trait GetCodeOccurence {
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence;
}
//
pub trait GetCodeOccurenceAsString {
    fn get_code_occurence_as_string(
        &self,
        config: &crate::config_mods::config_struct::ConfigStruct,
    ) -> String;
}

// pub fn get_code_occurence_as_string(
//         &self,
//         config: &crate::config_mods::config_struct::ConfigStruct,
//     ) -> String {
//         self.code_occurence
//             .get_code_path(config.get_source_place_type())
//     }
//

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
