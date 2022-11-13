use crate::common::where_was::WhereWas;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::traits::get_source::GetSource;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_trait::WhereWasTrait;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromCrate;
use impl_get_source_with_method::ImplGetSourceWithMethodFromCrate;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromCrate;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromCrate;
use init_error::InitErrorFromCrate;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::options::ClientOptions;
use mongodb::Client;

#[derive(
    Debug,
    ImplGetSourceWithMethodFromCrate,
    ImplDisplayForErrorStruct,
    InitErrorFromCrate,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromCrate,
    ImplGetWhereWasOneOrManyOneForErrorStructFromCrate,
)]
pub struct MongoInsertDocsInEmptyCollectionError {
    source: MongoInsertDocsInEmptyCollectionErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithoutMethodFromCrate, ImplDisplayForSimpleErrorEnum)]
pub enum MongoInsertDocsInEmptyCollectionErrorEnum {
    ClientOptionsParse(mongodb::error::Error),
    ClientWithOptions(mongodb::error::Error),
    CountDocuments(mongodb::error::Error),
    NotEmpty(u64),
    CollectionInsertMany(mongodb::error::Error),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_insert_docs_in_empty_collection(
    mongo_url: String,
    db_name_handle: &str,
    db_collection_handle: String,
    collection_field_name: String,
    vec_of_values: Vec<String>,
    source_place_type: &SourcePlaceType,
    should_trace: bool,
) -> Result<(), Box<MongoInsertDocsInEmptyCollectionError>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(
            MongoInsertDocsInEmptyCollectionError::init_error_with_possible_trace(
                MongoInsertDocsInEmptyCollectionErrorEnum::ClientOptionsParse(e),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
                source_place_type,
                &GIT_INFO,
                should_trace,
            ),
        )),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(Box::new(
                MongoInsertDocsInEmptyCollectionError::init_error_with_possible_trace(
                    MongoInsertDocsInEmptyCollectionErrorEnum::ClientWithOptions(e),
                    WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                    source_place_type,
                    &GIT_INFO,
                    should_trace,
                ),
            )),
            Ok(client) => {
                let collection = client
                    .database(db_name_handle)
                    .collection(&db_collection_handle);
                match collection.count_documents(None, None).await {
                    Err(e) => Err(Box::new(
                        MongoInsertDocsInEmptyCollectionError::init_error_with_possible_trace(
                            MongoInsertDocsInEmptyCollectionErrorEnum::CountDocuments(e),
                            WhereWas {
                                time: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .expect("cannot convert time to unix_epoch"),
                                location: *core::panic::Location::caller(),
                            },
                            source_place_type,
                            &GIT_INFO,
                            should_trace,
                        ),
                    )),
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            Err(Box::new(
                                MongoInsertDocsInEmptyCollectionError::init_error_with_possible_trace(
                                    MongoInsertDocsInEmptyCollectionErrorEnum::NotEmpty(documents_number),
                                    WhereWas {
                                        time: std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .expect("cannot convert time to unix_epoch"),
                                        location: *core::panic::Location::caller(),
                                    },
                                    source_place_type,
                                    &GIT_INFO,
                                    should_trace,
                                ),
                            ))
                        } else {
                            if let Err(e) = collection
                                .insert_many(
                                    vec_of_values
                                        .iter()
                                        .map(|value| doc! { &collection_field_name: value })
                                        .collect::<Vec<Document>>(),
                                    None,
                                )
                                .await
                            {
                                return Err(Box::new(
                                    MongoInsertDocsInEmptyCollectionError::init_error_with_possible_trace(
                                        MongoInsertDocsInEmptyCollectionErrorEnum::CollectionInsertMany(e),
                                        WhereWas {
                                            time: std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .expect("cannot convert time to unix_epoch"),
                                            location: *core::panic::Location::caller(),
                                        },
                                        source_place_type,
                                        &GIT_INFO,
                                        should_trace,
                                    ),
                                ));
                            }
                            Ok(())
                        }
                    }
                }
            }
        },
    }
}
