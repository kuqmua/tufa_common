pub trait NewErrorTest<T> {
    fn new_error_test(
        source: T,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    ) -> Self;
}
