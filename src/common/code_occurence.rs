use crate::common::git::git_info::GitInformation;
use crate::common::where_was::GitInfoForWhereWas;
use crate::common::where_was::WhereWas;
use crate::traits::file_line_column::FileLineColumnTrait;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CodeOccurence {
    where_was_hashmap: HashMap<GitInfoForWhereWas, TimeFileLineColumn>,
}

// impl

#[derive(Debug, Clone)]
pub struct TimeFileLineColumn {
    pub time: std::time::Duration,
    pub file_line_column: FileLineColumn,
}
#[derive(Debug, Clone)]
pub struct FileLineColumn {
    pub file: String, //&'a str
    pub line: u32,
    pub column: u32,
}

impl FileLineColumnTrait for FileLineColumn {
    fn file_line_column(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.column)
    }
    //todo make it const fn
    fn github_file_line_column(
        &self,
        git_info: &crate::common::where_was::GitInfoForWhereWas,
    ) -> String {
        let file = self.file.clone();
        let backslash = "/";
        match file.find(backslash) {
            Some(index) => {
                git_info.get_git_source_file_link(&file[index + backslash.len()..], self.line)
            }
            None => String::from("cant find backslash symbol in file path of location"),
        }
    }
}
