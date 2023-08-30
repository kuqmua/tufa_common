#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ConjunctiveOperator {
    Or,
    And,
}

impl std::fmt::Display for ConjunctiveOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConjunctiveOperator::Or => write!(f, "{}", crate::server::postgres::constants::OR_NAME),
            ConjunctiveOperator::And => {
                write!(f, "{}", crate::server::postgres::constants::AND_NAME)
            }
        }
    }
}
