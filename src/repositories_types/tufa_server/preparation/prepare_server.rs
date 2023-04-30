#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PrepareServerErrorNamed<'a> {
    CheckAvailability {
        #[eo_error_occurence]
        check_availability: crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    // InitDbsWrapper {
    //     inner_error: InitDbsWrapperError,
    //     code_occurence: crate::code_occurence::CodeOccurence<'a>,
    // },
}
