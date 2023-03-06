#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum MongoCheckCollectionIsNotEmptyError<'a> {
    Mongo {
        inner_errors:
            std::collections::HashMap<String, MongoCheckCollectionIsNotEmptyErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum MongoCheckCollectionIsNotEmptyErrorEnum<'a> {
    CountDocumentsOrigin(MongoCheckCollectionIsNotEmptyErrorEnumCountDocuments<'a>),
    IsNotEmptyOrigin(MongoCheckCollectionIsNotEmptyErrorEnumIsNotEmptyOrigin<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum MongoCheckCollectionIsNotEmptyErrorEnumCountDocuments<'a> {
    CountDocuments {
        error: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum MongoCheckCollectionIsNotEmptyErrorEnumIsNotEmptyOrigin<'a> {
    IsNotEmptyOrigin {
        error: u64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
