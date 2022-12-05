pub trait CodeOccurenceMethods {
    fn new(
        git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self;
    fn add(&mut self, another_code_occurence: Self);
}
