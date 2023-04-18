#[macro_export]
macro_rules! code_occurence {
    ( $( $x:expr ),* ) => {{
        tufa_common::common::code_occurence::CodeOccurence::new(
            &crate::global_variables::compile_time::git_info::GIT_INFO,
            file!(),
            line!(),
            column!(),
            {
                use tufa_common::traits::fields::GetServerPort;
                *crate::global_variables::runtime::config::CONFIG.get_server_port()
            }
        )
    }};
}

#[macro_export]
macro_rules! code_occurence_tufa_common {
    ( $( $x:expr ),* ) => {{
        crate::common::code_occurence::CodeOccurence::new(
            &crate::global_variables::compile_time::git_info::GIT_INFO,
            file!(),
            line!(),
            column!(),
            {
                use crate::traits::fields::GetServerPort;
                *crate::global_variables::runtime::config::CONFIG.get_server_port()
            }
        )
    }};
}
