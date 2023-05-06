#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetLocalProvidersLinkPartsErrorNamed<'a> {
    GetLinkPartsFromLocalJsonFile {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        get_link_parts_from_local_json_file: std::collections::HashMap<std::string::String, GetLocalProvidersLinkPartsErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetLocalProvidersLinkPartsErrorUnnamed<'a> {
    GetLinkPartsFromLocalJsonFile(crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::GetLinkPartsFromLocalJsonFileErrorNamed<'a>),
}

#[derive(Clone, Debug, valuable::Valuable)]
pub struct TracingVec {
    pub vec: Vec<String>,
}

pub async fn get_local_providers_link_parts<'a>(
    config: &'a (
        impl crate::traits::fields::GetIsLinksLimitEnabledProviders
        + crate::traits::fields::GetPathToProviderLinkPartsFolder
        + crate::traits::fields::GetLogFileExtension

        + crate::traits::fields::GetIsLinksLimitEnabledArxiv
        + crate::traits::fields::GetIsLinksLimitEnabledBiorxiv
        + crate::traits::fields::GetIsLinksLimitEnabledGithub
        + crate::traits::fields::GetIsLinksLimitEnabledHabr
        + crate::traits::fields::GetIsLinksLimitEnabledMedrxiv
        + crate::traits::fields::GetIsLinksLimitEnabledReddit
        + crate::traits::fields::GetIsLinksLimitEnabledTwitter
        + crate::traits::fields::GetLinksLimitArxiv
        + crate::traits::fields::GetLinksLimitBiorxiv
        + crate::traits::fields::GetLinksLimitGithub
        + crate::traits::fields::GetLinksLimitHabr
        + crate::traits::fields::GetLinksLimitMedrxiv
        + crate::traits::fields::GetLinksLimitReddit
        + crate::traits::fields::GetLinksLimitTwitter

        + crate::traits::fields::GetIsEnabledArxiv
        + crate::traits::fields::GetIsEnabledBiorxiv
        + crate::traits::fields::GetIsEnabledGithub
        + crate::traits::fields::GetIsEnabledHabr
        + crate::traits::fields::GetIsEnabledMedrxiv
        + crate::traits::fields::GetIsEnabledReddit
        + crate::traits::fields::GetIsEnabledTwitter
    )
) -> Result<std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>, Box<GetLocalProvidersLinkPartsErrorNamed<'a>>> {
    let result_vec = futures::future::join_all(
            {
                use crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
                crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::get_enabled_providers_vec(config) //maybe its not exactly correct
            }
            .into_iter()
            .map(|pk| async move {
                (
                    pk,
                    crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::get_link_parts_from_local_json_file(
                        pk,
                        config
                    ).await,
                )
            }),
    )
    .await;
    let mut errors_hashmap: std::collections::HashMap<std::string::String, GetLocalProvidersLinkPartsErrorUnnamed<'a>> =
        std::collections::HashMap::new();
    let mut success_hashmap: std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>> =
        std::collections::HashMap::with_capacity(result_vec.len());
    for (pk, result) in result_vec {
        match result {
            Err(e) => {
                errors_hashmap.insert(pk.to_string(), GetLocalProvidersLinkPartsErrorUnnamed::GetLinkPartsFromLocalJsonFile(*e));
            }
            Ok(vec) => {
                success_hashmap.insert(pk, vec);
            }
        }
    }
    if !errors_hashmap.is_empty() {
        return Err(Box::new(
            GetLocalProvidersLinkPartsErrorNamed::GetLinkPartsFromLocalJsonFile {
                get_link_parts_from_local_json_file: errors_hashmap,
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        ));
    }
    Ok(success_hashmap)
}
