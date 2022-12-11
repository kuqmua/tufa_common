use crate::traits::code_occurence_methods::CodeOccurenceNew;
use crate::traits::code_occurence_methods::CodeOccurenceNewWithAddition;

pub trait NewErrorTest<T> {
    fn new_with_git_info_file_line_column(
        source: T,
        git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self;
    fn new_with_code_occurance(
        source: T,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    ) -> Self;
}

impl<SourceGeneric, ReturnSelfGeneric> crate::traits::new_error_test::NewErrorTest<SourceGeneric>
    for ReturnSelfGeneric
where
    ReturnSelfGeneric: NewErrorTestTestTest<SourceGeneric>,
{
    fn new_with_git_info_file_line_column(
        source: SourceGeneric,
        git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> ReturnSelfGeneric {
        ReturnSelfGeneric::new(
            source,
            crate::common::code_occurence::CodeOccurence::new(git_info, file, line, column),
        )
    }
    fn new_with_code_occurance(
        source: SourceGeneric,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    ) -> ReturnSelfGeneric {
        ReturnSelfGeneric::new(source, code_occurence)
    }
}

pub trait NewErrorTestTestTest<SourceGeneric> {
    fn new(
        source: SourceGeneric,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    ) -> Self;
}
