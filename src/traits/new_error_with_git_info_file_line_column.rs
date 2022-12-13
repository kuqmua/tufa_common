use crate::traits::code_occurence_methods::CodeOccurenceNew;
use crate::traits::code_occurence_methods::CodeOccurenceNewErrorWithOneAddition;
use crate::traits::init_error::InitError;

pub trait NewErrorWithGitInfoFileLineColumn<SourceGeneric> {
    fn new_error_with_git_info_file_line_column(
        source: SourceGeneric,
        git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self;
}

impl<SourceGeneric, ReturnSelfGeneric> NewErrorWithGitInfoFileLineColumn<SourceGeneric>
    for ReturnSelfGeneric
where
    ReturnSelfGeneric: InitError<SourceGeneric>,
{
    fn new_error_with_git_info_file_line_column(
        source: SourceGeneric,
        git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> ReturnSelfGeneric {
        ReturnSelfGeneric::init_error(
            source,
            crate::common::code_occurence::CodeOccurence::new(git_info, file, line, column),
        )
    }
}
