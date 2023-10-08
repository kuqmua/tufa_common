#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RegexFilter {
    pub regex: std::string::String,
    pub conjuctive_operator: crate::server::postgres::conjuctive_operator::ConjunctiveOperator,
}

impl crate::server::postgres::bind_query::BindQuery for RegexFilter {
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
        query = query.bind(self.regex);
        query
    }
}

impl crate::server::postgres::bind_query::BindQuery for Vec<RegexFilter> {
    fn try_increment(
        &self,
        increment: &mut u64,
    ) -> Result<(), crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed> {
        for _ in self {
            match increment.checked_add(1) {
                Some(incr) => {
                    *increment = incr;
                },
                None => {
                    return Err(crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed::CheckedAdd { 
                        checked_add: std::string::String::from("checked_add is None"), 
                        code_occurence: crate::code_occurence_tufa_common!(), 
                    });
                },
            }
        }
        Ok(())
    }
    fn try_generate_bind_increments(&self, increment: &mut u64) -> Result<std::string::String, crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed> {
        let mut value = std::string::String::default();
        for _ in self {
            match increment.checked_add(1) {
                Some(incr) => {
                    *increment = incr;
                    value.push_str(&format!("${increment},"));
                },
                None => {
                    return Err(crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed::CheckedAdd { 
                        checked_add: std::string::String::from("checked_add is None"), 
                        code_occurence: crate::code_occurence_tufa_common!(), 
                    });
                },
            }
        }
        value.pop();
        Ok(value)
    }
    fn bind_value_to_query(
        self,
        mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self {
            query = query.bind(element.regex);
        }
        query
    }
}