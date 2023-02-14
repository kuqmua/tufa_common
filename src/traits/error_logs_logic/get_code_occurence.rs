pub trait GetCodeOccurence<'a> {
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurenceLifetime;
}

pub trait GetCodeOccurenceWithDeserialize<'a> {
    fn get_code_occurence_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize;
}
