use strum_macros::Display;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::log::SetLoggerError;

#[derive(Debug, thiserror::Error, serde::Serialize)] //, error_occurence::ImplErrorOccurence
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

impl<'a> std::fmt::Display for InitSubcriberErrorEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "kwko")
    }
}

// todo - implement conversion to serialize\Deserialize version

#[derive(Debug)]
pub struct Kekw {}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)] //, serde::Serialize, error_occurence::ImplErrorOccurence
pub enum Seven<'a> {
    Something {
        #[display_is_not_implemented]
        error: Kekw,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl crate::traits::display_foreign_type::DisplayForeignType for Kekw {
    fn display_foreign_type(&self) -> String {
        String::from("kekw")
    }
}
