// #[tracing::instrument(
//     name = "get_providers_posts_routee",
//     skip_all,
//     fields(user_id=%*user_id)
// )]
pub async fn get_providers_posts_route(//todo - if add here function parameter - actix-web trait will be not satisfied - what to do?
    // config: &'static (
    //     impl crate::traits::fields::GetGithubToken
    //     + crate::traits::fields::GetProvidersLinkPartsSource

    //     + crate::traits::fields::GetCheckLinkArxiv
    //     + crate::traits::fields::GetCheckLinkBiorxiv
    //     + crate::traits::fields::GetCheckLinkGithub
    //     + crate::traits::fields::GetCheckLinkHabr
    //     + crate::traits::fields::GetCheckLinkMedrxiv
    //     + crate::traits::fields::GetCheckLinkReddit
    //     + crate::traits::fields::GetCheckLinkTwitter
        // + std::marker::Send 
        // + std::marker::Sync
    // )
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let time = std::time::Instant::now();
    // if let Err(e) = crate::repositories_types::tufa_server::providers::get_providers_posts::get_providers_posts(config).await {
    //     return Ok(actix_web::HttpResponse::InternalServerError().finish());
    // };
    let message = format!(
        "get_providers_posts done in {} seconds",
        time.elapsed().as_secs()
    );
    Ok(actix_web::HttpResponse::Ok().finish())
}
