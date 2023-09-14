#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGenerateBindIncrementsErrorNamed {
    CheckedAdd {
        #[eo_display_with_serialize_deserialize]
        checked_add: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

pub trait BindQuery {
    fn try_increment(&self, increment: &mut u64)
        -> Result<(), TryGenerateBindIncrementsErrorNamed>;
    fn try_generate_bind_increments(
        &self,
        increment: &mut u64,
    ) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed>;
    fn bind_value_to_query(
        self,
        query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>;
}

impl crate::server::postgres::bind_query::BindQuery for std::string::String {
    fn try_increment(
        &self,
        increment: &mut u64,
    ) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                Ok(())
            }
            None => Err(crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                checked_add: std::string::String::from("checked_add is None"),
                code_occurence: crate::code_occurence_tufa_common!(),
            })
        }
    }
    fn try_generate_bind_increments(
        &self,
        increment: &mut u64,
    ) -> Result<
        std::string::String,
        crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
    > {
        let mut increments = std::string::String::default();
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                increments.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                    checked_add: std::string::String::from("checked_add is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        }
        Ok(increments)
    }
    fn bind_value_to_query(
        self,
        mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self);
        query
    }
}
