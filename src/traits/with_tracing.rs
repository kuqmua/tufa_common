use crate::helpers::where_was::WhereWas;

pub trait WithTracing<T> {
    fn with_tracing(source: T, where_was: WhereWas) -> Self;
}
