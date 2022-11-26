use crate::common::code_occurence::TimeFileLineColumn;
use crate::common::where_was::GitInfoForWhereWas;

pub trait CodeOccurenceInsertTrait {
    fn insert(&mut self, key: GitInfoForWhereWas, value_element: TimeFileLineColumn);
}
