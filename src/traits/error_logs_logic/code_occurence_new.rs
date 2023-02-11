pub trait CodeOccurenceNew<'a> {
    fn new(
        git_info: &'a crate::common::git::git_info::GitInformation<'a>,
        file: &'a str,
        line: u32,
        column: u32,
    ) -> Self;
}
