#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoDropEmptyCollectionErrorNamed<'a> {
    MongoDB {
        #[eo_display_foreign_type]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CollectionIsNotEmpty {
        #[eo_display_with_serialize_deserialize]
        collection_name: std::string::String,
        #[eo_display_with_serialize_deserialize]
        collection_len: u64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}