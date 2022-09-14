use std::io::Write;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn write_bytes_into_file_sync(
    path: &std::path::Path,
    bytes: &[u8],
) -> Result<(), std::io::Error> {
    if let Some(prefix) = path.parent() {
        std::fs::create_dir_all(prefix)?;
    }
    let mut log_file = std::fs::File::create(path)?;
    log_file.write_all(bytes)?;
    log_file.sync_all()?;
    Ok(())
}
