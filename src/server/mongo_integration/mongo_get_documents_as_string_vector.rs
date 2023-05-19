#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoGetDocumentsAsStringVectorErrorNamed<'a> {
    MongoDB {
        #[eo_display]
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

pub async fn mongo_get_documents_as_string_vector(
    collection: mongodb::Collection<mongodb::bson::Document>,
    db_collection_document_field_name_handle: &str,
    option_aggregation: Option<mongodb::bson::Document>,
) -> Result<Vec<String>, Box<crate::server::mongo_integration::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed>> {
    match collection.aggregate(option_aggregation, None).await {
        Err(e) => Err(Box::new(
            crate::server::mongo_integration::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: crate::code_occurence_tufa_common!()
            }
        )),
        Ok(mut cursor) => {
            let mut vec_of_strings: Vec<String> = Vec::new();
            loop {
                match {
                    use futures::stream::TryStreamExt;
                    cursor.try_next()
                }.await {
                    Err(e) => {
                        return Err(Box::new(
                            crate::server::mongo_integration::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed::MongoDB {
                                mongodb: e,
                                code_occurence: crate::code_occurence_tufa_common!()
                            }
                        ));
                    }
                    Ok(option_document) => match option_document {
                        None => {
                            break;
                        }
                        Some(document) => {
                            match document.get(db_collection_document_field_name_handle) {
                                None => return Err(Box::new(
                                    crate::server::mongo_integration::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed::NoKeyInDocument {
                                        key: db_collection_document_field_name_handle,
                                        code_occurence: crate::code_occurence_tufa_common!()
                                    }
                                )),
                                Some(bson_handle) => match bson_handle {
                                    mongodb::bson::Bson::String(value) => {
                                        vec_of_strings.push(value.to_string());
                                    }
                                    other_bson_type => {
                                        return Err(Box::new(
                                            crate::server::mongo_integration::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed::WrongBsonType {
                                                bson: other_bson_type.clone(),
                                                code_occurence: crate::code_occurence_tufa_common!()
                                            }
                                    ));
                                    }
                                },
                            }
                        }
                    },
                }
            }
            Ok({
                use itertools::Itertools;
                vec_of_strings.into_iter().unique().collect()
            })
        }
    }
}
