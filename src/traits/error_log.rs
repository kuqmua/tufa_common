pub trait ErrorLog<ConfigGeneric> {
    fn log(&self, config: &ConfigGeneric);
}

impl<ConfigGeneric, SelfGeneric> ErrorLog<ConfigGeneric> for SelfGeneric
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::config_log::ConfigLog
        + crate::traits::fields::GetLogType 
        + crate::traits::get_color::ErrorColorBold
        ,
    SelfGeneric:
        crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
            ConfigGeneric,
        >
        + crate::traits::get_code_occurence::GetCodeOccurenceAsString<ConfigGeneric>
        + crate::traits::get_source::GetSourceAsString<ConfigGeneric>
        + crate::traits::error_to_string::ErrorToString<ConfigGeneric>,
{
    fn log(&self, config: &ConfigGeneric) {
        config.log(self.error_to_string(config));
    }
}