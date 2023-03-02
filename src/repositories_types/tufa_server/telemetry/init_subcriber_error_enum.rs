use strum_macros::Display;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::log::SetLoggerError;

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum InitSubcriberErrorEnum<'a> {
    SetLogger {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    SetGlobalDefault {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
