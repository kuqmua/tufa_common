use crate::common::code_occurence::TimeFileLineColumnIncrement;
use crate::common::git::git_info::GitInformationWithoutLifetimes;
use std::collections::HashMap;

pub trait GetCodeOccurence {
    fn get_code_occurence(
        &self,
    ) -> &HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>;
}
