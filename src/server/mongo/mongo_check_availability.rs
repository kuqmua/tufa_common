use crate::common::where_was::WhereWas;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
use crate::traits::get_source::GetSource;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_methods::WhereWasMethods;
use impl_display_for_error::ImplDisplayForError;
use impl_error_with_tracing::ImplErrorWithTracingFromCrate;
use impl_get_source::ImplGetSourceFromCrate;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromCrate;
use init_error::InitErrorFromCrate;
use mongodb::options::ClientOptions;
use mongodb::Client;

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckAvailabilityError<'a> {
    Mongo {
        #[eo_display_foreign_type]
        error: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_check_availability<'a>(
    client_options: ClientOptions,
    db_name: &str,
    source_place_type: &SourcePlaceType,
    should_trace: bool,
) -> Result<(), Box<MongoCheckAvailabilityError<'a>>> {
    match Client::with_options(client_options) {
        Err(e) => Err(Box::new(MongoCheckAvailabilityError::Mongo {
            error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        })),
        Ok(client) => {
            if let Err(e) = client.database(db_name).list_collection_names(None).await {
                return Err(Box::new(MongoCheckAvailabilityError::Mongo {
                    error: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }));
            }
            Ok(())
        }
    }
}
