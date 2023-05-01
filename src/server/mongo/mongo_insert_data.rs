#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertDataErrorNamed<'a> {
    Errors {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        errors_hashmap: std::collections::HashMap<std::string::String, MongoInsertDataErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertDataErrorUnnamed<'a> {
    MongoInsertDocsInEmptyCollection(crate::server::mongo::mongo_insert_docs_in_empty_collection::MongoInsertDocsInEmptyCollectionErrorNamed<'a>)
}