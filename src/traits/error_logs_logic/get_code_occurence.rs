pub trait GetCodeOccurenceLifetime<'a> {
    fn get_code_occurence_lifetime(&self) -> &crate::common::code_occurence::CodeOccurenceLifetime;
}

pub trait GetCodeOccurenceLifetimeWithDeserialize<'a> {
    fn get_code_occurence_lifetime_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize;
}
