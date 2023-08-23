#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PostgresNumber(pub u32); //u32::Max == postgres limited by the number of tuples that can fit onto 4,294,967,295 pages (rows per table)

impl std::fmt::Display for PostgresNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery for PostgresNumber {
    fn bind_sqlx_query(
        self,
        mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.0);
        query
    }
}

impl crate::common::url_encode::UrlEncode for PostgresNumber {
    fn url_encode(&self) -> std::string::String {
        urlencoding::encode(&self.0.to_string()).to_string()
    }
}

impl crate::server::routes::helpers::get_inner_length::GetInnerLength for PostgresNumber {
    fn get_inner_length(&self) -> usize {
        1
    }
}
