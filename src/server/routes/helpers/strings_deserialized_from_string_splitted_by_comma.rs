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
