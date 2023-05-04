use crate::global_variables::hardcode::HABR_LINK_FIRST_PART;

pub fn generate_habr_links(habr_names: Vec<String>) -> Vec<String> {
    //example https://habr.com/ru/rss/all/all/?fl=ru?with_hubs=true:?with_tags=true:
    habr_names
        .iter()
        .map(|name| format!("{HABR_LINK_FIRST_PART}{name}"))
        .collect()
}
