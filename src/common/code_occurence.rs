use crate::common::git::git_info::GitInformation;
use crate::common::where_was::GitInfoForWhereWas;
use crate::common::where_was::WhereWas;
use crate::traits::code_occurence::CodeOccurenceTrait;
use crate::traits::file_line_column::FileLineColumnTrait;
use crate::traits::readable_time::ReadableTimeTrait;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CodeOccurence {
    where_was_hashmap: HashMap<GitInfoForWhereWas, Vec<TimeFileLineColumnIncrement>>,
}

impl CodeOccurenceTrait for CodeOccurence {
    fn insert(&mut self, key: GitInfoForWhereWas, value_element: TimeFileLineColumn) {
        match self.where_was_hashmap.is_empty() {
            true => {
                self.where_was_hashmap.insert(
                    key,
                    vec![TimeFileLineColumnIncrement {
                        increment: 0,
                        value: value_element,
                    }],
                );
            }
            false => {
                let last_increment = {
                    let mut increment_handle = 0;
                    self.where_was_hashmap.iter().for_each(|(_, v)| {
                        v.iter().for_each(|e| {
                            if e.increment > increment_handle {
                                increment_handle = e.increment;
                            }
                        });
                    });
                    increment_handle
                };
                self.where_was_hashmap
                    .entry(key)
                    .and_modify(|vec_existing_value_elements| {
                        vec_existing_value_elements.push(TimeFileLineColumnIncrement {
                            increment: last_increment,
                            value: value_element.clone(), //todo how to rewrite it without clone() ?
                        });
                    })
                    .or_insert_with(|| {
                        vec![TimeFileLineColumnIncrement {
                            increment: last_increment,
                            value: value_element,
                        }]
                    });
            }
        };
    }
}

#[derive(Debug, Clone)]
pub struct TimeFileLineColumnIncrement {
    pub increment: u64, //potential overflow?
    pub value: TimeFileLineColumn,
}

#[derive(Debug, Clone)]
pub struct TimeFileLineColumn {
    pub time: std::time::Duration,
    pub file_line_column: FileLineColumn,
}

impl ReadableTimeTrait for TimeFileLineColumn {
    fn readable_time(&self, timezone: i32) -> String {
        chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.time)
            .with_timezone(&chrono::FixedOffset::east(timezone))
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}

impl FileLineColumnTrait for TimeFileLineColumn {
    fn file_line_column(&self) -> String {
        self.file_line_column()
    }
    //todo make it const fn
    fn github_file_line_column(
        &self,
        git_info: &crate::common::where_was::GitInfoForWhereWas,
    ) -> String {
        self.github_file_line_column(git_info)
    }
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
