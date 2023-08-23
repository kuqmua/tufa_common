pub trait GenerateBindIncrements {
    fn generate_bind_increments(&self, increment: &mut u64) -> std::string::String;
    fn bind_sqlx_query_x(
        self,
        query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>;
}
