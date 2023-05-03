#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertManyErrorNamed<'a> {
    InsertMany {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        insert_many: std::collections::HashMap<std::string::String, MongoInsertManyErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertManyErrorUnnamed<'a> {
    InsertMany(MongoInsertManyHandleErrorNamed<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoInsertManyHandleErrorNamed<'a> {
    InsertMany {
        #[eo_display_foreign_type]
        insert_many: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
