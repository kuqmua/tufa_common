use crate::traits::get_source::GetSource;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_methods::WhereWasMethods;
use impl_display_for_error::ImplDisplayForError;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum NetCheckAvailabilityError<'a> {
    ReqwestGetOrigin {
        #[eo_display_foreign_type]
        error: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ClientOrigin {
        #[eo_display_foreign_type]
        error: reqwest::StatusCode,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ServerOrigin {
        #[eo_display_foreign_type]
        error: reqwest::StatusCode,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn net_check_availability<'a>(
    link: String,
    should_trace: bool,
) -> Result<(), Box<NetCheckAvailabilityError<'a>>> {
    match reqwest::get(link).await {
        Err(e) => Err(Box::new(NetCheckAvailabilityError::ReqwestGetOrigin {
            error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        })),
        Ok(res) => {
            let status = res.status();
            if status.is_client_error() {
                return Err(Box::new(NetCheckAvailabilityError::ClientOrigin {
                    error: status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }));
            }
            if status.is_server_error() {
                return Err(Box::new(NetCheckAvailabilityError::ServerOrigin {
                    error: status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }));
            }
            Ok(())
        }
    }
}
