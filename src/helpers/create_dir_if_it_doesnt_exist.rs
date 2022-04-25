use std::fs;
use std::path::Path;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn create_dir_if_it_doesnt_exist(path: &str) -> Result<(), Box<std::io::Error>> {
    if Path::new(path).exists() {
        return Ok(());
    }
    let result_of_creating_directory = fs::create_dir_all(path);
    if let Err(e) = result_of_creating_directory {
        return Err(Box::new(e));
    }
    Ok(())
}
