pub trait GetCodeOccurence {
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence;
}

pub trait GetCodeOccurenceLifetime<'a> {
    fn get_code_occurence_lifetime(&self) -> &crate::common::code_occurence::CodeOccurenceLifetime;
}
