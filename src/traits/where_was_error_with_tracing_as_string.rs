use crate::config::source_place_type::SourcePlaceType;
use crate::helpers::git::git_info::GitInformation;
use crate::traits::get_bunyan_where_was::GetBunyanWhereWas;
use crate::where_was::WhereWas;

pub trait WhereWasErrorWithTracingAsString<T> {
    fn where_was_error_with_tracing_as_string(
        source: T,
        where_was: WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
    ) -> String;
}

impl<T, SourceType> WhereWasErrorWithTracingAsString<SourceType> for T
where
    SourceType: GetBunyanWhereWas,
{
    fn where_was_error_with_tracing_as_string(
        source: SourceType,
        where_was: WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
    ) -> String {
        format!(
            "{} {}",
            where_was.file_line_column(),
            source.get_bunyan_where_was(source_place_type, git_info)
        )
    }
}
