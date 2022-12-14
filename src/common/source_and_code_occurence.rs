#[derive(Debug, Clone)]
pub struct SourceAndCodeOccurence {
    pub source: String,
    pub code_occurence: crate::common::code_occurence::CodeOccurence,
}
