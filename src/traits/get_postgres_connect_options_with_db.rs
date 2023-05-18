pub trait GetPostgresConnectOptionsWithDb {
    fn get_postgres_connect_options_with_db(&self) -> sqlx::postgres::PgConnectOptions;
}

impl<SelfGeneric> GetPostgresConnectOptionsWithDb for SelfGeneric
where
    Self: crate::traits::config_fields::GetPostgresDb
        + crate::traits::get_postgres_connect_options_without_db::GetPostgresConnectOptionsWithoutDb,
{
    fn get_postgres_connect_options_with_db(&self) -> sqlx::postgres::PgConnectOptions {
        let mut options = self.get_postgres_connect_options_without_db().database(&self.get_postgres_db());
        {
            use sqlx::ConnectOptions;
            options.log_statements(tracing::log::LevelFilter::Trace)
        };
        options
    }
}
