#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoGetDocumentsAsStringVectorErrorNamed<'a> {
    MongoDB {
        #[eo_display_foreign_type]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    WrongBsonType {
        #[eo_display_with_serialize_deserialize]
        bson: mongodb::bson::Bson,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NoKeyInDocument {
        #[eo_display_with_serialize_deserialize]
        key: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}