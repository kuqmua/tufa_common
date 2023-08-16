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

impl crate::common::url_encode::UrlEncode for StringsDeserializedFromStringSplittedByComma {
    fn url_encode(&self) -> std::string::String {
        urlencoding::encode(&{
            let mut elements_stringified =
                self.0
                    .iter()
                    .fold(std::string::String::from(""), |mut acc, element| {
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

impl crate::server::routes::helpers::bind_sqlx_query::BindSqlxQuery
    for StringsDeserializedFromStringSplittedByComma
{
    fn bind_sqlx_query(
        self,
        mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.0 {
            query = query.bind(element);
        }
        query
    }
}

impl crate::server::routes::helpers::get_inner_length::GetInnerLength
    for StringsDeserializedFromStringSplittedByComma
{
    fn get_inner_length(&self) -> usize {
        self.0.len()
    }
}
