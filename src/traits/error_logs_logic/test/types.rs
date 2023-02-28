#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum ThreeError<'a> {
    Something {
        //todo how to implement from for it?
        inner_error: crate::traits::error_logs_logic::test::types::FourWrapperError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum FourWrapperError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::collections::HashMap<String, FourErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum FourErrorEnum<'a> {
    Five(FiveError<'a>),
    Six(SixError<'a>),
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum FiveError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::collections::HashMap<String, FiveErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum FiveErrorEnum<'a> {
    FiveOne(FiveOneError<'a>),
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum FiveOneError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum SixError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::vec::Vec<SixErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum SixErrorEnum<'a> {
    Seven(SevenError<'a>),
    Eight(EightError<'a>),
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum SevenError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)] //
pub enum EightError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    SomethingElse {
        inner_error: NineError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ImplErrorOccurence)]
pub enum NineError<'a> {
    NineSomething {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
