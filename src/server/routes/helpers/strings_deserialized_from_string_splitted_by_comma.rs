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
    fn generate_bind_increments(&self, increment: &mut u64) -> std::string::String {
        let mut increments = std::string::String::from("");
        for _ in 0..self.0.len() {
            *increment += 1;
            increments.push_str(&format!("${increment}, "));
        }
        if let false = increments.is_empty() {
            increments.pop();
            increments.pop();
        }
        increments
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
