use crate::common::code_occurence::TimeFileLineColumn;
use crate::common::where_was::GitInfoForWhereWas;
use crate::config_mods::source_place_type::SourcePlaceType;

pub trait CodeOccurenceTrait {
    fn new(key: GitInfoForWhereWas, value_element: TimeFileLineColumn) -> Self;
    fn insert(&mut self, key: GitInfoForWhereWas, value_element: TimeFileLineColumn);
    fn tracing_log(&self, source_place_type: &SourcePlaceType) -> String;
}
