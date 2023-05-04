use crate::global_variables::hardcode::GITHUB_LINK_FIRST_PART;
use crate::global_variables::hardcode::GITHUB_LINK_SECOND_PART;

pub fn generate_github_links(github_names: Vec<String>) -> Vec<String> {
    //https://github.com/kuqmua.private.atom?token=EXAMPLE_FROM_CONFIG
    github_names
        .iter()
        .map(|name| {
            format!(
                "{GITHUB_LINK_FIRST_PART}{name}{GITHUB_LINK_SECOND_PART}{}",
                crate::global_variables::runtime::config::CONFIG.github_token
            )
        })
        .collect()
}
