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
        impl crate::traits::config_fields::GetIsLinksLimitEnabledProviders
        + crate::traits::config_fields::GetPathToProviderLinkPartsFolder
        + crate::traits::config_fields::GetLogFileExtension

        + crate::traits::config_fields::GetIsLinksLimitEnabledArxiv
        + crate::traits::config_fields::GetIsLinksLimitEnabledBiorxiv
        + crate::traits::config_fields::GetIsLinksLimitEnabledGithub
        + crate::traits::config_fields::GetIsLinksLimitEnabledHabr
        + crate::traits::config_fields::GetIsLinksLimitEnabledMedrxiv
        + crate::traits::config_fields::GetIsLinksLimitEnabledReddit
        + crate::traits::config_fields::GetIsLinksLimitEnabledTwitter
        + crate::traits::config_fields::GetLinksLimitArxiv
        + crate::traits::config_fields::GetLinksLimitBiorxiv
        + crate::traits::config_fields::GetLinksLimitGithub
        + crate::traits::config_fields::GetLinksLimitHabr
        + crate::traits::config_fields::GetLinksLimitMedrxiv
        + crate::traits::config_fields::GetLinksLimitReddit
        + crate::traits::config_fields::GetLinksLimitTwitter

        + crate::traits::config_fields::GetIsEnabledArxiv
        + crate::traits::config_fields::GetIsEnabledBiorxiv
        + crate::traits::config_fields::GetIsEnabledGithub
        + crate::traits::config_fields::GetIsEnabledHabr
        + crate::traits::config_fields::GetIsEnabledMedrxiv
        + crate::traits::config_fields::GetIsEnabledReddit
        + crate::traits::config_fields::GetIsEnabledTwitter
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
