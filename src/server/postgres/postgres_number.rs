#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PostgresNumber(pub u32); //u32::Max == postgres limited by the number of tuples that can fit onto 4,294,967,295 pages (rows per table)

impl std::fmt::Display for PostgresNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::server::postgres::bind_query::BindQuery for PostgresNumber {
    fn try_increment(
        &self,
        increment: &mut u64,
    ) -> Result<(), crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                Ok(())
            },
            None => Err(crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed::CheckedAdd { 
                checked_add: std::string::String::from("checked_add is None"), 
                code_occurence: crate::code_occurence_tufa_common!(), 
            }),
        }
    }
    fn try_generate_bind_increments(&self, increment: &mut u64) -> Result<std::string::String, crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                Ok(format!("${increment}"))
            },
            None => Err(crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed::CheckedAdd { 
                checked_add: std::string::String::from("checked_add is None"), 
                code_occurence: crate::code_occurence_tufa_common!(), 
            }),
        }
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
    fn serde_urlencoded_parameter(self) -> std::string::String {
        self.to_string()
    }
}
