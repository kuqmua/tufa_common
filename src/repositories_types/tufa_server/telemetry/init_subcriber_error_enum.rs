#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum InitSubcriberErrorEnum {
    SetGlobalDefault {
        #[eo_display_foreign_type]
        error: tracing::dispatcher::SetGlobalDefaultError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    SetLogger {
        #[eo_display_foreign_type]
        error: tracing::log::SetLoggerError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
