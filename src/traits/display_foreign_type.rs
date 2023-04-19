pub trait DisplayForeignType {
    fn display_foreign_type(&self) -> String;
}

impl crate::traits::display_foreign_type::DisplayForeignType for sqlx::Error {
    fn display_foreign_type(&self) -> String {
        match self {
            sqlx::Error::Configuration(e) => format!("sqlx::Error::Configuration({e})"),
            sqlx::Error::Database(e) => format!("sqlx::Error::Database({e})"),
            sqlx::Error::Io(e) => format!("sqlx::Error::Io({e})"),
            sqlx::Error::Tls(e) => format!("sqlx::Error::Tls({e})"),
            sqlx::Error::Protocol(e) => format!("sqlx::Error::Protocol({e})"),
            sqlx::Error::RowNotFound => format!("sqlx::Error::RowNotFound"),
            sqlx::Error::TypeNotFound { type_name } => format!("sqlx::Error::TypeNotFound{{type_name: {type_name}}}"),
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => format!("sqlx::Error::ColumnIndexOutOfBounds{{index: {index}, len: {len}}}"),
            sqlx::Error::ColumnNotFound(e) => format!("sqlx::Error::ColumnNotFound({e})"),
            sqlx::Error::ColumnDecode { index, source } => format!("sqlx::Error::ColumnDecode{{index: {index}, source: {source}}}"),
            sqlx::Error::Decode(e) => format!("sqlx::Error::Decode({e})"),
            sqlx::Error::PoolTimedOut => format!("sqlx::Error::PoolTimedOut"),
            sqlx::Error::PoolClosed => format!("sqlx::Error::PoolClosed"),
            sqlx::Error::WorkerCrashed => format!("sqlx::Error::WorkerCrashed"),
            sqlx::Error::Migrate(e) => format!("sqlx::Error::Migrate({e})"),
            i => format!("sqlx::Error (default case) {i}"),
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
