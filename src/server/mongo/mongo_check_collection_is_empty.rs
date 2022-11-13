use crate::common::where_was::WhereWas;
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
pub struct MongoCheckCollectionIsEmptyError {
    source: MongoCheckCollectionIsEmptyErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithoutMethodFromCrate, ImplDisplayForSimpleErrorEnum)]
pub enum MongoCheckCollectionIsEmptyErrorEnum {
    ClientOptionsParse(mongodb::error::Error),
    ClientWithOptions(mongodb::error::Error),
    CountDocuments(mongodb::error::Error),
    NotEmpty(u64),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_check_collection_is_empty(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
    should_trace: bool,
) -> Result<(), Box<MongoCheckCollectionIsEmptyError>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(
            MongoCheckCollectionIsEmptyError::init_error_with_possible_trace(
                MongoCheckCollectionIsEmptyErrorEnum::ClientOptionsParse(e),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO,
                should_trace,
            ),
        )),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(Box::new(
                MongoCheckCollectionIsEmptyError::init_error_with_possible_trace(
                    MongoCheckCollectionIsEmptyErrorEnum::ClientWithOptions(e),
                    WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                    &CONFIG.source_place_type,
                    &GIT_INFO,
                    should_trace,
                ),
            )),
            Ok(client) => {
                match client
                    .database(db_name)
                    .collection::<Document>(db_collection_name)
                    .count_documents(None, None)
                    .await
                {
                    Err(e) => Err(Box::new(
                        MongoCheckCollectionIsEmptyError::init_error_with_possible_trace(
                            MongoCheckCollectionIsEmptyErrorEnum::CountDocuments(e),
                            WhereWas {
                                time: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .expect("cannot convert time to unix_epoch"),
                                location: *core::panic::Location::caller(),
                            },
                            &CONFIG.source_place_type,
                            &GIT_INFO,
                            should_trace,
                        ),
                    )),
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            return Err(Box::new(
                                MongoCheckCollectionIsEmptyError::init_error_with_possible_trace(
                                    MongoCheckCollectionIsEmptyErrorEnum::NotEmpty(
                                        documents_number,
                                    ),
                                    WhereWas {
                                        time: std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .expect("cannot convert time to unix_epoch"),
                                        location: *core::panic::Location::caller(),
                                    },
                                    &CONFIG.source_place_type,
                                    &GIT_INFO,
                                    should_trace,
                                ),
                            ));
                        }
                        Ok(())
                    }
                }
            }
        },
    }
}
