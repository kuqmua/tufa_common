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
        String::deserialize(deserializer)?
    }
    .split(',')
    .fold(
        (
            Vec::new(),
            std::string::String::from(""),
            std::string::String::from(""),
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

impl crate::common::url_encode::UrlEncode for BigserialIds {
    fn url_encode(&self) -> std::string::String {
        urlencoding::encode(&{
            let mut elements_stringified =
                self.0.iter().fold(String::from(""), |mut acc, element| {
                    acc.push_str(&format!("{element},"));
                    acc
                });
            if let false = elements_stringified.is_empty() {
                elements_stringified.pop();
            }
            elements_stringified
        })
        .to_string()
    }
}

impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery for BigserialIds {
    fn bind_sqlx_query<'q, TableScheme: for<'a> serde::Deserialize<'a>>(
        self,
        mut query: sqlx::query::QueryAs<
            'q,
            sqlx::Postgres,
            TableScheme,
            sqlx::postgres::PgArguments,
        >,
    ) -> sqlx::query::QueryAs<'q, sqlx::Postgres, TableScheme, sqlx::postgres::PgArguments> {
        for element in self.0 {
            query = query.bind(element.into_inner());
        }
        query
    }
}

impl crate::server::routes::helpers::get_inner_length::GetInnerLength for BigserialIds {
    fn get_inner_length(&self) -> usize {
        self.0.len()
    }
}
