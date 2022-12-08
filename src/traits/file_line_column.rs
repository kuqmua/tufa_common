pub trait GetFile {
    fn get_file(&self) -> &String;
}
pub trait GetLine {
    fn get_line(&self) -> u32;
}
pub trait GetColumn {
    fn get_column(&self) -> u32;
}
