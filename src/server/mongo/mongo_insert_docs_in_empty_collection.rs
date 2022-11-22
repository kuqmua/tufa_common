use crate::common::where_was::WhereWas;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::traits::get_source::GetSource;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_trait::WhereWasTrait;
use impl_display_for_error::ImplDisplayForError;
use impl_error_with_tracing::ImplErrorWithTracingFromCrate;
use impl_get_source::ImplGetSourceFromCrate;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromCrate;
use init_error::InitErrorFromCrate;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::options::ClientOptions;
use mongodb::Client;

#[derive(
    Debug,
    ImplGetSourceFromCrate,
    ImplDisplayForError,
    InitErrorFromCrate,
    ImplErrorWithTracingFromCrate,
    ImplGetWhereWasOriginOrWrapperFromCrate,
)]
pub struct MongoInsertDocsInEmptyCollectionWrapperError {
    source: MongoInsertDocsInEmptyCollectionOriginErrorEnum,
    where_was: WhereWas,
}

#[derive(
    Debug, ImplGetSourceFromCrate, ImplDisplayForError, ImplGetWhereWasOriginOrWrapperFromCrate,
)]
pub enum MongoInsertDocsInEmptyCollectionOriginErrorEnum {
    ClientWithOptionsOrigin(mongodb::error::Error),
    CountDocumentsOrigin(mongodb::error::Error),
    NotEmptyOrigin(u64),
    CollectionInsertManyOrigin(mongodb::error::Error),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_insert_docs_in_empty_collection(
    client_options: ClientOptions,
    db_name_handle: &str,
    db_collection_handle: String,
    collection_field_name: String,
    vec_of_values: Vec<String>,
    source_place_type: &SourcePlaceType,
    should_trace: bool,
) -> Result<(), Box<MongoInsertDocsInEmptyCollectionWrapperError>> {
    match Client::with_options(client_options) {
        Err(e) => Err(Box::new(
            MongoInsertDocsInEmptyCollectionWrapperError::init_error_with_possible_trace(
                MongoInsertDocsInEmptyCollectionOriginErrorEnum::ClientWithOptionsOrigin(e),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: crate::common::where_was::GitInfoForWhereWas {
                        commit_id: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.commit_id,
                        ),
                        repo_link: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.repo_link,
                        ),
                        author: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author,
                        ),
                        author_email: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author_email,
                        ),
                        commit_unix_time: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO
                                .commit_unix_time,
                        ),
                        timezone: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                        ),
                        message: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.message,
                        ),
                    },
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
                    MongoInsertDocsInEmptyCollectionWrapperError::init_error_with_possible_trace(
                        MongoInsertDocsInEmptyCollectionOriginErrorEnum::CountDocumentsOrigin(e),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            file: String::from(file!()),
                            line: line!(),
                            column: column!(),
                            git_info: crate::common::where_was::GitInfoForWhereWas {
                                commit_id: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .commit_id,
                                ),
                                repo_link: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .repo_link,
                                ),
                                author: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .author,
                                ),
                                author_email: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .author_email,
                                ),
                                commit_unix_time: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .commit_unix_time,
                                ),
                                timezone: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .timezone,
                                ),
                                message: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .message,
                                ),
                            },
                        },
                        source_place_type,
                        &GIT_INFO,
                        should_trace,
                    ),
                )),
                Ok(documents_number) => {
                    if documents_number > 0 {
                        Err(Box::new(
                            MongoInsertDocsInEmptyCollectionWrapperError::init_error_with_possible_trace(
                                MongoInsertDocsInEmptyCollectionOriginErrorEnum::NotEmptyOrigin(
                                    documents_number,
                                ),
                                WhereWas {
                                    time: std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .expect("cannot convert time to unix_epoch"),
                                    file: String::from(file!()),
                                    line: line!(),
                                    column: column!(),
                                    git_info: crate::common::where_was::GitInfoForWhereWas {
                        commit_id: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.commit_id,
                        ),
                        repo_link: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.repo_link,
                        ),
                        author: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author,
                        ),
                        author_email: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author_email,
                        ),
                        commit_unix_time: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO
                                .commit_unix_time,
                        ),
                        timezone: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                        ),
                        message: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.message,
                        ),
                    },
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
                                    MongoInsertDocsInEmptyCollectionWrapperError::init_error_with_possible_trace(
                                        MongoInsertDocsInEmptyCollectionOriginErrorEnum::CollectionInsertManyOrigin(e),
                                        WhereWas {
                                            time: std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .expect("cannot convert time to unix_epoch"),
                                            file: String::from(file!()),
                                            line: line!(),
                                            column: column!(),
                                            git_info: crate::common::where_was::GitInfoForWhereWas {
                        commit_id: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.commit_id,
                        ),
                        repo_link: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.repo_link,
                        ),
                        author: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author,
                        ),
                        author_email: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author_email,
                        ),
                        commit_unix_time: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO
                                .commit_unix_time,
                        ),
                        timezone: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                        ),
                        message: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.message,
                        ),
                    },
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
    }
}
