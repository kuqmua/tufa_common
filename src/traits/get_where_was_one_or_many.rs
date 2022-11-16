use crate::common::where_was::WhereWasOriginOrWrapper;

pub trait GetWhereWasOriginOrWrapper {
    fn get_where_was_one_or_many(&self) -> WhereWasOriginOrWrapper;
}
