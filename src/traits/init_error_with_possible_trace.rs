use crate::config::source_place_type::SourcePlaceType;
use crate::helpers::git::git_info::GitInformation;
use crate::traits::new_error::NewError;
use crate::traits::with_tracing::WithTracing;
use crate::where_was::WhereWas;

pub trait InitErrorWithPossibleTrace<GenericErrorStruct, GenericErrorStructSource>
where
    GenericErrorStruct: WithTracing<GenericErrorStructSource> + NewError<GenericErrorStructSource>,
{
    fn init_error_with_possible_trace(
        source: GenericErrorStructSource,
        where_was: WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        should_trace: bool,
    ) -> Self;
}

impl<GenericErrorStruct, GenericErrorStructSource>
    InitErrorWithPossibleTrace<GenericErrorStruct, GenericErrorStructSource> for GenericErrorStruct
where
    GenericErrorStruct: WithTracing<GenericErrorStructSource> + NewError<GenericErrorStructSource>,
{
    fn init_error_with_possible_trace(
        source: GenericErrorStructSource,
        where_was: WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        should_trace: bool,
    ) -> Self {
        match should_trace {
            true => Self::with_tracing(source, where_was, source_place_type, git_info),
            false => Self::new(source, where_was),
        }
    }
}
