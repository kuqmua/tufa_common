// use crate::write_error_posts_wrapper::write_error_posts_wrapper;
//     let future_possible_drop_collection = mongo_drop_collection_wrapper(
//         mongo_url,
//         db_name_handle,
//         &format!("{key}{db_collection_handle_second_part}"),
//         false,
//     );
//     match future_possible_drop_collection {
//         Ok(result_flag) => {
//             if result_flag {
//                 println!("drop done!");
//             } else {
//                 println!("drop fail with flag");
//             }
//         }
//         Err(e) => {
//             println!("drop fail with error {e:#?}",);
//         }
//     }

// use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;
// use reqwest::StatusCode;
// #[derive(Debug)]
// pub enum PostErrorVariant {
//     //todo: think about this naming
//     NoItems {
//         link: String,
//         no_items_error: NoItemsError,
//         provider_kind: crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind,
//     },
//     RssFetchAndParseProviderDataError {
//         link: String,
//         provider_kind: crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind,
//         error: RssFetchLinkError,
//     }, //rewrite this error coz it must not be string. dont know to to clone error between threads
// }

// #[derive(Debug)]
// pub enum RssPartError {
//     ReqwestError(reqwest::Error),
//     StatusCode(StatusCode),
// }
//TODO: WRITE CONVERSION FUNCTION INTO COMMON ERROR ENUM AND MOVE IT INTO write_error_posts_wrapper

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetProviderPostsErrorNamed<'a> {
    // GetLocalProvidersLinkParts {
    //     #[eo_error_occurence]
    //     get_providers_link_parts: crate::repositories_types::tufa_server::providers::providers_info::get_providers_link_parts::GetProvidersLinkPartsErrorNamed<'a>,
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    // },
    CheckProvidersLinkPartsEmpty {
        #[eo_error_occurence]
        check_providers_link_parts_empty: crate::repositories_types::tufa_server::providers::check_providers_link_parts_on_empty::CheckProvidersLinkPartsEmptyErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    GetNewProvidersPosts {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        hashmap_provider_kind_rss_part: std::collections::HashMap<std::string::String, GetProviderPostsErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetProviderPostsErrorUnnamed<'a> {
    GetNewProvidersPosts(crate::repositories_types::tufa_server::providers::provider_kind::functions::rss_part::RssPartErrorNamed<'a>),
}

pub async fn get_providers_posts<'a>(
    config: &'a (
        impl crate::traits::config_fields::GetGithubToken
        + crate::traits::config_fields::GetProvidersLinkPartsSource

        + crate::traits::config_fields::GetPathToProviderLinkPartsFolder
        + crate::traits::config_fields::GetLogFileExtension
        + crate::traits::config_fields::GetMongoUrl
        + crate::traits::config_fields::GetMongoProvidersLinkPartsDbName
        + crate::traits::config_fields::GetMongoProvidersLogsDbCollectionDocumentFieldNameHandle
        + crate::traits::config_fields::GetMongoProvidersLogsDbCollectionHandleSecondPart
    )
) -> Result<(), Box<GetProviderPostsErrorNamed<'a>>> {
    todo!()
    // match crate::repositories_types::tufa_server::providers::providers_info::get_providers_link_parts::get_providers_link_parts(config).await {
    //     Err(e) => Err(Box::new(
    //         GetProviderPostsErrorNamed::GetLocalProvidersLinkParts {
    //             get_providers_link_parts: *e,
    //             code_occurence: crate::code_occurence_tufa_common!()
    //         },
    //     )),
    //     Ok(providers_link_parts) => {
    //         match crate::repositories_types::tufa_server::providers::check_providers_link_parts_on_empty::check_providers_link_parts_on_empty(providers_link_parts) {
    //             Err(e) => {
    //                 return Err(Box::new(
    //                     GetProviderPostsErrorNamed::CheckProvidersLinkPartsEmpty {
    //                         check_providers_link_parts_empty: *e,
    //                         code_occurence: crate::code_occurence_tufa_common!()
    //                     },
    //                 ));
    //             }
    //             Ok(non_empty_providers_link_parts) => {
    //                 let hm = crate::repositories_types::tufa_server::check_new_providers_posts::check_new_providers_posts(non_empty_providers_link_parts, config).await;
    //                 let mut error_hashmap = std::collections::HashMap::with_capacity(hm.len());
    //                 let mut success_hashmap = std::collections::HashMap::with_capacity(hm.len());
    //                 for (key, value) in hm {
    //                     match value {
    //                         Err(e) => {
    //                             error_hashmap.insert(key.to_string(), GetProviderPostsErrorUnnamed::GetNewProvidersPosts(e));
    //                         }
    //                         Ok(vec) => {
    //                             success_hashmap.insert(key, vec);
    //                         }
    //                     }
    //                 }
    //                 if !error_hashmap.is_empty() {
    //                     return Err(Box::new(GetProviderPostsErrorNamed::GetNewProvidersPosts {
    //                         hashmap_provider_kind_rss_part: error_hashmap,
    //                         code_occurence: crate::code_occurence_tufa_common!()
    //                     }));
    //                 }
    //                 Ok(())
    //             }
    //         }
    //         // //todo: conversion function before write_error_posts_wrapper
    //         // //commented before conversion function implementation
    //         // // if !vec.is_empty() {
    //         // //     for (pk, result_vec) in vec {
    //         // //         match result_vec {
    //         // //             Ok((vec_common_rss_post_structs, vec_post_error_variants)) => {
    //         // //                 let wrong_cases_thread = thread::spawn(move || {
    //         // //                     block_on(write_error_posts_wrapper(vec_post_error_variants));
    //         // //                 });
    //         // //                 match wrong_cases_thread.join() {
    //         // //                     Ok(_) => {}
    //         // //                     Err(e) => {}
    //         // //                 }
    //         // //             }
    //         // //             Err(e) => {}
    //         // //         }
    //         // //     }
    //         // // }
    //     }
    // }
}
