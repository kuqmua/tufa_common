#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Cat {
  pub id: i64,
  pub name: String,
  pub color: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum GetAllResponse<'a> {
    Ok(Vec<Cat>),
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

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum GetResponse<'a> {
    Ok(Vec<Cat>),
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