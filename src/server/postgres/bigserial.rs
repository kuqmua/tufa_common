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
        let bigserial_value = Self::try_from_i64(i64_value)?;
        Ok(bigserial_value)
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

impl<'a> crate::common::std_str_from_str_with_lifetime::StdStrFromStrWithLifetime<'a>
    for Bigserial
{
    type Err = BigserialTryFromStrErrorNamed;

    fn std_str_from_str_with_lifetime(str_value: &'a str) -> Result<Self, Self::Err> {
        match str_value.parse::<i64>() {
            Ok(i64_value) => match Self::try_from_i64(i64_value) {
                Ok(_) => todo!(),
                Err(bigserial_try_from_i64_error) => {
                    Err(BigserialTryFromStrErrorNamed::NotPositive {
                        not_positive: bigserial_try_from_i64_error,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    })
                }
            },
            Err(parse_int_error) => Err(BigserialTryFromStrErrorNamed::ParseIntError {
                parse_int_error,
                str_value: str_value.to_string(),
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        }
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

impl Bigserial {
    //its not TryFrom<i64> coz its not supported lifetimes in Error annotation
    pub fn try_from_i64<'a>(
        possible_bigserial: i64,
    ) -> Result<Bigserial, BigserialTryFromI64ErrorNamed> {
        if possible_bigserial.is_positive() {
            Ok(Bigserial(possible_bigserial))
        } else {
            Err(BigserialTryFromI64ErrorNamed::NotPositive {
                not_positive: possible_bigserial,
                code_occurence: crate::code_occurence_tufa_common!(),
            })
        }
    }
}

pub trait GetPostgresBigserialId {
    fn get_postgres_bigserial_id(&self) -> &Bigserial;
}
