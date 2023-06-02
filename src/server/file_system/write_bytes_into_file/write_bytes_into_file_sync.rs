#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum WriteBytesIntoFileSyncErrorNamed<'a> {
    StdIo {
        #[eo_display]
        std_io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub fn write_bytes_into_file_sync(
    path: &std::path::Path,
    bytes: std::string::String,
) -> Result<(), Box<WriteBytesIntoFileSyncErrorNamed>> {
    if let Some(prefix) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(prefix) {
            return Err(Box::new(WriteBytesIntoFileSyncErrorNamed::StdIo {
                std_io_error: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }));
        }
    }
    match std::fs::File::create(path) {
        Err(e) => Err(Box::new(WriteBytesIntoFileSyncErrorNamed::StdIo {
            std_io_error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        })),
        Ok(mut file) => {
            if let Err(e) = {
                use std::io::Write;
                file.write_all(bytes.as_bytes())
            } {
                return Err(Box::new(WriteBytesIntoFileSyncErrorNamed::StdIo {
                    std_io_error: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }));
            }
            if let Err(e) = file.sync_all() {
                return Err(Box::new(WriteBytesIntoFileSyncErrorNamed::StdIo {
                    std_io_error: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }));
            }
            Ok(())
        }
    }
}
