use crate::common::code_occurence::TimeFileLineColumn;
use crate::common::code_occurence::TimeFileLineColumnIncrement;
use crate::common::where_was::GitInfoForWhereWas;
use crate::config_mods::log_type::LogType;
use crate::config_mods::source_place_type::SourcePlaceType;
use std::collections::HashMap;

pub trait CodeOccurenceTrait {
    fn insert_with_key_check(&mut self, key: GitInfoForWhereWas, value_element: TimeFileLineColumn);
    fn add(&mut self, hashmap: HashMap<GitInfoForWhereWas, Vec<TimeFileLineColumnIncrement>>);
    fn log(
        &self,
        source_place_type: &SourcePlaceType,
        log_type: LogType,
        source: String,
        style: ansi_term::Style,
    );
}
