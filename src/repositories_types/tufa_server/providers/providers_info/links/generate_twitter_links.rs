pub fn generate_twitter_links(twitter_subs_names: Vec<String>) -> Vec<String> {
    //example https://nitter.pussthecat.org/Tom_McGurl/rss
    twitter_subs_names
        .iter()
        .map(|name| format!("https://{}/{name}/rss", crate::repositories_types::tufa_server::providers::providers_info::get_twitter_provider_name::get_twitter_provider_name()))
        .collect()
}
