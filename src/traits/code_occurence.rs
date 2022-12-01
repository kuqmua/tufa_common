use crate::common::code_occurence::TimeFileLineColumn;
use crate::common::where_was::GitInfoForWhereWas;
use crate::config_mods::log_type::LogType;
use crate::config_mods::source_place_type::SourcePlaceType;

pub trait CodeOccurenceTrait {
    // fn new_with_increment(key: GitInfoForWhereWas, value_element: TimeFileLineColumn) -> Self;
    fn insert(&mut self, key: GitInfoForWhereWas, value_element: TimeFileLineColumn);
    fn log(
        &self,
        source_place_type: &SourcePlaceType,
        log_type: LogType,
        source: String,
        error_red: u8,
        error_green: u8,
        error_blue: u8,
    );
}
