#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PostgresNumber(pub u32); //u32::Max == postgres limited by the number of tuples that can fit onto 4,294,967,295 pages (rows per table)

impl std::fmt::Display for PostgresNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for PostgresNumber {
    fn serde_urlencoded_parameter(
        &self,
    ) -> Result<
        std::string::String,
        crate::common::serde_urlencoded::SerdeUrlencodedParameterErrorNamed,
    > {
        match serde_urlencoded::to_string(self.0) {
            Ok(value) => Ok(value),
            Err(e) => Err(
                crate::common::serde_urlencoded::SerdeUrlencodedParameterErrorNamed::UrlEncode {
                    url_encode: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                },
            ),
        }
    }
}
