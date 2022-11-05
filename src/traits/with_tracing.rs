use crate::{
    config_mods::source_place_type::SourcePlaceType, helpers::git::git_info::GitInformation,
    where_was::WhereWas,
};

pub trait WithTracing<T> {
    fn with_tracing(
        source: T,
        where_was: WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
    ) -> Self;
}
