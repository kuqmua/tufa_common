use crate::repositories_types::tufa_server::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::repositories_types::tufa_server::providers::provider_kind::functions::rss_part::rss_part;
use crate::repositories_types::tufa_server::providers::provider_kind::functions::rss_part::RssPartErrorNamed;
use crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
use futures::future::join_all;
use std::collections::HashMap;

pub async fn check_new_providers_posts<'a>(
    providers_link_parts: HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
) -> HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Result<Vec<CommonRssPostStruct>, RssPartErrorNamed<'a>>> {
    let tasks_vec = providers_link_parts
        .into_iter()
        .map(|(pk, link_parts)| async move {
            match rss_part(pk, pk.generate_provider_links(link_parts)).await {
                Ok(posts_vec) => (pk, Ok(posts_vec)),
                Err(e) => (pk, Err(*e)),
            }
        });
    let posts_and_errors_to_return = join_all(tasks_vec)
        .await
        .into_iter()
        .map(|(pk, s)| (pk, s))
        .collect();
    posts_and_errors_to_return
}
