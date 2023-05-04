use crate::global_variables::hardcode::TWITTER_LINK_FIRST_PART;
use crate::global_variables::hardcode::TWITTER_LINK_SECOND_PART;
use crate::global_variables::hardcode::TWITTER_LINK_THIRD_PART;
use crate::repositories_types::tufa_server::providers::providers_info::get_twitter_provider_name::get_twitter_provider_name;

pub fn generate_twitter_links(twitter_subs_names: Vec<String>) -> Vec<String> {
    //example https://nitter.pussthecat.org/Tom_McGurl/rss
    twitter_subs_names
        .iter()
        .map(|name| {
            format!(
                "{TWITTER_LINK_FIRST_PART}{}{TWITTER_LINK_SECOND_PART}{name}{TWITTER_LINK_THIRD_PART}",
                get_twitter_provider_name()
            )
        })
        .collect()
}
