#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ErrorOccurence)]
pub enum ThreeError<'a> {
    Something {
        #[eo_error_occurence]
        inner_error: crate::traits::error_logs_logic::test::types::FourWrapperError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ErrorOccurence)]
pub enum FourWrapperError<'a> {
    Something {
        #[eo_hashmap_key_display_value_error_occurence]
        inner_errors: std::collections::HashMap<String, FourErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ErrorOccurence)]
pub enum FourErrorEnum<'a> {
    Five(FiveError<'a>),
    Six(SixError<'a>),
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ErrorOccurence)]
pub enum FiveError<'a> {
    Something {
        #[eo_hashmap_key_display_value_error_occurence]
        inner_errors: std::collections::HashMap<String, FiveErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ErrorOccurence)]
pub enum FiveErrorEnum<'a> {
    FiveOne(FiveOneError<'a>),
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ErrorOccurence)]
pub enum FiveOneError<'a> {
    Something {
        #[eo_display]
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ErrorOccurence)]
pub enum SixError<'a> {
    Something {
        #[eo_vec_error_occurence]
        inner_errors: std::vec::Vec<SixErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ErrorOccurence)]
pub enum SixErrorEnum<'a> {
    Seven(SevenError<'a>),
    Eight(EightError<'a>),
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ErrorOccurence)]
pub enum SevenError<'a> {
    Something {
        #[eo_display]
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ErrorOccurence)]
pub enum EightError<'a> {
    Something {
        #[eo_display]
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    SomethingElse {
        #[eo_error_occurence]
        inner_error: NineError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, serde::Serialize, error_occurence::ErrorOccurence)]
pub enum NineError<'a> {
    NineSomething {
        #[eo_display]
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
