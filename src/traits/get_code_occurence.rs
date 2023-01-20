use super::error_display::ToStringHandle;

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
        self.get_code_occurence_old_way().to_string_handle(config)
    }
}
