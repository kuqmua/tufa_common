#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BigserialIds(
    #[serde(deserialize_with = "deserialize_bigserial_ids")]
    pub  Vec<crate::server::postgres::bigserial::Bigserial>,
);

fn deserialize_bigserial_ids<'de, D>(
    deserializer: D,
) -> Result<Vec<crate::server::postgres::bigserial::Bigserial>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let (vec_values, mut stringified_parse_fails, mut stringified_bigserial_fails) = {
        use serde::Deserialize;
        std::string::String::deserialize(deserializer)?
    }
    .split(',')
    .fold(
        (
            Vec::new(),
            std::string::String::default(),
            std::string::String::default(),
        ),
        |mut acc, element| {
            match element.parse::<i64>() {
                Ok(value) => match crate::server::postgres::bigserial::Bigserial::try_from(value) {
                    Ok(bigserial) => {
                        acc.0.push(bigserial);
                    }
                    Err(_) => {
                        acc.1.push_str(&format!("{element},"));
                    }
                },
                Err(_) => {
                    acc.1.push_str(&format!("{element},"));
                }
            }
            acc
        },
    );
    let default_message = "invalid type (expected array Postgresql Bigserial as rust Vec<i64>):";
    let stringified_parse_fails_message = "failed to parse each element into rust i64";
    let stringified_bigserial_fails_message = "failed to convert each element into Postgresql Bigserial - must be in range 1 <= *your value* <= 9223372036854775807(only positive part of rust i64)";
    match (
        stringified_parse_fails.is_empty(),
        stringified_bigserial_fails.is_empty(),
    ) {
        (true, true) => Ok(vec_values),
        (true, false) => {
            stringified_bigserial_fails.pop();
            Err(serde::de::Error::custom(
                &format!(
                    "{default_message} `{stringified_bigserial_fails}`, {stringified_bigserial_fails_message}")
                )
            )
        }
        (false, true) => {
            stringified_parse_fails.pop();
            Err(serde::de::Error::custom(&format!(
                "{default_message} `{stringified_parse_fails}`, {stringified_parse_fails_message}"
            )))
        }
        (false, false) => {
            stringified_parse_fails.pop();
            stringified_bigserial_fails.pop();
            Err(serde::de::Error::custom(
                &format!(
                    "{default_message} 1) `{stringified_parse_fails}`, {stringified_parse_fails_message}. 2) `{stringified_bigserial_fails}`, {stringified_bigserial_fails_message}")
                )
            )
        }
    }
}

impl crate::server::postgres::bind_query::BindQuery for BigserialIds {
    fn try_increment(
        &self,
        increment: &mut u64,
    ) -> Result<(), crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed> {
        for _ in 0..self.0.len() {
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
    fn try_generate_bind_increments(
        &self,
        increment: &mut u64,
    ) -> Result<
        std::string::String,
        crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
    > {
        let mut increments = std::string::String::default();
        for _ in 0..self.0.len() {
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
            increments.push_str(&format!("${increment}, "));
        }
        increments.pop();
        increments.pop();
        Ok(increments)
    }
    fn bind_value_to_query(
        self,
        mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.0 {
            query = query.bind(element.into_inner());
        }
        query
    }
}

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for BigserialIds {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        let mut value = self.0.into_iter().fold(String::from(""), |mut acc, elem| {
            acc.push_str(&format!("{elem},"));
            acc
        });
        value.pop();
        value
    }
}
