pub trait DisplayForeignType {
    fn display_foreign_type(&self) -> String;
}

impl crate::traits::display_foreign_type::DisplayForeignType for sqlx::Error {
    fn display_foreign_type(&self) -> String {
        match self {
            sqlx::Error::Configuration(_) => String::from("sqlx::Error::Configuration"),
            sqlx::Error::Database(_) => String::from("sqlx::Error::Database"),
            sqlx::Error::Io(_) => String::from("sqlx::Error::Io"),
            sqlx::Error::Tls(_) => String::from("sqlx::Error::Tls"),
            sqlx::Error::Protocol(_) => String::from("sqlx::Error::Protocol"),
            sqlx::Error::RowNotFound => String::from("sqlx::Error::RowNotFound"),
            sqlx::Error::TypeNotFound { type_name } => String::from("sqlx::Error::TypeNotFound"),
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => String::from("sqlx::Error::ColumnIndexOutOfBounds"),
            sqlx::Error::ColumnNotFound(_) => String::from("sqlx::Error::ColumnNotFound"),
            sqlx::Error::ColumnDecode { index, source } => String::from("sqlx::Error::ColumnDecode"),
            sqlx::Error::Decode(_) => String::from("sqlx::Error::Decode"),
            sqlx::Error::PoolTimedOut => String::from("sqlx::Error::PoolTimedOut"),
            sqlx::Error::PoolClosed => String::from("sqlx::Error::PoolClosed"),
            sqlx::Error::WorkerCrashed => String::from("sqlx::Error::WorkerCrashed"),
            sqlx::Error::Migrate(_) => String::from("sqlx::Error::Migrate"),
            _ => String::from("unknown sqlx::Error"),
        }
    }
}

impl crate::traits::display_foreign_type::DisplayForeignType
    for tracing::dispatcher::SetGlobalDefaultError
{
    fn display_foreign_type(&self) -> String {
        String::from("SetGlobalDefaultError")
    }
}

impl crate::traits::display_foreign_type::DisplayForeignType for tracing::log::SetLoggerError {
    fn display_foreign_type(&self) -> String {
        String::from("SetLoggerError")
    }
}

// impl crate::traits::display_foreign_type::DisplayForeignType for mongodb::error::Error {
//     fn display_foreign_type(&self) -> String {
//         match self.kind {
//             mongodb::error::ErrorKind::InvalidArgument { message } => todo!(),
//             mongodb::error::ErrorKind::Authentication { message } => todo!(),
//             mongodb::error::ErrorKind::BsonDeserialization(_) => todo!(),
//             mongodb::error::ErrorKind::BsonSerialization(_) => todo!(),
//             mongodb::error::ErrorKind::BulkWrite(_) => todo!(),
//             mongodb::error::ErrorKind::Command(_) => todo!(),
//             mongodb::error::ErrorKind::DnsResolve { message } => todo!(),
//             mongodb::error::ErrorKind::Internal { message } => todo!(),
//             mongodb::error::ErrorKind::Io(_) => todo!(),
//             mongodb::error::ErrorKind::ConnectionPoolCleared { message } => todo!(),
//             mongodb::error::ErrorKind::InvalidResponse { message } => todo!(),
//             mongodb::error::ErrorKind::ServerSelection { message } => todo!(),
//             mongodb::error::ErrorKind::SessionsNotSupported => todo!(),
//             mongodb::error::ErrorKind::InvalidTlsConfig { message } => todo!(),
//             mongodb::error::ErrorKind::Write(_) => todo!(),
//             mongodb::error::ErrorKind::Transaction { message } => todo!(),
//             mongodb::error::ErrorKind::IncompatibleServer { message } => todo!(),
//             mongodb::error::ErrorKind::MissingResumeToken => todo!(),
//             _ => todo!(),
//         }
//     }
// }
