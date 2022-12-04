use crate::common::code_occurence::TimeFileLineColumn;
use crate::common::code_occurence::TimeFileLineColumnIncrement;
use crate::common::git::git_info::GitInformationWithoutLifetimes;
use crate::config_mods::log_type::LogType;
use crate::config_mods::source_place_type::SourcePlaceType;
use std::collections::HashMap;

pub trait LogCodeOccurence {
    fn log_code_occurence(
        &self,
        source_place_type: &SourcePlaceType,
        log_type: LogType,
        source: String,
        style: ansi_term::Style,
    );
}
