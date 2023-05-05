#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum CheckProvidersLinkPartsEmptyErrorNamed<'a> {
    Full {
        #[eo_display_with_serialize_deserialize]
        message: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Partially {
        #[eo_vec_display_with_serialize_deserialize]
        provider_kind_vec: Vec<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub fn check_providers_link_parts_on_empty<'a>(
    providers_link_parts: std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
) -> Result<std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>, Box<CheckProvidersLinkPartsEmptyErrorNamed<'a>>> {
    if providers_link_parts.is_empty() {
        return Err(Box::new(
            CheckProvidersLinkPartsEmptyErrorNamed::Full {
                message: "providers link parts is empty",
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        ));
    }
    let mut non_empty_providers_link_parts = std::collections::HashMap::with_capacity(providers_link_parts.len());
    let mut empty_providers_link_parts = std::collections::HashMap::with_capacity(providers_link_parts.len());
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
        return Err(Box::new(
            CheckProvidersLinkPartsEmptyErrorNamed::Partially {
                provider_kind_vec: pk_vec,
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        ));
    }
    Ok(non_empty_providers_link_parts)
}
