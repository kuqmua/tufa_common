#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Cat {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CatToInsert {
    pub name: String,
    pub color: String,
}

#[derive(serde::Deserialize)]
pub struct SelectQueryParameters {
    pub limit: Option<crate::server::postgres::rows_per_table::RowsPerTable>,
    pub name: Option<String>,
    pub color: Option<String>,
}

pub static DEFAULT_SELECT_ALL_LIMIT: u32 = 10;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum GetAllResponse<'a> {
    Ok(Vec<Cat>),
    #[serde(borrow)]
    Err(PostgresSelectCatsErrorNamedWithSerializeDeserialize<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresSelectCatsErrorNamed<'a> {
    SelectCats {
        #[eo_display]
        select_cats: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(serde::Deserialize)]
pub struct SelectByIdPathParameters {
    pub id: i64,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresSelectErrorNamed<'a> {
    SelectCat {
        #[eo_display]
        select_cat: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum SelectByIdResponse<'a> {
    #[serde(borrow)]
    BigserialError(
        crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamedWithSerializeDeserialize<
            'a,
        >,
    ),
    Ok(Cat),
    #[serde(borrow)]
    Select(PostgresSelectCatErrorNamedWithSerializeDeserialize<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresSelectCatErrorNamed<'a> {
    SelectCat {
        #[eo_display]
        select_cat: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresInsertCatErrorNamed<'a> {
    InsertCat {
        #[eo_display]
        select_cat: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
