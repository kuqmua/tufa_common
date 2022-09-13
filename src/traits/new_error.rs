use crate::where_was::WhereWas;

pub trait NewError<T> {
    fn new(source: T, where_was: WhereWas) -> Self;
}
