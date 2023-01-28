use super::error_display::ToStringHandle;
use crate::{
    global_variables::runtime::config, traits::error_display::ToStringHandleCodeOccurence,
};

pub trait GetCodeOccurenceOldWay {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay;
}
