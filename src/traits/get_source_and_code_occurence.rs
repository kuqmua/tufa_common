pub trait GetSourceAndCodeOccurence {
    fn get_source_and_code_occurence(
        &self,
    ) -> crate::common::source_and_code_occurence::SourceAndCodeOccurence;
}

impl<SelfGeneric> GetSourceAndCodeOccurence for SelfGeneric
where
    SelfGeneric:
        crate::traits::get_source::GetSource + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn get_source_and_code_occurence(
        &self,
    ) -> crate::common::source_and_code_occurence::SourceAndCodeOccurence {
        crate::common::source_and_code_occurence::SourceAndCodeOccurence {
            source: self.get_source(),
            code_occurence: self.get_code_occurence().clone(), //todo
        }
    }
}
