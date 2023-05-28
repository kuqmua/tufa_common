pub static CRATES_IO_SLASH_CRATES_SLASH_LINK: &'static str = "https://crates.io/crates/";
pub static NON_EXHAUSTIVE: &'static str = "#[non_exhaustive]";

pub trait DisplayForeignType {
    fn display_foreign_type(&self) -> String;
}

// impl crate::common::display_foreign_type::DisplayForeignType for sqlx::Error {
//     fn display_foreign_type(&self) -> String {
//         format!("{self}")
//         // let link_to_crate = format!("{CRATES_IO_SLASH_CRATES_SLASH_LINK}sqlx");
//         // match self {
//         //     sqlx::Error::Configuration(e) => format!("sqlx::Error::Configuration({e}) {link_to_crate}"),
//         //     sqlx::Error::Database(e) => format!("sqlx::Error::Database({e}) {link_to_crate}"),
//         //     sqlx::Error::Io(e) => format!("sqlx::Error::Io({e}) {link_to_crate}"),
//         //     sqlx::Error::Tls(e) => format!("sqlx::Error::Tls({e}) {link_to_crate}"),
//         //     sqlx::Error::Protocol(e) => format!("sqlx::Error::Protocol({e}) {link_to_crate}"),
//         //     sqlx::Error::RowNotFound => format!("sqlx::Error::RowNotFound {link_to_crate}"),
//         //     sqlx::Error::TypeNotFound { type_name } => format!("sqlx::Error::TypeNotFound{{type_name: {type_name}}} {link_to_crate}"),
//         //     sqlx::Error::ColumnIndexOutOfBounds { index, len } => format!("sqlx::Error::ColumnIndexOutOfBounds{{index: {index}, len: {len}}} {link_to_crate}"),
//         //     sqlx::Error::ColumnNotFound(e) => format!("sqlx::Error::ColumnNotFound({e}) {link_to_crate}"),
//         //     sqlx::Error::ColumnDecode { index, source } => format!("sqlx::Error::ColumnDecode{{index: {index}, source: {source}}} {link_to_crate}"),
//         //     sqlx::Error::Decode(e) => format!("sqlx::Error::Decode({e}) {link_to_crate}"),
//         //     sqlx::Error::PoolTimedOut => format!("sqlx::Error::PoolTimedOut {link_to_crate}"),
//         //     sqlx::Error::PoolClosed => format!("sqlx::Error::PoolClosed {link_to_crate}"),
//         //     sqlx::Error::WorkerCrashed => format!("sqlx::Error::WorkerCrashed {link_to_crate}"),
//         //     sqlx::Error::Migrate(e) => format!("sqlx::Error::Migrate({e}) {link_to_crate}"),
//         //     e => format!("sqlx::Error {NON_EXHAUSTIVE} {e} {link_to_crate}"),
//         // }
//     }
// }

impl crate::common::display_foreign_type::DisplayForeignType
    for tracing::dispatcher::SetGlobalDefaultError
{
    fn display_foreign_type(&self) -> String {
        String::from("tracing::dispatcher::SetGlobalDefaultError")
    }
}

impl crate::common::display_foreign_type::DisplayForeignType for tracing::log::SetLoggerError {
    fn display_foreign_type(&self) -> String {
        String::from("tracing::log::SetLoggerError")
    }
}

// impl crate::common::display_foreign_type::DisplayForeignType for mongodb::error::Error {
//     fn display_foreign_type(&self) -> String {
//         format!("{self}")
//         // let link_to_crate = format!("{CRATES_IO_SLASH_CRATES_SLASH_LINK}mongodb");
//         // match *self.kind.clone() {
//         //     mongodb::error::ErrorKind::InvalidArgument { message, .. } => format!("mongodb::error::ErrorKind::InvalidArgument{{message: {message}}} {link_to_crate}"),
//         //     mongodb::error::ErrorKind::Authentication { message, .. } => format!("mongodb::error::ErrorKind::Authentication{{message: {message}}} {link_to_crate}"),
//         //     mongodb::error::ErrorKind::BsonDeserialization(e) => format!("mongodb::error::ErrorKind::BsonDeserialization({e}) {link_to_crate}"),
//         //     mongodb::error::ErrorKind::BsonSerialization(e) => format!("mongodb::error::ErrorKind::BsonSerialization({e}) {link_to_crate}"),
//         //     mongodb::error::ErrorKind::BulkWrite(e) => format!("mongodb::error::ErrorKind::BulkWrite(todo - error too big to show) {link_to_crate}"),//todo this error is too big. maybe
//         //     mongodb::error::ErrorKind::Command(e) => format!("mongodb::error::ErrorKind::Command({e}) {link_to_crate}"),
//         //     mongodb::error::ErrorKind::DnsResolve { message, .. } => format!("mongodb::error::ErrorKind::DnsResolve{{message: {message}}} {link_to_crate}"),
//         //     mongodb::error::ErrorKind::Internal { message, .. } => format!("mongodb::error::ErrorKind::Internal{{message: {message}}} {link_to_crate}"),
//         //     mongodb::error::ErrorKind::Io(e) => format!("mongodb::error::ErrorKind::Io({e}) {link_to_crate}"),
//         //     mongodb::error::ErrorKind::ConnectionPoolCleared { message, .. } => format!("mongodb::error::ErrorKind::ConnectionPoolCleared{{message: {message}}} {link_to_crate}"),
//         //     mongodb::error::ErrorKind::InvalidResponse { message, .. } => format!("mongodb::error::ErrorKind::InvalidResponse{{message: {message}}} {link_to_crate}"),
//         //     mongodb::error::ErrorKind::ServerSelection { message, .. } => format!("mongodb::error::ErrorKind::ServerSelection{{message: {message}}} {link_to_crate}"),
//         //     mongodb::error::ErrorKind::SessionsNotSupported => format!("mongodb::error::ErrorKind::SessionsNotSupported {link_to_crate}"),
//         //     mongodb::error::ErrorKind::InvalidTlsConfig { message, .. } => format!("mongodb::error::ErrorKind::InvalidTlsConfig{{message: {message}}} {link_to_crate}"),
//         //     mongodb::error::ErrorKind::Write(e) => {
//         //         match e {
//         //             mongodb::error::WriteFailure::WriteConcernError(e) => format!("mongodb::error::ErrorKind::Write(mongodb::error::WriteFailure::WriteConcernError({})) {link_to_crate}", e.code_name),//todo - not all fields
//         //             mongodb::error::WriteFailure::WriteError(e) => todo!("mongodb::error::ErrorKind::Write(mongodb::error::WriteFailure({})) {link_to_crate}", e.code),//todo - not all fields
//         //             e => format!("mongodb::error::ErrorKind::Write(mongodb::error::WriteFailure {NON_EXHAUSTIVE}) {link_to_crate}"),
//         //         }
//         //     },
//         //     mongodb::error::ErrorKind::Transaction { message, .. } => format!("mongodb::error::ErrorKind::Transaction{{message: {message}}} {link_to_crate}"),
//         //     mongodb::error::ErrorKind::IncompatibleServer { message, .. } => format!("mongodb::error::ErrorKind::IncompatibleServer{{message: {message}}} {link_to_crate}"),
//         //     mongodb::error::ErrorKind::MissingResumeToken => format!("mongodb::error::ErrorKind::MissingResumeToken {link_to_crate}"),
//         //     e => format!("mongodb::error::ErrorKind {NON_EXHAUSTIVE} {e} {link_to_crate}"),
//         // }
//     }
// }

impl crate::common::display_foreign_type::DisplayForeignType for reqwest::Error {
    fn display_foreign_type(&self) -> String {
        format!("{self}")
    }
}

impl crate::common::display_foreign_type::DisplayForeignType for reqwest::StatusCode {
    fn display_foreign_type(&self) -> String {
        format!("{self}")
    }
}
