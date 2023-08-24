pub trait BindQuery {
    fn generate_bind_increments(&self, increment: &mut u64) -> std::string::String;
    fn bind_value_to_query(
        self,
        query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>;
}
