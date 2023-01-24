use crate::{common::where_was::WhereWas, config_mods::source_place_type::SourcePlaceType};

pub trait WithTracing<T> {
    fn with_tracing(source: T, where_was: WhereWas, source_place_type: &SourcePlaceType) -> Self;
}
