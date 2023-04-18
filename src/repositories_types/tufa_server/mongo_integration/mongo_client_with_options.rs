#[derive(Debug)]//, thiserror::Error, error_occurence::ErrorOccurence
pub enum MongoClientWithOptionsOriginError<'a> {
    Mongo {
        // #[eo_display_foreign_type]
        error: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
