#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoDropEmptyDbErrorNamed<'a> {
    MongoDB {
        #[eo_display_foreign_type]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    CollectionNamesListIsNotEmpty {
        #[eo_display_with_serialize_deserialize]
        database: &'a str,
        #[eo_display_with_serialize_deserialize]
        list_collection_names_len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}