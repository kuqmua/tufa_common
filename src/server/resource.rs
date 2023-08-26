#[derive(Debug, Clone, PartialEq, Eq, from_str::FromStr)]
pub enum Resource {
    Local,
    Mongodb,
    PostgreSql,
}

impl Default for Resource {
    fn default() -> Self {
        Self::Local
    }
}
