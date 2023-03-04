use strum_macros::Display;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::log::SetLoggerError;

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum InitSubcriberErrorEnum<'a> {
    SetGlobalDefault {
        #[display_is_not_implemented]
        error: SetGlobalDefaultError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    SetLogger {
        #[display_is_not_implemented]
        error: SetLoggerError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl crate::traits::display_foreign_type::DisplayForeignType for SetGlobalDefaultError {
    fn display_foreign_type(&self) -> String {
        String::from("SetGlobalDefaultError")
    }
}

impl crate::traits::display_foreign_type::DisplayForeignType for SetLoggerError {
    fn display_foreign_type(&self) -> String {
        String::from("SetLoggerError")
    }
}
