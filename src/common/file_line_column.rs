#[derive(Debug, Clone)] //todo fields implementations
pub struct FileLineColumn {
    pub file: String, //&'a str
    pub line: u32,
    pub column: u32,
}

impl crate::traits::get_file::GetFile for FileLineColumn {
    fn get_file(&self) -> &String {
        &self.file
    }
}

impl crate::traits::get_line::GetLine for FileLineColumn {
    fn get_line(&self) -> u32 {
        self.line
    }
}

impl crate::traits::get_column::GetColumn for FileLineColumn {
    fn get_column(&self) -> u32 {
        self.column
    }
}