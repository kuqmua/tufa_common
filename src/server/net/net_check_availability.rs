#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum NetCheckAvailabilityErrorNamed<'a> {
    ReqwestGet {
        #[eo_display_foreign_type]
        reqwest_get: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ResponseStatus {
        #[eo_display_foreign_type]
        status: reqwest::StatusCode,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    }
}

pub async fn net_check_availability<'a>(link: &String) -> Result<(), Box<NetCheckAvailabilityErrorNamed<'a>>> {
    match reqwest::get(link).await {
        Err(e) => Err(Box::new(NetCheckAvailabilityErrorNamed::ReqwestGet {
            reqwest_get: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        })),
        Ok(res) => {
            let status = res.status();
            match (status.is_client_error(), status.is_server_error()) {
                (true, true) => Err(Box::new(NetCheckAvailabilityErrorNamed::ResponseStatus {
                    status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                })),
                (true, false) => Err(Box::new(NetCheckAvailabilityErrorNamed::ResponseStatus {
                    status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                })),
                (false, true) => Err(Box::new(NetCheckAvailabilityErrorNamed::ResponseStatus {
                    status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                })),
                (false, false) => Ok(()),
            }
        }
    }
}
