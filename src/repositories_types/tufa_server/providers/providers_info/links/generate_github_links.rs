pub fn generate_github_links(
    github_names: Vec<String>,
    config: &impl crate::traits::config_fields::GetGithubToken
) -> Vec<String> {
    //https://github.com/kuqmua.private.atom?token=EXAMPLE_FROM_CONFIG
    github_names
        .iter()
        .map(|name| format!(
            "https://github.com/{name}.private.atom?token={}",
            config.get_github_token()
        ))
        .collect()
}
