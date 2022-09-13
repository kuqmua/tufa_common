use crate::where_was::WhereWasOneOrMany;

pub trait GetWhereWasOneOrMany {
    fn get_where_was_one_or_many(&self) -> WhereWasOneOrMany;
}
