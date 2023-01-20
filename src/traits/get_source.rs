use crate::traits::separator_symbol::SeparatorSymbol;

pub trait GetSource {
    fn get_source(&self) -> String;
}

pub trait GetSourceAsString<ConfigGeneric> {
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String;
}

impl<ConfigGeneric, HashMapKeyGeneric, HashMapValueGeneric> GetSourceAsString<ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone,
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: crate::traits::get_source::GetSourceAsString<ConfigGeneric>
        + crate::traits::get_code_occurence::GetCodeOccurenceAsString<ConfigGeneric>,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        let symbol = config.symbol();
        let mut source_as_string = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            let source_as_string = value.get_source_as_string(config);
            let get_code_occurence_as_string = value.get_code_occurence_as_string(config);
            acc.push_str(&format!(
                "[key: {}]{} {}{} {}{}",
                key, symbol, source_as_string, symbol, get_code_occurence_as_string, symbol
            ));
            acc
        });
        config.pop_last(&mut source_as_string);
        source_as_string
    }
}

impl<ConfigGeneric, VecElementGeneric> GetSourceAsString<ConfigGeneric> for Vec<VecElementGeneric>
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone,
    VecElementGeneric: crate::traits::get_source::GetSourceAsString<ConfigGeneric>
        + crate::traits::get_code_occurence::GetCodeOccurenceAsString<ConfigGeneric>,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        let symbol = config.symbol();
        let mut source_as_string = self.iter().fold(String::from(""), |mut acc, vec_element| {
            acc.push_str(&format!(
                "{}{} {}{}",
                vec_element.get_source_as_string(config),
                symbol,
                vec_element.get_code_occurence_as_string(config),
                symbol
            ));
            acc
        });
        config.pop_last(&mut source_as_string);
        source_as_string
    }
}
