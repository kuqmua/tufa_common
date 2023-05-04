use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
// use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::GetLinkPartsFromLocalJsonFileErrorNamed;

use crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
use futures::future::join_all;
use impl_error_with_tracing::ImplErrorWithTracingFromCrate;
use impl_get_source::ImplGetSourceFromCrate;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromCrate;
use init_error::InitErrorFromCrate;
use std::collections::HashMap;
use crate::common::where_was::WhereWas;
use crate::traits::get_log_where_was::GetLogWhereWas;
use crate::traits::get_source::GetSource;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_methods::WhereWasMethods;
use valuable::Valuable;

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetLocalProvidersLinkPartsErrorNamed<'a> {
    GetLinkPartsFromLocalJsonFile {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        get_link_parts_from_localJson_file: HashMap<std::string::String, GetLocalProvidersLinkPartsErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetLocalProvidersLinkPartsErrorUnnamed<'a> {
    GetLinkPartsFromLocalJsonFile(crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::GetLinkPartsFromLocalJsonFileErrorNamed<'a>),
}

#[derive(Clone, Debug, Valuable)]
pub struct TracingVec {
    pub vec: Vec<String>,
}

pub async fn get_local_providers_link_parts<'a>(
    should_trace: bool,
) -> Result<HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>, Box<GetLocalProvidersLinkPartsErrorNamed<'a>>> {
    let result_vec = join_all(
        crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::get_enabled_providers_vec() //maybe its not exactly correct
            .into_iter()
            .map(|pk| async move {
                (
                    pk,
                    crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::get_link_parts_from_local_json_file(pk).await,
                )
            }),
    )
    .await;
    let mut errors_hashmap: HashMap<std::string::String, GetLocalProvidersLinkPartsErrorUnnamed<'a>> =
        HashMap::new();
    let mut success_hashmap: HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>> =
        HashMap::with_capacity(result_vec.len());
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
                get_link_parts_from_localJson_file: errors_hashmap,
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        ));
    }
    Ok(success_hashmap)
}
