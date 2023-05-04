pub fn generate_arxiv_links(arxiv_names: Vec<String>) -> Vec<String> {
    //example http://export.arxiv.org/rss/astro-ph.CO
    arxiv_names
        .iter()
        .map(|name| {
            format!(
                "{}{name}",
                crate::global_variables::hardcode::ARXIV_LINK_FIRST_PART
            )
        })
        .collect()
}
