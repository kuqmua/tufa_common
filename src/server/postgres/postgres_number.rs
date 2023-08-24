#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PostgresNumber(pub u32); //u32::Max == postgres limited by the number of tuples that can fit onto 4,294,967,295 pages (rows per table)

impl std::fmt::Display for PostgresNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::common::url_encode::UrlEncode for PostgresNumber {
    fn url_encode(&self) -> std::string::String {
        urlencoding::encode(&self.0.to_string()).to_string()
    }
}

impl crate::server::postgres::bind_query::BindQuery for PostgresNumber {
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
