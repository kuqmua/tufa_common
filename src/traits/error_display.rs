use crate::traits::code_path::CodePath;
use crate::traits::separator_symbol::SeparatorSymbol;

pub trait ErrorDisplay {
    fn error_display(&self) -> String;
}

pub trait ErrorDisplayInner<ConfigGeneric> {
    fn error_display_inner(&self, config: &ConfigGeneric) -> String;
}

impl<SelfGeneric, ConfigGeneric> ErrorDisplayInner<ConfigGeneric> for SelfGeneric
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone,
    SelfGeneric: crate::traits::get_source::GetSourceAsString<ConfigGeneric>
        + crate::traits::get_code_occurence::GetCodeOccurenceOldWay,
{
    fn error_display_inner(&self, config: &ConfigGeneric) -> String {
        let code_occurence = self.get_code_occurence_old_way();
        format!(
            "{}{}{} {} host: {:?} pid: {}",
            self.get_source_as_string(config),
            config.symbol(),
            code_occurence.get_code_path(config.get_source_place_type()),
            chrono::DateTime::<chrono::Utc>::from(
                std::time::UNIX_EPOCH + code_occurence.time_file_line_column.time,
            )
            .with_timezone(&chrono::FixedOffset::east_opt(*config.get_timezone()).unwrap())
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
            gethostname::gethostname(),
            std::process::id()
        )
    }
}

pub trait ToStringHandle<ConfigGeneric> {
    fn to_string_handle(&self, config: &ConfigGeneric) -> String;
}

impl<VecElementGeneric, ConfigGeneric> ToStringHandle<ConfigGeneric> for Vec<VecElementGeneric>
where
    ConfigGeneric: crate::traits::fields::GetLogType,
    VecElementGeneric: std::fmt::Display, //ToStringHandle<ConfigGeneric>,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        let symbol = config.symbol();
        let source_as_string = self.iter().fold(String::from(""), |mut acc, vec_element| {
            acc.push_str(
                &vec_element
                    // .to_string_handle(config)
                    .to_string()
                    .lines()
                    .fold(String::from(""), |mut acc, vec_element| {
                        acc.push_str(&format!(" {}{}", vec_element, symbol));
                        acc
                    }),
            );
            acc
        });
        format!("[{}{}]", symbol, source_as_string)
    }
}

impl<HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric> ToStringHandle<ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: std::fmt::Display, //ToStringHandle<ConfigGeneric>,
    ConfigGeneric: crate::traits::fields::GetLogType,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        let symbol = config.symbol();
        let source_as_string = self.iter().fold(String::from(""), |mut acc, (key, value)| {
            let source_element_as_string = value
                // .to_string_handle(config)
                .to_string()
                .lines()
                .fold(String::from(""), |mut acc, line| {
                    acc.push_str(&format!(" {}{}", line, symbol));
                    acc
                });
            acc.push_str(&format!(
                "{} [{}{}]{}",
                key, symbol, source_element_as_string, symbol
            ));
            acc
        });
        source_as_string
    }
}

pub trait OriginSourceToString {
    fn origin_source_to_string(&self) -> String;
}
