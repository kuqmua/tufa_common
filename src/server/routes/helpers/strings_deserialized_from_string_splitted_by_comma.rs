#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StringsDeserializedFromStringSplittedByComma(
    #[serde(deserialize_with = "deserialize_strings_deserialized_from_string_splitted_by_comma")]
    pub Vec<std::string::String>,
);

fn deserialize_strings_deserialized_from_string_splitted_by_comma<'de, D>(
    deserializer: D,
) -> Result<Vec<std::string::String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok({
        use serde::Deserialize;
        String::deserialize(deserializer)?
    }
    .split(',')
    .map(|element| element.to_string())
    .collect::<Vec<std::string::String>>())
}

impl crate::server::postgres::bind_query::BindQuery
    for StringsDeserializedFromStringSplittedByComma
{
    fn try_generate_bind_increments(&self, increment: &mut u64) -> Result<std::string::String, crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed> {
        let mut increments = std::string::String::from("");
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
        if let false = increments.is_empty() {
            increments.pop();
            increments.pop();
        }
        Ok(increments)
    }
    fn bind_value_to_query(
        self,
        mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.0 {
            query = query.bind(element);
        }
        query
    }
}

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter
    for StringsDeserializedFromStringSplittedByComma
{
    fn serde_urlencoded_parameter(self) -> std::string::String {
        let mut value = self.0.into_iter().fold(String::from(""), |mut acc, elem| {
            acc.push_str(&format!("{elem},"));
            acc
        });
        value.pop();
        value
    }
}
