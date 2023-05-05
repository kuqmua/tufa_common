use crate::repositories_types::tufa_server::mongo_integration::mongo_get_providers_link_parts::mongo_get_providers_link_parts;
// use crate::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed;

use crate::repositories_types::tufa_server::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts;
use std::collections::HashMap;
use crate::common::where_was::WhereWas;
use crate::server::resource::Resource;
// use crate::postgres_integration::postgres_get_providers_link_parts::postgres_get_providers_link_parts;
// use crate::postgres_integration::postgres_get_providers_link_parts::PostgresGetProviderLinksError;

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetProvidersLinkPartsErrorNamed<'a> {
    GetLocalProvidersLinkParts {
        #[eo_error_occurence]
        get_local_providers_link_parts: crate::repositories_types::tufa_server::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    MongoGetProvidersLinkParts {
         #[eo_error_occurence]
        mongo_get_providers_link_parts: crate::repositories_types::tufa_server::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    // PostgreSql {
    //     // source: PostgresGetProviderLinksError,
    //     // code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    // },
}

pub async fn get_providers_link_parts<'a>(
    resource: &Resource,
) -> Result<HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>, Box<GetProvidersLinkPartsErrorNamed<'a>>> {
    todo!()
    // match resource {
    //     Resource::Local => match get_local_providers_link_parts(false).await {
    //         Err(error_hashmap) => Err(Box::new(
    //             GetProvidersLinkPartsErrorNamed::GetLocalProvidersLinkParts {
    //                 get_local_providers_link_parts: *error_hashmap,
    //                 code_occurence: crate::code_occurence_tufa_common!(),
    //             }
    //         )),
    //         Ok(success_hashmap) => Ok(success_hashmap),
    //     },
    //     Resource::Mongodb => match mongo_get_providers_link_parts().await {
    //         Err(e) => Err(Box::new(
                
    //             GetProvidersLinkPartsErrorNamed::MongoGetProvidersLinkParts {
    //                 mongo_get_providers_link_parts: e,
    //                 code_occurence: crate::code_occurence_tufa_common!(),
    //             },
    //         )),
    //         Ok(success_hashmap) => Ok(success_hashmap),
    //     },
    //     Resource::PostgreSql => todo!(),
    // }
}
