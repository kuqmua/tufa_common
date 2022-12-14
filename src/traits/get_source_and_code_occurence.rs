pub trait GetSourceAndCodeOccurence {
    fn get_code_occurence_with_source(
        &self,
    ) -> &crate::common::source_and_code_occurence::SourceAndCodeOccurence;
}
