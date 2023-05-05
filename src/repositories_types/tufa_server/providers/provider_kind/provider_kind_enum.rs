use crate::traits::get_source::GetSource;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_methods::WhereWasMethods;
use crate::global_variables::runtime::config::CONFIG;

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

#[derive(Debug)]
pub enum FetchAndParseProviderDataErrorEnum {
    // AsyncFetchLinks {
    //     source: Vec<(String, Box<crate::server::http_request::http_request_error::HttpRequestErrorNamed<'a>>)>, //link, error
    //     where_was: crate::common::where_was::WhereWas,
    // },
    NoItems {
        source: Vec<(String, crate::repositories_types::tufa_server::fetch::rss_metainfo_fetch_structures::NoItemsError)>, //link, error
        where_was: crate::common::where_was::WhereWas,
    },
}

// impl ProviderKind {
//     pub async fn fetch_and_parse_provider_data(
//         self,
//         links: Vec<String>,
//     ) -> Result<Vec<crate::repositories_types::tufa_server::fetch::info_structures::common_rss_structures::CommonRssPostStruct>, Box<FetchAndParseProviderDataErrorEnum>> {
//         let time = std::time::Instant::now();
//         let capacity = links.len();
//         let vec_to_return = futures::future::join_all(links.iter().map(|url| async move {
//             let result = crate::server::http_request::wrappers::text::async_http_request_text::async_http_request_text_wrapper::<
//                 String,
//                 reqwest::cookie::Jar,
//                 core::time::Duration,
//                 u32,
//                 u32,
//                 u32,
//                 std::time::Duration,
//                 std::net::IpAddr,
//                 std::time::Duration,
//                 String, //todo - dyn std::any::Any
//                 String,
//                 String,
//                 String,
//                 String,
//                 String,
//                 String,
//                 String,
//                 String,
//                 String,
//             >(
//                 url,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 crate::server::http_request::http_request_method::HttpRequestMethod::Get,
//                 &crate::global_variables::runtime::config::CONFIG.source_place_type,
//                 false,
//             )
//             .await;
//             (url, result)
//         }))
//         .await;
//         let mut half_success_vec = Vec::with_capacity(capacity);
//         let mut async_fetch_links_error_vec = Vec::new();
//         for (link, result) in vec_to_return {
//             match result {
//                 Err(e) => {
//                     async_fetch_links_error_vec.push((link.to_string(), e));
//                 }
//                 Ok(str) => {
//                     half_success_vec.push((link, str));
//                 }
//             }
//         }
//         if !async_fetch_links_error_vec.is_empty() {
//             //todo: maybe not all links must return Ok ?
//             return Err(Box::new(
//                 FetchAndParseProviderDataErrorEnum::AsyncFetchLinks {
//                     source: async_fetch_links_error_vec,
//                     where_was: crate::common::where_was::WhereWas {
//                         time: std::time::SystemTime::now()
//                             .duration_since(std::time::UNIX_EPOCH)
//                             .expect("cannot convert time to unix_epoch"),
//                         file: String::from(file!()),
//                         line: line!(),
//                         column: column!(),
//                         git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
//                     },
//                 },
//             ));
//         }
//         let mut success_vec = Vec::with_capacity(capacity);
//         let mut no_items_error_vec = Vec::new();
//         for (link, response_text) in half_success_vec {
//             match crate::repositories_types::tufa_server::fetch::rss_parse_string_into_struct::rss_parse_string_into_struct(response_text, link, self) {
//                 Err(e) => no_items_error_vec.push((link.to_string(), e)),
//                 Ok(post_struct) => {
//                     success_vec.push(post_struct); //todo maybe add link here?
//                 }
//             }
//         }
//         if !no_items_error_vec.is_empty() {
//             return Err(Box::new(FetchAndParseProviderDataErrorEnum::NoItems {
//                 source: no_items_error_vec,
//                 where_was: crate::common::where_was::WhereWas {
//                     time: std::time::SystemTime::now()
//                         .duration_since(std::time::UNIX_EPOCH)
//                         .expect("cannot convert time to unix_epoch"),
//                     file: String::from(file!()),
//                     line: line!(),
//                     column: column!(),
//                     git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
//                 },
//             }));
//         }
//         Ok(success_vec)
//     }
// }

impl ProviderKind {
    pub fn get_mongo_provider_link_parts_aggregation(&self) -> Option<mongodb::bson::Document> {
        if crate::global_variables::runtime::config::CONFIG.is_links_limit_enabled_providers
            && self.is_mongo_link_parts_randomize_order_enabled()
        {
            Some(mongodb::bson::doc! { "$sample" : {"size": crate::global_variables::runtime::config::CONFIG.links_limit_providers as i64}});
        } else if crate::global_variables::runtime::config::CONFIG.is_links_limit_enabled_providers {
            Some(mongodb::bson::doc! { "$limit" :  crate::global_variables::runtime::config::CONFIG.links_limit_providers as i64});
        } else if self.is_links_limit_enabled()
            && self.is_mongo_link_parts_randomize_order_enabled()
        {
            Some(mongodb::bson::doc! { "$sample" : {"size": self.links_limit() as i64}});
        } else if self.is_links_limit_enabled() {
            Some(mongodb::bson::doc! { "$limit" : self.links_limit() as i64});
        } else if self.is_mongo_link_parts_randomize_order_enabled() {
            println!("todo: mongo sample(randomized aggregation) only works if size is valid number. No aggregation applied");
            return None;
        }
        None
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetLinkPartsFromLocalJsonFileErrorNamed<'a> {
    TokioFsFileOpen {
        #[eo_display]
        tokio_fs_file_open: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    TokioIoAsyncReadExtReadToEnd {
        #[eo_display]
        tokio_io_async_read_ext_read_to_end: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    SerdeJsonFromSlice {
        #[eo_display]
        serde_json_from_slice: serde_json::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct ProvidersInitJsonSchema {
    pub data: Vec<String>,
}

impl ProviderKind {
    pub async fn get_link_parts_from_local_json_file<'a>(self) -> Result<Vec<String>, Box<GetLinkPartsFromLocalJsonFileErrorNamed<'a>>> {
        match tokio::fs::File::open(&{
            use crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
            self.get_init_local_data_file_path()
        }).await {
            Err(e) => Err(Box::new(
                GetLinkPartsFromLocalJsonFileErrorNamed::TokioFsFileOpen {
                    tokio_fs_file_open: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }
            )),
            Ok(mut file) => {
                let mut content = Vec::new();
                if let Err(e) = tokio::io::AsyncReadExt::read_to_end(&mut file, &mut content).await
                {
                    return Err(Box::new(
                        GetLinkPartsFromLocalJsonFileErrorNamed::TokioIoAsyncReadExtReadToEnd {
                            tokio_io_async_read_ext_read_to_end: e,
                            code_occurence: crate::code_occurence_tufa_common!()
                        }
                    ));
                }
                match serde_json::from_slice::<ProvidersInitJsonSchema>(&content) {
                    Err(e) => Err(Box::new(
                        GetLinkPartsFromLocalJsonFileErrorNamed::SerdeJsonFromSlice {
                            serde_json_from_slice: e,
                            code_occurence: crate::code_occurence_tufa_common!()
                        }
                    )),
                    Ok(file_content_as_struct) => {
                        let unique_vec: Vec<String> =
                            {
                                use itertools::Itertools;
                                file_content_as_struct.data.into_iter().unique()
                            }.collect();
                        let return_vec: Vec<String>;
                        //todo - add correct impl for is_links_limit_enabled - like is_links_limit_enabled_providers && is_links_limit_enabled_arxiv
                        if crate::global_variables::runtime::config::CONFIG.is_links_limit_enabled_providers && self.is_links_limit_enabled()
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

#[derive(Debug)]
pub struct MongoGetProviderLinkPartsError {
    pub source: Box<MongoGetProviderLinkPartsErrorEnum>,
}

#[derive(Debug)]
pub enum MongoGetProviderLinkPartsErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: crate::common::where_was::WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
        where_was: crate::common::where_was::WhereWas,
    },
    // MongoGetDocumentsAsStringVector {
    //     source: Box<MongoGetDocumentsAsStringVectorErrorEnum>,
    //     where_was: crate::common::where_was::WhereWas,
    // },
}

impl ProviderKind {
    //rust does not support async traits yet (end of 2021). only with third party crate
    pub async fn mongo_get_provider_link_parts(
        pk: crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind,
    ) -> Result<Vec<String>, MongoGetProviderLinkPartsError> {
        todo!()
        // match mongodb::options::ClientOptions::parse(crate::global_variables::runtime::config::CONFIG.get_mongo_url()).await {
        //     Err(e) => Err(MongoGetProviderLinkPartsError {
        //         source: Box::new(MongoGetProviderLinkPartsErrorEnum::ClientOptionsParse {
        //             source: e,
        //             where_was: crate::common::where_was::WhereWas {
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
        //     Ok(client_options) => match mongodb::Client::with_options(client_options) {
        //         Err(e) => Err(MongoGetProviderLinkPartsError {
        //             source: Box::new(MongoGetProviderLinkPartsErrorEnum::ClientWithOptions {
        //                 source: e,
        //                 where_was: crate::common::where_was::WhereWas {
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
        //                     .database(&crate::global_variables::runtime::config::CONFIG.mongo_providers_logs_db_name)
        //                     .collection::<mongodb::bson::Document>(&pk.get_mongo_log_collection_name()),
        //                 &crate::global_variables::runtime::config::CONFIG.mongo_providers_logs_db_collection_document_field_name_handle,
        //                 ProviderKind::get_mongo_provider_link_parts_aggregation(&pk),
        //             )
        //             .await
        //             {
        //                 Err(e) => Err(MongoGetProviderLinkPartsError {
        //                     source: Box::new(
        //                         MongoGetProviderLinkPartsErrorEnum::MongoGetDocumentsAsStringVector {
        //                             source: e,
        //     where_was: crate::common::where_was::WhereWas {
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
