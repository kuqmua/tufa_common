pub trait GetSourceAndCodeOccurence {
    fn get_code_occurence_with_source(
        &self,
    ) -> crate::common::source_and_code_occurence::SourceAndCodeOccurence;
}

impl<SelfGeneric> GetSourceAndCodeOccurence for SelfGeneric
where
    Self:
        crate::traits::get_source::GetSource + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn get_code_occurence_with_source(
        &self,
    ) -> crate::common::source_and_code_occurence::SourceAndCodeOccurence {
        crate::common::source_and_code_occurence::SourceAndCodeOccurence {
            source: self.get_source(),
            code_occurence: self.get_code_occurence().clone(), //todo
        }
    }
}
