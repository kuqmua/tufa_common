use std::path::Path;
use tokio::io::AsyncWriteExt;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn write_bytes_into_file_async_tokio(
    path: &Path,
    bytes: &[u8],
) -> Result<(), std::io::Error> {
    if let Some(prefix) = path.parent() {
        std::fs::create_dir_all(prefix)?;
    }
    let mut file = tokio::fs::File::open(path).await?;
    file.write_all(bytes).await?;
    Ok(())
}
