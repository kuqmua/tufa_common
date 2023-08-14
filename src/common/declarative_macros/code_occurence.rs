#[macro_export]
macro_rules! code_occurence {
    ( $( $x:expr ),* ) => {{
        tufa_common::common::code_occurence::CodeOccurence::new(
            $crate::global_variables::compile_time::git_info::GIT_INFO
                .to_git_info_without_lifetime(),
            file!().to_string(),
            line!(),
            column!(),
        )
    }};
}

#[macro_export]
macro_rules! code_occurence_tufa_common {
    ( $( $x:expr ),* ) => {{
        $crate::common::code_occurence::CodeOccurence::new(
            $crate::global_variables::compile_time::git_info::GIT_INFO
                .to_git_info_without_lifetime(),
            file!().to_string(),
            line!(),
            column!(),
        )
    }};
}
