use crate::repositories_types::tufa_server::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::FetchAndParseProviderDataErrorEnum;
use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
use reqwest::StatusCode;
use crate::common::where_was::WhereWas;

#[derive(Debug)]
pub enum RssPartErrorEnum {
    CheckLinkStatusCodeError {
        source: reqwest::Error,
        where_was: WhereWas,
    },
    StatusCode {
        source: StatusCode,
        where_was: WhereWas,
    },
    FetchAndParseProviderData {
        source: FetchAndParseProviderDataErrorEnum,
        where_was: WhereWas,
    },
}

pub async fn rss_part(
    pk: crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind,
    vec_of_provider_links: Vec<String>,
) -> Result<Vec<CommonRssPostStruct>, Box<RssPartErrorEnum>> {
    match reqwest::get(pk.check_link()).await {
        Err(e) => Err(Box::new(RssPartErrorEnum::CheckLinkStatusCodeError {
            source: e,
            where_was: WhereWas {
                time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("cannot convert time to unix_epoch"),
                file: String::from(file!()),
                line: line!(),
                column: column!(),
                git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
            },
        })),
        Ok(res) => {
            let status_code = res.status();
            if !StatusCode::is_success(&status_code) {
                return Err(Box::new(RssPartErrorEnum::StatusCode {
                    source: status_code,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                        git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                    },
                }));
            }
            match crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::fetch_and_parse_provider_data(pk, vec_of_provider_links).await {
                Err(e) => Err(Box::new(RssPartErrorEnum::FetchAndParseProviderData {
                    source: *e,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                        git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                    },
                })),
                Ok(vec) => Ok(vec),
            }
        }
    }
}
