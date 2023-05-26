#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum GetAllResponse<'a> {
    Ok(Vec<super::Cat>),
    #[serde(borrow)]
    Err(PostgresSelectAllCatsErrorNamedWithSerializeDeserialize<'a>)
}


#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresSelectAllCatsErrorNamed<'a> {
    SelectCats {
        #[eo_display]
        select_cats: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}