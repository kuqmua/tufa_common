pub trait BindSqlxQuery {
    fn bind_sqlx_query<'q, TableScheme: for<'a> serde::Deserialize<'a>>(
        self,
        query: sqlx::query::QueryAs<'q, sqlx::Postgres, TableScheme, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::QueryAs<'q, sqlx::Postgres, TableScheme, sqlx::postgres::PgArguments>;
}
