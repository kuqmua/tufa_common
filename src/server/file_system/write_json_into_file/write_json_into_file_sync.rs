#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum WriteJsonIntoFileSyncErrorNamed<'a> {
    SerdeJson{
        #[eo_display]
        serde_json: serde_json::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    WriteBytesIntoFile {
        #[eo_error_occurence]
        write_bytes_into_file: crate::server::file_system::write_bytes_into_file::write_bytes_into_file_sync::WriteBytesIntoFileSyncErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub fn write_json_into_file_async(
    path: &std::path::Path,
    json_object: serde_json::Value,
) -> Result<(), Box<WriteJsonIntoFileSyncErrorNamed<'_>>> {
    match serde_json::to_string_pretty(&json_object) {
        Err(e) => Err(Box::new(WriteJsonIntoFileSyncErrorNamed::SerdeJson {
            serde_json: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        })),
        Ok(stringified_json) => {
            if let Err(e) = crate::server::file_system::write_bytes_into_file::write_bytes_into_file_sync::write_bytes_into_file_sync(
                path,
                stringified_json,
            ) {
                return Err(
                    Box::new(
                        WriteJsonIntoFileSyncErrorNamed::WriteBytesIntoFile{
                            write_bytes_into_file: *e,
                            code_occurence: crate::code_occurence_tufa_common!()
                        }
                    )
                );
            }
            Ok(())
        }
    }
}
