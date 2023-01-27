use super::error_display::ToStringHandle;

pub trait GetCodeOccurenceOldWay {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay;
}

pub trait GetCodeOccurenceAsString {
    fn get_code_occurence_as_string(&self) -> String;
}

impl<SelfGeneric> GetCodeOccurenceAsString for SelfGeneric
where
    SelfGeneric: crate::traits::get_code_occurence::GetCodeOccurenceOldWay,
{
    fn get_code_occurence_as_string(&self) -> String {
        self.get_code_occurence_old_way().to_string_handle()
    }
}
