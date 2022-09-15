use crate::helpers::write_bytes_into_file_async_tokio::write_bytes_into_file_async_tokio;
use serde_json::Value;
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum WriteJsonIntoFileError {
    #[error("serde_json::to_string_pretty serde_json::Error error: `{0}`.")]
    SerdeJsonError(
        #[from]
        #[source]
        serde_json::Error,
    ),
    #[error("write_string_into_file_with_tokio std::io::Error error: `{0}`.")]
    StdIoError(
        #[from]
        #[source]
        std::io::Error,
    ),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn write_json_into_file(
    path: &Path,
    json_object: Value,
) -> Result<(), WriteJsonIntoFileError> {
    let stringified_json = serde_json::to_string_pretty(&json_object)?;
    Ok(write_bytes_into_file_async_tokio(path, stringified_json.as_bytes()).await?)
}
