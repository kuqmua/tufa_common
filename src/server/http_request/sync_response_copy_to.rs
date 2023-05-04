#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum SyncResponseCopyToErrorNamed<'a> {
    CopyTo {
        #[eo_display_foreign_type]
        reqwest_error: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub fn sync_copy_to<'a, W: ?Sized>(
    mut response: reqwest::blocking::Response,
    w: &mut W,
    source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    should_trace: bool,
) -> Result<u64, Box<SyncResponseCopyToErrorNamed<'a>>>
where
    W: std::io::Write,
{
    match response.copy_to(w) {
        Err(e) => Err(Box::new(
            SyncResponseCopyToErrorNamed::CopyTo {
                reqwest_error: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        )),
        Ok(copy_to) => Ok(copy_to),
    }
}
