#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum RssPartErrorNamed<'a> {
    CheckLinkStatusCodeError {
        #[eo_display_foreign_type]
        reqwest_error: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    StatusCode {
        #[eo_display_foreign_type]
        status_code: reqwest::StatusCode,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    // FetchAndParseProviderData {
    //     source: crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::FetchAndParseProviderDataErrorEnum,
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    // },
}

pub async fn rss_part<'a>(
    pk: crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind,
    vec_of_provider_links: Vec<String>,
) -> Result<Vec<crate::repositories_types::tufa_server::fetch::info_structures::common_rss_structures::CommonRssPostStruct>, Box<RssPartErrorNamed<'a>>> {
    match reqwest::get({
        use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
        pk.check_link()
    }).await {
        Err(e) => Err(Box::new(RssPartErrorNamed::CheckLinkStatusCodeError {
            reqwest_error: e,
            code_occurence: crate::code_occurence_tufa_common!()
        })),
        Ok(res) => {
            let status_code = res.status();
            if !reqwest::StatusCode::is_success(&status_code) {
                return Err(Box::new(RssPartErrorNamed::StatusCode {
                    status_code,
                    code_occurence: crate::code_occurence_tufa_common!()
                }));
            }
            // match crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::fetch_and_parse_provider_data(pk, vec_of_provider_links).await {
            //     Err(e) => Err(Box::new(RssPartErrorNamed::FetchAndParseProviderData {
            //         source: *e,
            //         code_occurence: crate::code_occurence_tufa_common!()
            //     })),
            //     Ok(vec) => Ok(vec),
            // }
            todo!()
        }
    }
}
