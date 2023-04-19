use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
use impl_display_for_error::ImplDisplayForError;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use std::time::Duration;

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckAvailabilityError<'a> {
    Postgres {
        #[eo_display_foreign_type]
        error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn postgres_check_availability<'a>(
    postgres_url: String,
    should_trace: bool,
) -> Result<(), Box<PostgresCheckAvailabilityError<'a>>> {
    if let Err(e) = PgPoolOptions::new()
        .max_connections(1)
        .connect_timeout(Duration::from_millis(CONFIG.postgres_connection_timeout))
        .connect(&postgres_url)
        .await
    {
        return Err(Box::new(PostgresCheckAvailabilityError::Postgres {
            error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }));
    }
    Ok(())
}
