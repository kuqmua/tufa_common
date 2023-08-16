//just for example
// pub trait BindSqlxQuery {
//     fn bind_sqlx_query<'q, TableScheme: for<'a> serde::Deserialize<'a>>(
//         self,
//         query: sqlx::query::QueryAs<'q, sqlx::Postgres, TableScheme, sqlx::postgres::PgArguments>,
//     ) -> sqlx::query::QueryAs<'q, sqlx::Postgres, TableScheme, sqlx::postgres::PgArguments>;
// }

// impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery for BigserialIds {
//     fn bind_sqlx_query<'q, TableScheme: for<'a> serde::Deserialize<'a>>(
//         self,
//         mut query: sqlx::query::QueryAs<
//             'q,
//             sqlx::Postgres,
//             TableScheme,
//             sqlx::postgres::PgArguments,
//         >,
//     ) -> sqlx::query::QueryAs<'q, sqlx::Postgres, TableScheme, sqlx::postgres::PgArguments> {
//         for element in self.0 {
//             query = query.bind(element.into_inner());
//         }
//         query
//     }
// }

pub trait BindSqlxQuery {
    fn bind_sqlx_query(
        self,
        query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>;
}
