#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PrepareServerError<'a> {
    CheckAvailability {
        #[eo_error_occurence]
        inner_error: crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    // InitDbsWrapper {
    //     inner_error: InitDbsWrapperError,
    //     code_occurence: crate::code_occurence::CodeOccurence<'a>,
    // },
}
