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
