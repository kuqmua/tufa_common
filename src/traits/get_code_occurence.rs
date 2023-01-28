use super::error_display::ToStringHandle;
use crate::{
    global_variables::runtime::config, traits::error_display::ToStringHandleCodeOccurence,
};

pub trait GetCodeOccurenceOldWay {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay;
}

pub trait GetCodeOccurenceAsString<ConfigGeneric> {
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String;
}

impl<SelfGeneric, ConfigGeneric> GetCodeOccurenceAsString<ConfigGeneric> for SelfGeneric
where
    SelfGeneric: crate::traits::get_code_occurence::GetCodeOccurenceOldWay,
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::get_server_address::GetServerAddress,
{
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String {
        self.get_code_occurence_old_way()
            .to_string_handle_code_occurence(config)
    }
}
