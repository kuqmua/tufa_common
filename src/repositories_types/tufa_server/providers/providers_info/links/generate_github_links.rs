pub fn generate_github_links(
    github_names: Vec<String>,
    config: &impl crate::traits::fields::GetGithubToken
) -> Vec<String> {
    //https://github.com/kuqmua.private.atom?token=EXAMPLE_FROM_CONFIG
    github_names
        .iter()
        .map(|name| {
            format!(
                "{}{name}{}{}",
                crate::global_variables::hardcode::GITHUB_LINK_FIRST_PART,
                crate::global_variables::hardcode::GITHUB_LINK_SECOND_PART,
                config.get_github_token()
            )
        })
        .collect()
}
