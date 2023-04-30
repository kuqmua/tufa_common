#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckCollectionIsNotEmptyErrorNamed<'a> {
    Mongo {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        inner_errors:
            std::collections::HashMap<std::string::String, MongoCheckCollectionIsNotEmptyErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckCollectionIsNotEmptyErrorUnnamed<'a> {
    CountDocumentsOrigin(MongoCheckCollectionIsNotEmptyErrorCountDocumentsErrorNamed<'a>),
    IsNotEmptyOrigin(MongoCheckCollectionIsNotEmptyErrorIsNotEmptyOriginErrorNamed<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckCollectionIsNotEmptyErrorCountDocumentsErrorNamed<'a> {
    CountDocuments {
        #[eo_display_foreign_type]
        error: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoCheckCollectionIsNotEmptyErrorIsNotEmptyOriginErrorNamed<'a> {
    IsNotEmptyOrigin {
        #[eo_display_with_serialize_deserialize]
        error: u64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
