#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum GetResponse<'a> {
    Ok(Vec<super::Cat>),
    #[serde(borrow)]
    Err(PostgresSelectCatsErrorNamedWithSerializeDeserialize<'a>)
}


#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresSelectCatsErrorNamed<'a> {
    SelectCats {
        #[eo_display]
        select_cats: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}