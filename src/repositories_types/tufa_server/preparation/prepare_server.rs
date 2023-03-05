#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum PrepareServerError<'a> {
    CheckAvailability {
        inner_error: crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    // InitDbsWrapper {
    //     inner_error: InitDbsWrapperError,
    //     code_occurence: crate::code_occurence::CodeOccurence<'a>,
    // },
}
