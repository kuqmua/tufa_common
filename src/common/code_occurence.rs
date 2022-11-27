use crate::common::git::git_info::GitInformation;
use crate::common::where_was::GitInfoForWhereWas;
use crate::common::where_was::WhereWas;
use crate::config_mods::log_type::LogType;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::code_occurence::CodeOccurenceTrait;
use crate::traits::file_line_column::FileLineColumnTrait;
use crate::traits::readable_time::ReadableTimeTrait;
use crate::traits::separator_symbol_trait::SeparatorSymbolTrait;
use std::collections::HashMap;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct CodeOccurence {
    where_was_hashmap: HashMap<GitInfoForWhereWas, Vec<TimeFileLineColumnIncrement>>,
}

pub enum SeparatorSymbol {
    LineBreak,
    DotSpace,
}

//its must be implement for the future of
impl SeparatorSymbolTrait for SeparatorSymbol {
    fn symbol(&self) -> &str {
        match self {
            SeparatorSymbol::LineBreak => "\n",
            SeparatorSymbol::DotSpace => ", ",
        }
    }
    fn pop_last(&self, vec: &mut Vec<String>) {
        for i in 0..self.symbol().len() {
            vec.pop();
        }
    }
}

impl CodeOccurenceTrait for CodeOccurence {
    fn new(key: GitInfoForWhereWas, value_element: TimeFileLineColumn) -> Self {
        Self {
            where_was_hashmap: HashMap::from([(
                key,
                vec![TimeFileLineColumnIncrement {
                    increment: 0,
                    value: value_element,
                }],
            )]),
        }
    }
    fn insert(&mut self, key: GitInfoForWhereWas, value_element: TimeFileLineColumn) {
        let last_increment = {
            let mut increment_handle = 0;
            self.where_was_hashmap.values().for_each(|v| {
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
    fn log(&self, source_place_type: &SourcePlaceType, log_type: LogType, source: String) {
        match source_place_type {
            SourcePlaceType::Source => {
                let mut vec: Vec<OccurenceFilter> =
                    Vec::with_capacity(self.where_was_hashmap.values().fold(0, |mut acc, elem| {
                        acc += elem.len();
                        acc
                    }));
                self.where_was_hashmap.iter().for_each(|(k, v)| {
                    v.iter().for_each(|e| {
                        vec.push(OccurenceFilter {
                            increment: e.increment,
                            occurence: e.value.file_line_column(),
                        })
                    })
                });
                //todo check reserve or not
                vec.sort_by(|a, b| a.increment.cmp(&b.increment));
                let occurences = vec
                    .into_iter()
                    .map(|e| format!("{}/n", e.occurence))
                    .collect::<Vec<String>>();
            }
            SourcePlaceType::Github => {
                let mut vec: Vec<OccurenceFilter> =
                    Vec::with_capacity(self.where_was_hashmap.values().fold(0, |mut acc, elem| {
                        acc += elem.len();
                        acc
                    }));
                self.where_was_hashmap.iter().for_each(|(k, v)| {
                    v.iter().for_each(|e| {
                        vec.push(OccurenceFilter {
                            increment: e.increment,
                            occurence: e.value.github_file_line_column(k),
                        })
                    })
                });
                //todo check reserve or not
                vec.sort_by(|a, b| a.increment.cmp(&b.increment));
                let occurences = vec
                    .into_iter()
                    .map(|e| e.occurence)
                    .collect::<Vec<String>>();
            }
            SourcePlaceType::None => {
                // ()
            }
        }
    }
}

// impl Vec<Occurence> {
//     pub fn kekw(&self) -> String {
//         let mut value = String::from("");
//         self.iter().for_each(|c| {
//             value.push_str(&format!("{}\n", c.occurence));
//         });
//         value
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Occurence {
//     pub occurence: String,
// }

#[derive(Debug, Clone)]
pub struct OccurenceFilter {
    pub increment: u64, //potential overflow?
    pub occurence: String,
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
