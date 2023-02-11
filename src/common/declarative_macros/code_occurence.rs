#[macro_export]
macro_rules! code_occurence_tufa_common {
    ( $( $x:expr ),* ) => {{
        use crate::traits::error_logs_logic::code_occurence_new::CodeOccurenceNew;
        crate::common::code_occurence::CodeOccurenceLifetime::new(
            &crate::global_variables::compile_time::git_info::GIT_INFO,
            file!(),
            line!(),
            column!(),
        )
    }};
}

#[macro_export]
macro_rules! code_occurence {
    ( $( $x:expr ),* ) => {{
        use tufa_common::traits::error_logs_logic::code_occurence_new::CodeOccurenceNew;
        tufa_common::common::code_occurence::CodeOccurenceLifetime::new(
            &tufa_common::global_variables::compile_time::git_info::GIT_INFO,
            file!(),
            line!(),
            column!(),
        )
    }};
}
