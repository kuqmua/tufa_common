#[derive(Debug)]
pub struct RemoveDirError {
    pub error: std::io::Error,
}

#[derive(Debug)]
pub enum CleanLogsDirError {
    PathIsNotDir { path: String },
    CannotRemoveDir { error: RemoveDirError },
}
impl From<String> for CleanLogsDirError {
    fn from(e: String) -> Self {
        CleanLogsDirError::PathIsNotDir { path: e }
    }
}
impl From<std::io::Error> for CleanLogsDirError {
    fn from(e: std::io::Error) -> Self {
        CleanLogsDirError::CannotRemoveDir {
            error: RemoveDirError { error: e },
        }
    }
}

#[derive(
    provider_kind_from_config::ProviderKindFromConfig,
    enum_extension::EnumExtension,
    strum_macros::EnumIter,
    Clone,
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
    strum_macros::Display,
)]
#[strum(serialize_all = "snake_case")]
pub enum ProviderKind {
    Arxiv,
    Biorxiv,
    Github,
    Habr,
    Medrxiv,
    Reddit,
    Twitter,
}

//////////////////

use crate::repositories_types::tufa_server::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::repositories_types::tufa_server::fetch::rss_metainfo_fetch_structures::NoItemsError;
use crate::repositories_types::tufa_server::fetch::rss_parse_string_into_struct::rss_parse_string_into_struct;
use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
use futures::future::join_all;
use std::time::Instant;
use crate::common::where_was::WhereWas;
use crate::config_mods::print_type::PrintType;
use crate::server::http_request::http_request_error::HttpRequestOriginError;
use crate::server::http_request::http_request_method::HttpRequestMethod;
use crate::server::http_request::wrappers::text::async_http_request_text::async_http_request_text_wrapper;
use crate::traits::get_color::WarningHighColor;
use crate::traits::get_git_source_file_link::GetGitSourceFileLink;

#[derive(Debug)]
pub enum FetchAndParseProviderDataErrorEnum {
    AsyncFetchLinks {
        source: Vec<(String, Box<HttpRequestOriginError>)>, //link, error
        where_was: WhereWas,
    },
    NoItems {
        source: Vec<(String, NoItemsError)>, //link, error
        where_was: WhereWas,
    },
}

impl ProviderKind {
    pub async fn fetch_and_parse_provider_data(
        self,
        links: Vec<String>,
    ) -> Result<Vec<CommonRssPostStruct>, Box<FetchAndParseProviderDataErrorEnum>> {
        let time = Instant::now();
        let capacity = links.len();
        let vec_to_return = join_all(links.iter().map(|url| async move {
            let result = async_http_request_text_wrapper::<
                String,
                reqwest::cookie::Jar,
                core::time::Duration,
                u32,
                u32,
                u32,
                std::time::Duration,
                std::net::IpAddr,
                std::time::Duration,
                String, //todo - dyn std::any::Any
                String,
                String,
                String,
                String,
                String,
                String,
                String,
                String,
                String,
            >(
                url,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                HttpRequestMethod::Get,
                &CONFIG.source_place_type,
                false,
            )
            .await;
            (url, result)
        }))
        .await;
        let mut half_success_vec = Vec::with_capacity(capacity);
        let mut async_fetch_links_error_vec = Vec::new();
        for (link, result) in vec_to_return {
            match result {
                Err(e) => {
                    async_fetch_links_error_vec.push((link.to_string(), e));
                }
                Ok(str) => {
                    half_success_vec.push((link, str));
                }
            }
        }
        if !async_fetch_links_error_vec.is_empty() {
            //todo: maybe not all links must return Ok ?
            return Err(Box::new(
                FetchAndParseProviderDataErrorEnum::AsyncFetchLinks {
                    source: async_fetch_links_error_vec,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                        git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                    },
                },
            ));
        }
        let mut success_vec = Vec::with_capacity(capacity);
        let mut no_items_error_vec = Vec::new();
        for (link, response_text) in half_success_vec {
            match rss_parse_string_into_struct(response_text, link, self) {
                Err(e) => no_items_error_vec.push((link.to_string(), e)),
                Ok(post_struct) => {
                    success_vec.push(post_struct); //todo maybe add link here?
                }
            }
        }
        if !no_items_error_vec.is_empty() {
            return Err(Box::new(FetchAndParseProviderDataErrorEnum::NoItems {
                source: no_items_error_vec,
                where_was: WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                },
            }));
        }
        Ok(success_vec)
    }
}

////////////////

// use crate::global_variables::runtime::config::CONFIG;
// use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
use mongodb::bson::doc;
use mongodb::bson::Document;

impl ProviderKind {
    pub fn get_mongo_provider_link_parts_aggregation(&self) -> Option<Document> {
        if CONFIG.is_links_limit_enabled_providers
            && self.is_mongo_link_parts_randomize_order_enabled()
        {
            Some(doc! { "$sample" : {"size": CONFIG.links_limit_providers as i64}});
        } else if CONFIG.is_links_limit_enabled_providers {
            Some(doc! { "$limit" :  CONFIG.links_limit_providers as i64});
        } else if self.is_links_limit_enabled()
            && self.is_mongo_link_parts_randomize_order_enabled()
        {
            Some(doc! { "$sample" : {"size": self.links_limit() as i64}});
        } else if self.is_links_limit_enabled() {
            Some(doc! { "$limit" : self.links_limit() as i64});
        } else if self.is_mongo_link_parts_randomize_order_enabled() {
            println!("todo: mongo sample(randomized aggregation) only works if size is valid number. No aggregation applied");
            return None;
        }
        None
    }
}

//////////////////////
// use crate::global_variables::runtime::config::CONFIG;
// use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;

// use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
use crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
use impl_display_for_error::ImplDisplayForError;
use impl_error_with_tracing::ImplErrorWithTracingFromCrate;
use impl_get_source::ImplGetSourceFromCrate;

use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromCrate;
use init_error::InitErrorFromCrate;
use itertools::Itertools;
// use crate::common::where_was::WhereWas;
use crate::traits::get_source::GetSource;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_methods::WhereWasMethods;

#[derive(
    Debug,
    ImplGetWhereWasOriginOrWrapperFromCrate,
    ImplGetSourceFromCrate,
    ImplDisplayForError,
    InitErrorFromCrate,
    ImplErrorWithTracingFromCrate,
)]
pub struct GetLinkPartsFromLocalJsonFileWrapperError {
    source: GetLinkPartsFromLocalJsonFileOriginErrorEnum,
    where_was: WhereWas,
}

#[derive(
    Debug,
    ImplGetSourceFromCrate,
    ImplDisplayForError,
    ImplGetWhereWasOriginOrWrapperFromCrate,
)]
pub enum GetLinkPartsFromLocalJsonFileOriginErrorEnum {
    TokioFsFileOpenOrigin(std::io::Error),
    TokioIoAsyncReadExtReadToEndOrigin(std::io::Error),
    SerdeJsonFromSliceOrigin(serde_json::Error),
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct ProvidersInitJsonSchema {
    pub data: Vec<String>,
}

impl ProviderKind {
    pub async fn get_link_parts_from_local_json_file(
        self,
        should_trace: bool,
    ) -> Result<Vec<String>, Box<GetLinkPartsFromLocalJsonFileWrapperError>> {
        match tokio::fs::File::open(&self.get_init_local_data_file_path()).await {
            Err(e) => Err(Box::new(
                GetLinkPartsFromLocalJsonFileWrapperError::init_error_with_possible_trace(
                    GetLinkPartsFromLocalJsonFileOriginErrorEnum::TokioFsFileOpenOrigin(e),
                    WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                        git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                    }, //
                    &CONFIG.source_place_type,
                    should_trace,
                ),
            )),
            Ok(mut file) => {
                let mut content = Vec::new();
                if let Err(e) = tokio::io::AsyncReadExt::read_to_end(&mut file, &mut content).await
                {
                    return Err(Box::new(
                        GetLinkPartsFromLocalJsonFileWrapperError::init_error_with_possible_trace(
                            GetLinkPartsFromLocalJsonFileOriginErrorEnum::TokioIoAsyncReadExtReadToEndOrigin(e),
                            WhereWas {
                                time: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .expect("cannot convert time to unix_epoch"),
                                file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                        git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                            },
                            &CONFIG.source_place_type,
                            should_trace,
                        ),
                    ));
                }
                match serde_json::from_slice::<ProvidersInitJsonSchema>(&content) {
                    Err(e) => Err(Box::new(
                        GetLinkPartsFromLocalJsonFileWrapperError::init_error_with_possible_trace(
                            GetLinkPartsFromLocalJsonFileOriginErrorEnum::SerdeJsonFromSliceOrigin(
                                e,
                            ),
                            WhereWas {
                                time: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .expect("cannot convert time to unix_epoch"),
                                file: String::from(file!()),
                                line: line!(),
                                column: column!(),
                                git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                            },
                            &CONFIG.source_place_type,
                            should_trace,
                        ),
                    )),
                    Ok(file_content_as_struct) => {
                        let unique_vec: Vec<String> =
                            file_content_as_struct.data.into_iter().unique().collect();
                        let return_vec: Vec<String>;
                        //todo - add correct impl for is_links_limit_enabled - like is_links_limit_enabled_providers && is_links_limit_enabled_arxiv
                        if CONFIG.is_links_limit_enabled_providers && self.is_links_limit_enabled()
                        {
                            let limit = self.links_limit();
                            if unique_vec.len() > limit {
                                return_vec = unique_vec
                                    .into_iter()
                                    .enumerate()
                                    .filter_map(|(index, element)| match index > limit {
                                        false => None,
                                        true => Some(element),
                                    })
                                    .collect::<Vec<String>>();
                            } else {
                                return_vec = unique_vec;
                            }
                        } else {
                            return_vec = unique_vec;
                        }
                        Ok(return_vec)
                    }
                }
            }
        }
    }
}


//////////////////////


// use crate::global_variables::runtime::config::CONFIG;
// use crate::mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector;
// use crate::mongo_integration::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorEnum;

//  use crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
// use mongodb::bson::Document;
use mongodb::options::ClientOptions;
use mongodb::Client;
// use crate::common::where_was::WhereWas;
use crate::traits::get_mongo_url::GetMongoUrl;

#[derive(Debug)]
pub struct MongoGetProviderLinkPartsError {
    pub source: Box<MongoGetProviderLinkPartsErrorEnum>,
}

#[derive(Debug)]
pub enum MongoGetProviderLinkPartsErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    // MongoGetDocumentsAsStringVector {
    //     source: Box<MongoGetDocumentsAsStringVectorErrorEnum>,
    //     where_was: WhereWas,
    // },
}

impl ProviderKind {
    //rust does not support async traits yet (end of 2021). only with third party crate
    pub async fn mongo_get_provider_link_parts(
        pk: crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind,
    ) -> Result<Vec<String>, MongoGetProviderLinkPartsError> {
        todo!()
        // match ClientOptions::parse(CONFIG.get_mongo_url()).await {
        //     Err(e) => Err(MongoGetProviderLinkPartsError {
        //         source: Box::new(MongoGetProviderLinkPartsErrorEnum::ClientOptionsParse {
        //             source: e,
        //             where_was: WhereWas {
        //                 time: std::time::SystemTime::now()
        //                     .duration_since(std::time::UNIX_EPOCH)
        //                     .expect("cannot convert time to unix_epoch"),
        //                 file: String::from(file!()),
        //                 line: line!(),
        //                 column: column!(),
        //                 git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
        //             },
        //         }),
        //     }),
        //     Ok(client_options) => match Client::with_options(client_options) {
        //         Err(e) => Err(MongoGetProviderLinkPartsError {
        //             source: Box::new(MongoGetProviderLinkPartsErrorEnum::ClientWithOptions {
        //                 source: e,
        //                 where_was: WhereWas {
        //                     time: std::time::SystemTime::now()
        //                         .duration_since(std::time::UNIX_EPOCH)
        //                         .expect("cannot convert time to unix_epoch"),
        //                     file: String::from(file!()),
        //                     line: line!(),
        //                     column: column!(),
        //                     git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
        //                 },
        //             }),
        //         }),
        //         Ok(client) => {
        //             match mongo_get_documents_as_string_vector(
        //                 client
        //                     .database(&CONFIG.mongo_providers_logs_db_name)
        //                     .collection::<Document>(&pk.get_mongo_log_collection_name()),
        //                 &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle,
        //                 ProviderKind::get_mongo_provider_link_parts_aggregation(&pk),
        //             )
        //             .await
        //             {
        //                 Err(e) => Err(MongoGetProviderLinkPartsError {
        //                     source: Box::new(
        //                         MongoGetProviderLinkPartsErrorEnum::MongoGetDocumentsAsStringVector {
        //                             source: e,
        //     where_was: WhereWas {
        //         time: std::time::SystemTime::now()
        //             .duration_since(std::time::UNIX_EPOCH)
        //             .expect("cannot convert time to unix_epoch"),
        //         file: String::from(file!()),
        //         line: line!(),
        //         column: column!(),
        //         git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
        //     },
        //                 })}),
        //                 Ok(vec_of_strings) => Ok(vec_of_strings),
        //             }
        //         }
        //     },
        // }
    }
}
