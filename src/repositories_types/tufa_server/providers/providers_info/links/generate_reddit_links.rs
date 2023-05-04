pub fn generate_reddit_links(subreddits_names: Vec<String>) -> Vec<String> {
    //example https://www.reddit.com/r/3Dprinting/new.json
    subreddits_names
        .iter()
        .map(|name| format!(
            "{}{name}{}",
            crate::global_variables::hardcode::REDDIT_LINK_FIRST_PART,
            crate::global_variables::hardcode::REDDIT_LINK_SECOND_PART
        ))
        .collect()
}
