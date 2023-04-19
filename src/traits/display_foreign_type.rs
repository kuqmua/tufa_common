pub trait DisplayForeignType {
    fn display_foreign_type(&self) -> &'static str;
}

impl crate::traits::display_foreign_type::DisplayForeignType for sqlx::Error {
    fn display_foreign_type(&self) -> &'static str {
        match self {
            sqlx::Error::Configuration(_) => "sqlx::Error::Configuration",
            sqlx::Error::Database(_) => "sqlx::Error::Database",
            sqlx::Error::Io(_) => "sqlx::Error::Io",
            sqlx::Error::Tls(_) => "sqlx::Error::Tls",
            sqlx::Error::Protocol(_) => "sqlx::Error::Protocol",
            sqlx::Error::RowNotFound => "sqlx::Error::RowNotFound",
            sqlx::Error::TypeNotFound { type_name } => "sqlx::Error::TypeNotFound",
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => "sqlx::Error::ColumnIndexOutOfBounds",
            sqlx::Error::ColumnNotFound(_) => "sqlx::Error::ColumnNotFound",
            sqlx::Error::ColumnDecode { index, source } => "sqlx::Error::ColumnDecode",
            sqlx::Error::Decode(_) => "sqlx::Error::Decode",
            sqlx::Error::PoolTimedOut => "sqlx::Error::PoolTimedOut",
            sqlx::Error::PoolClosed => "sqlx::Error::PoolClosed",
            sqlx::Error::WorkerCrashed => "sqlx::Error::WorkerCrashed",
            sqlx::Error::Migrate(_) => "sqlx::Error::Migrate",
            _ => "unknown sqlx::Error",
        }
    }
}