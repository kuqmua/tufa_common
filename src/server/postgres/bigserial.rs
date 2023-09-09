#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Bigserial(#[serde(deserialize_with = "deserialize_bigserial")] i64);

// DB is the database driver
// `'r` is the lifetime of the `Row` being decoded
impl<'r, DB: sqlx::Database> sqlx::Decode<'r, DB> for Bigserial
where
    // we want to delegate some of the work to string decoding so let's make sure strings
    // are supported by the database
    &'r str: sqlx::Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Bigserial, Box<dyn std::error::Error + 'static + Send + Sync>> {
        // the interface of ValueRef is largely unstable at the moment
        // so this is not directly implementable

        // however, you can delegate to a type that matches the format of the type you want
        // to decode (such as a UTF-8 string)

        let str_value = <&str as sqlx::Decode<DB>>::decode(value)?;
        let i64_value = str_value.parse::<i64>()?;
        let bigserial_value = Self::try_from(i64_value)?;
        Ok(bigserial_value)
    }
}

impl Bigserial {
    pub fn to_inner(&self) -> &i64 {
        &self.0
    }
    pub fn into_inner(self) -> i64 {
        self.0
    }
}

fn deserialize_bigserial<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::Deserialize;
    let possible_bigserial = i64::deserialize(deserializer)?;
    match possible_bigserial.is_positive() {
        true => Ok(possible_bigserial),
        false => Err(
            serde::de::Error::custom(&format!(
                "invalid type: Postgresql Bigserial `{possible_bigserial}`, expected Postgresql Bigserial as rust i64, there 1 <= *your value* <= 9223372036854775807(only positive part of rust i64)"
            )),
        )
    }
}

impl std::fmt::Display for Bigserial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum BigserialTryFromI64ErrorNamed {
    NotPositive {
        #[eo_display_with_serialize_deserialize]
        not_positive: i64,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

impl std::convert::TryFrom<i64> for Bigserial {
    type Error = BigserialTryFromI64ErrorNamed;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value.is_positive() {
            true => Ok(Bigserial(value)),
            false => Err(BigserialTryFromI64ErrorNamed::NotPositive {
                not_positive: value,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum BigserialTryFromStrErrorNamed {
    ParseIntError {
        #[eo_display]
        parse_int_error: std::num::ParseIntError,
        #[eo_display_with_serialize_deserialize]
        str_value: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NotPositive {
        #[eo_error_occurence]
        not_positive: BigserialTryFromI64ErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

impl std::convert::TryFrom<&str> for Bigserial {
    type Error = BigserialTryFromStrErrorNamed;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<i64>() {
            Ok(i64_value) => match Self::try_from(i64_value) {
                Ok(bigserial) => Ok(bigserial),
                Err(bigserial_try_from_i64_error) => {
                    Err(BigserialTryFromStrErrorNamed::NotPositive {
                        not_positive: bigserial_try_from_i64_error,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    })
                }
            },
            Err(parse_int_error) => Err(BigserialTryFromStrErrorNamed::ParseIntError {
                parse_int_error,
                str_value: value.to_string(),
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        }
    }
}

impl crate::server::postgres::bind_query::BindQuery for Bigserial {
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
        query = query.bind(self.into_inner());
        query
    }
}

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for Bigserial {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        self.to_string()
    }
}
