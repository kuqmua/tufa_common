#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertManyOriginError<'a> {
    Mongo {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        inner_errors: std::collections::HashMap<std::string::String, MongoInsertManyOriginErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertManyOriginErrorUnnamed<'a> {
    Mongo(MongoInsertManyOriginErrorNamed<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertManyOriginErrorNamed<'a> {
    Mongo {
        #[eo_display_foreign_type]
        error: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
