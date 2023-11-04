#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PostgresBigint(pub i64);

impl std::fmt::Display for PostgresBigint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::server::postgres::bind_query::BindQuery for PostgresBigint {
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

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for PostgresBigint {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        self.to_string()
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresBigintFromStrErrorNamed {
    NotCorrectValue {
        #[eo_display_with_serialize_deserialize]
        not_correct_value: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

impl std::str::FromStr for PostgresBigint {
    type Err = PostgresBigintFromStrErrorNamed;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match std::str::FromStr::from_str(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(Self::Err::NotCorrectValue {
                not_correct_value: format!("wrong rust i64 value: {value}, error: {e}"),
                code_occurence: crate::code_occurence_tufa_common!(),
            })
        }
    }
}