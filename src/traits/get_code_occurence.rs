use crate::traits::code_path::CodePath;

pub trait GetCodeOccurence {
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence;
}

pub trait GetCodeOccurenceOldWay {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay;
}

pub trait GetCodeOccurenceAsString<ConfigGeneric> {
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String;
}

impl<ConfigGeneric, SelfGeneric> GetCodeOccurenceAsString<ConfigGeneric> for SelfGeneric
where
    ConfigGeneric: crate::traits::fields::GetTimezone + crate::traits::fields::GetSourcePlaceType,
    SelfGeneric: crate::traits::get_code_occurence::GetCodeOccurenceOldWay,
{
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String {
        let code_occurence = self.get_code_occurence_old_way();
        format!(
            "{} {}",
            code_occurence.get_code_path(config.get_source_place_type()),
            chrono::DateTime::<chrono::Utc>::from(
                std::time::UNIX_EPOCH + code_occurence.time_file_line_column.time,
            )
            .with_timezone(&chrono::FixedOffset::east_opt(*config.get_timezone()).unwrap())
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
        )
    }
}
