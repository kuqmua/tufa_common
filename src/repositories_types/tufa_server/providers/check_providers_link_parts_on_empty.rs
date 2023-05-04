use std::collections::HashMap;
use crate::common::where_was::WhereWas;

#[derive(Debug)]
pub enum CheckProvidersLinkPartsEmptyError {
    Full {
        where_was: WhereWas,
    },
    Partially {
        source: Vec<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind>,
        where_was: WhereWas,
    },
}

pub fn check_providers_link_parts_on_empty(
    providers_link_parts: HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
) -> Result<HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>, Box<CheckProvidersLinkPartsEmptyError>> {
    if providers_link_parts.is_empty() {
        return Err(Box::new(CheckProvidersLinkPartsEmptyError::Full {
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
    let mut non_empty_providers_link_parts = HashMap::with_capacity(providers_link_parts.len());
    let mut empty_providers_link_parts = HashMap::with_capacity(providers_link_parts.len());
    for (pk, vec) in providers_link_parts {
        if vec.is_empty() {
            empty_providers_link_parts.insert(pk, vec);
        } else {
            non_empty_providers_link_parts.insert(pk, vec);
        }
    }
    if !empty_providers_link_parts.is_empty() {
        let mut pk_vec = Vec::with_capacity(empty_providers_link_parts.len());
        for pk in empty_providers_link_parts.keys() {
            pk_vec.push(*pk);
        }
        return Err(Box::new(CheckProvidersLinkPartsEmptyError::Partially {
            source: pk_vec,
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
    Ok(non_empty_providers_link_parts)
}
