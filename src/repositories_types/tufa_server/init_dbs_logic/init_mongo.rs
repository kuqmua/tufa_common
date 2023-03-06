#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum InitMongoWrapperError<'a> {
    Mongo {
        inner_error: InitMongoWrapperErrorEnum<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum InitMongoWrapperErrorEnum<'a> {
    ClientOptionsParseWrapper(crate::repositories_types::tufa_server::mongo_integration::mongo_client_options_parse::MongoClientOptionsParseOriginError<'a>),
    ClientWithOptionsWrapper(crate::repositories_types::tufa_server::mongo_integration::mongo_client_with_options::MongoClientWithOptionsOriginError<'a>),
    CollectionIsNotEmpty(crate::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyError<'a>),
    InsertManyErrorWrapper(crate::repositories_types::tufa_server::mongo_integration::mongo_insert_many::MongoInsertManyOriginError<'a>),
}
