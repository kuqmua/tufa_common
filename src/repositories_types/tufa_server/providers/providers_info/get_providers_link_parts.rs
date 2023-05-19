// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum GetProvidersLinkPartsErrorNamed<'a> {
//     GetLocalProvidersLinkParts {
//         #[eo_error_occurence]
//         get_local_providers_link_parts: crate::repositories_types::tufa_server::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsErrorNamed<'a>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     MongoGetProvidersLinkParts {
//          #[eo_error_occurence]
//         mongo_get_providers_link_parts: crate::server::mongo::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed<'a>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     // PostgreSql {
//     //     // source: PostgresGetProviderLinksError,
//     //     // code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     // },
// }

// pub async fn get_providers_link_parts<'a>(
//     config: &'static (
//         impl crate::traits::config_fields::GetProvidersLinkPartsSource

//         + crate::traits::config_fields::GetPathToProviderLinkPartsFolder
//         + crate::traits::config_fields::GetLogFileExtension

//         + crate::traits::config_fields::GetMongoUrl
//         + crate::traits::config_fields::GetMongoProvidersLinkPartsDbName
//         + crate::traits::config_fields::GetMongoProvidersLogsDbCollectionDocumentFieldNameHandle
//         + crate::traits::config_fields::GetMongoProvidersLogsDbCollectionHandleSecondPart
        // + std::marker::Send 
        // + std::marker::Sync
//     ) 
// ) -> Result<std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>, Box<GetProvidersLinkPartsErrorNamed<'a>>> {
//     match 
            // config.get_providers_link_parts_source() //postgres mongo local
        // {
//         crate::server::resource::Resource::Local => match crate::repositories_types::tufa_server::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts(config).await {
//             Err(error_hashmap) => Err(Box::new(
//                 GetProvidersLinkPartsErrorNamed::GetLocalProvidersLinkParts {
//                     get_local_providers_link_parts: *error_hashmap,
//                     code_occurence: crate::code_occurence_tufa_common!(),
//                 }
//             )),
//             Ok(success_hashmap) => Ok(success_hashmap),
//         },
//         crate::server::resource::Resource::Mongodb => match crate::server::mongo::mongo_get_providers_link_parts::mongo_get_providers_link_parts(config).await {
//             Err(e) => Err(Box::new(
                
//                 GetProvidersLinkPartsErrorNamed::MongoGetProvidersLinkParts {
//                     mongo_get_providers_link_parts: e,
//                     code_occurence: crate::code_occurence_tufa_common!(),
//                 },
//             )),
//             Ok(success_hashmap) => Ok(success_hashmap),
//         },
//         crate::server::resource::Resource::PostgreSql => todo!(),
//     }
// }
