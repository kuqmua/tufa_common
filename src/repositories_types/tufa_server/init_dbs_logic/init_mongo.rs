#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum InitMongoErrorNamed<'a> {
    ClientOptionsParse {
        #[eo_error_occurence]
        client_options_parse: crate::repositories_types::tufa_server::mongo_integration::mongo_client_options_parse::MongoClientOptionsParseOriginErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ClientWithOptions {
        #[eo_error_occurence]
        client_with_options: crate::repositories_types::tufa_server::mongo_integration::mongo_client_with_options::MongoClientWithOptionsOriginErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CollectionIsNotEmpty {
        #[eo_error_occurence]
        collection_is_not_empty: crate::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    InsertManyError {
        #[eo_error_occurence]
        insert_many: crate::repositories_types::tufa_server::mongo_integration::mongo_insert_many::MongoInsertManyOriginErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    }
}
