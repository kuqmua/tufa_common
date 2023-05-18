pub trait GetPostgresConnectionPool {
    fn get_postgres_connection_pool(&self) -> sqlx::PgPool;
}

impl<SelfGeneric> GetPostgresConnectionPool for SelfGeneric
where
    Self: crate::traits::config_fields::GetPostgresConnectionTimeout
        + crate::traits::get_postgres_connect_options_with_db::GetPostgresConnectOptionsWithDb,
{
    fn get_postgres_connection_pool(&self) -> sqlx::PgPool {
        sqlx::postgres::PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(*self.get_postgres_connection_timeout()))
        .connect_lazy_with(self.get_postgres_connect_options_with_db())
    }
}