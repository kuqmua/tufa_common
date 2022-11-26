use crate::common::git::git_info::GitInformation;
use crate::common::where_was::GitInfoForWhereWas;
use crate::traits::where_was_trait::WhereWasTrait;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CodeOccurence {
    where_was_hashmap: HashMap<GitInfoForWhereWas, TimeFileLineColumn>,
}

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

impl WhereWasTrait for WhereWas {
    fn readable_time(&self, timezone: i32) -> String {
        let datetime = chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.time)
            .with_timezone(&chrono::FixedOffset::east(timezone));
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    //todo make it const fn
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
        let index = file
            .find(backslash)
            .expect("cant find backslash symbol in file path of location"); //todo - bad code ?
        git_info.get_git_source_file_link(&file[index + backslash.len()..], self.line)
    }
}
