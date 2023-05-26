#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum GetResponse<'a> {
    Ok(Vec<super::Cat>),
    #[serde(borrow)]
    Err(PostgresSelectCatsError<'a>)
}


#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PostgresSelectCatsErrorNamed<'a> {
    SelectCats {
        #[eo_display]
        select_cats: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub struct PostgresSelectCatsError<'a> {
    #[serde(borrow)]
    pub error: PostgresSelectCatsErrorNamedWithSerializeDeserialize<'a>,
    pub port: crate::common::user_port::UserPort,
    pub pid: u32,
}

impl<'a> std::fmt::Display for PostgresSelectCatsError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
          f, 
          "ERROR: port: {} pid: {}\n{}",
          self.port,
          self.pid,
          crate::common::error_logs_logic::helpers::lines_space_backslash_addition(&self.error)
        )
    }
}