use crate::repositories_types::tufa_server::mongo_integration::mongo_get_providers_link_parts::mongo_get_providers_link_parts;
// use crate::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed;

use crate::repositories_types::tufa_server::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts;
use crate::repositories_types::tufa_server::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsWrapperError;
use std::collections::HashMap;
use crate::common::where_was::WhereWas;
use crate::server::resource::Resource;
// use crate::postgres_integration::postgres_get_providers_link_parts::postgres_get_providers_link_parts;
// use crate::postgres_integration::postgres_get_providers_link_parts::PostgresGetProviderLinksError;

#[derive(Debug)]
pub enum GetProvidersLinkPartsErrorEnum<'a> {
    Local {
        source: GetLocalProvidersLinkPartsWrapperError,
        where_was: WhereWas,
    },
    Mongodb {
        source: crate::repositories_types::tufa_server::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed<'a>,
        where_was: WhereWas,
    },
    PostgreSql {
        // source: PostgresGetProviderLinksError,
        // where_was: WhereWas,
    },
}

pub async fn get_providers_link_parts<'a>(
    resource: &Resource,
) -> Result<HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>, Box<GetProvidersLinkPartsErrorEnum<'a>>> {
    todo!()
    // match resource {
    //     Resource::Local => match get_local_providers_link_parts(false).await {
    //         Err(error_hashmap) => Err(Box::new(GetProvidersLinkPartsErrorEnum::Local {
    //             source: *error_hashmap,
    //             where_was: WhereWas {
    //                 time: std::time::SystemTime::now()
    //                     .duration_since(std::time::UNIX_EPOCH)
    //                     .expect("cannot convert time to unix_epoch"),
    //                 file: String::from(file!()),
    //                 line: line!(),
    //                 column: column!(),
    //                 git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
    //             },
    //         })),
    //         Ok(success_hashmap) => Ok(success_hashmap),
    //     },
    //     Resource::Mongodb => match mongo_get_providers_link_parts().await {
    //         Err(e) => Err(Box::new(GetProvidersLinkPartsErrorEnum::Mongodb {
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
    //         })),
    //         Ok(success_hashmap) => Ok(success_hashmap),
    //     },
    //     Resource::PostgreSql => todo!(),
    // }
}
