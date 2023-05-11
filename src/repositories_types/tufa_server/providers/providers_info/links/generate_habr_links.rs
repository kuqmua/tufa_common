pub fn generate_habr_links(habr_names: Vec<String>) -> Vec<String> {
    //example https://habr.com/ru/rss/all/all/?fl=ru?with_hubs=true:?with_tags=true:
    habr_names
        .iter()
        .map(|name| format!("https://habr.com/ru/rss/{name}"))
        .collect()
}
