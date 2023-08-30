#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RegexFilter {
    pub regex: std::string::String,
    pub conjuctive_operator: crate::server::postgres::conjuctive_operator::ConjunctiveOperator,
}

impl crate::server::postgres::bind_query::BindQuery for RegexFilter {
    fn generate_bind_increments(&self, increment: &mut u64) -> std::string::String {
        *increment += 1;
        format!("${increment}")
    }
    fn bind_value_to_query(
        self,
        mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.regex);
        query
    }
}
