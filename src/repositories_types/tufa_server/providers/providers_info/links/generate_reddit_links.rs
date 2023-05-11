pub fn generate_reddit_links(subreddits_names: Vec<String>) -> Vec<String> {
    //example https://www.reddit.com/r/3Dprinting/new.json
    subreddits_names
        .iter()
        .map(|name| format!("https://www.reddit.com/r/{name}/new.json"))
        .collect()
}
