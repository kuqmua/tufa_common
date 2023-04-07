pub trait GetCodeOccurence<'a> {
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence;
}

pub trait GetCodeOccurenceWithSerializeDeserialize<'a> {
    fn get_code_occurence_with_serialize_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize;
}
