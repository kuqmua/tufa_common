#[derive(Debug)]//, thiserror::Error, error_occurence::ImplErrorOccurence
pub enum MongoInsertManyOriginError<'a> {
    Mongo {
        inner_errors: std::collections::HashMap<String, MongoInsertManyOriginErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug)]//, thiserror::Error, error_occurence::ImplErrorOccurence
pub enum MongoInsertManyOriginErrorEnum<'a> {
    Mongo(MongoInsertManyOriginErrorEnumError<'a>),
}

#[derive(Debug)]//, thiserror::Error, error_occurence::ImplErrorOccurence
pub enum MongoInsertManyOriginErrorEnumError<'a> {
    Mongo {
        error: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
