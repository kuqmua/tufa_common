use crate::common::code_occurence::TimeFileLineColumn;
use crate::common::code_occurence::TimeFileLineColumnIncrement;
use crate::common::git::git_info::GitInformationWithoutLifetimes;
use std::collections::HashMap;

pub trait CodeOccurenceMethods {
    fn insert_with_key_check(
        &mut self,
        key: GitInformationWithoutLifetimes,
        value_element: TimeFileLineColumn,
    );
    fn add(
        &mut self,
        hashmap: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
    );
}
