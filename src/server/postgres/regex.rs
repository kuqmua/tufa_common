#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Regex(pub std::string::String);

impl crate::server::postgres::bind_query::BindQuery for Regex {
    fn generate_bind_increments(&self, increment: &mut u64) -> std::string::String {
        *increment += 1;
        format!("${increment}")
    }
    fn bind_value_to_query(
        self,
        mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.0);
        query
    }
}
