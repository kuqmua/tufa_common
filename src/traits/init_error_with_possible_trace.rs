use crate::common::where_was::WhereWas;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::new_error::NewError;
use crate::traits::with_tracing::WithTracing;

pub trait InitErrorWithPossibleTrace<GenericErrorStruct, GenericErrorStructSource>
where
    GenericErrorStruct: WithTracing<GenericErrorStructSource> + NewError<GenericErrorStructSource>,
{
    fn init_error_with_possible_trace(
        source: GenericErrorStructSource,
        where_was: WhereWas,
        source_place_type: &SourcePlaceType,
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
        should_trace: bool,
    ) -> Self {
        match should_trace {
            true => Self::with_tracing(source, where_was, source_place_type),
            false => Self::new(source, where_was),
        }
    }
}
