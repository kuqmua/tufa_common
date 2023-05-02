#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoGetProvidersLinkPartsErrorNamed<'a> {
    MongoDB {
        #[eo_display_foreign_type]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NoSuchCollections {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        no_such_collections: std::collections::HashMap<std::string::String, std::string::String>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    GetDocuments {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        get_documents: std::collections::HashMap<std::string::String, MongoGetDocumentsAsStringVectorErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoGetDocumentsAsStringVectorErrorUnnamed<'a> {
    MongoGetDocumentsAsStringVector(crate::server::mongo::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed<'a>),
}