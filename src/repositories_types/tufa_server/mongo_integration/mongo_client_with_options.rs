#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum MongoClientWithOptionsOriginError<'a> {
    Mongo {
        error: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
