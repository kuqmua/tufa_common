pub trait NewError<T> {
    fn new(source: T, where_was: crate::common::where_was::WhereWas) -> Self;
}
