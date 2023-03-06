#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum MongoClientWithOptionsOriginError<'a> {
    Mongo {
        inner_errors: std::collections::HashMap<String, MongoClientWithOptionsOriginErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum MongoClientWithOptionsOriginErrorEnum<'a> {
    CountDocumentsOrigin(MongoClientWithOptionsOriginErrorEnumCountDocuments<'a>),
    IsNotEmptyOrigin(MongoClientWithOptionsOriginErrorEnumIsNotEmptyOrigin<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum MongoClientWithOptionsOriginErrorEnumCountDocuments<'a> {
    CountDocuments {
        error: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum MongoClientWithOptionsOriginErrorEnumIsNotEmptyOrigin<'a> {
    IsNotEmptyOrigin {
        error: u64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
