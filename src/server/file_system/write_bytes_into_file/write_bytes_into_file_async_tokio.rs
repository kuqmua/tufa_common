#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum WriteBytesIntoFileAsyncTokioErrorNamed<'a> {
    StdIoError {
        #[eo_display]
        std_io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    }
}

pub async fn write_bytes_into_file_async_tokio<'a>(
    path: &'a std::path::Path,
    bytes: &[u8],
    source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
) -> Result<(), Box<WriteBytesIntoFileAsyncTokioErrorNamed<'a>>> {
    if let Some(prefix) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(prefix) {
            return Err(Box::new(
                WriteBytesIntoFileAsyncTokioErrorNamed::StdIoError {        
                    std_io_error: e,
                    code_occurence: crate::code_occurence_tufa_common!()
                }
            ));
        }
    }
    match tokio::fs::File::open(path).await {
        Err(e) => {
            return Err(Box::new(
                WriteBytesIntoFileAsyncTokioErrorNamed::StdIoError {        
                    std_io_error: e,
                    code_occurence: crate::code_occurence_tufa_common!()
                }
            ));
        }
        Ok(mut file) => {
            if let Err(e) = {
                use tokio::io::AsyncWriteExt;
                file.write_all(bytes)
            }.await {
                return Err(Box::new(
                    WriteBytesIntoFileAsyncTokioErrorNamed::StdIoError {        
                        std_io_error: e,
                        code_occurence: crate::code_occurence_tufa_common!()
                    }
                ));
            }
            Ok(())
        }
    }
}
