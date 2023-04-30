#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresCheckAvailabilityError<'a> {
    Postgres {
        #[eo_display_foreign_type]
        postgres_connect: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn postgres_check_availability<'a>(
    postgres_url: String,
    timeout: u64,
) -> Result<(), Box<PostgresCheckAvailabilityError<'a>>> {
    if let Err(e) = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect_timeout(std::time::Duration::from_millis(timeout))
        .connect(&postgres_url)
        .await
    {
        return Err(Box::new(PostgresCheckAvailabilityError::Postgres {
            postgres_connect: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }));
    }
    Ok(())
}
