pub trait InitError<SourceGeneric> {
    fn init_error(
        source: SourceGeneric,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    ) -> Self;
}
